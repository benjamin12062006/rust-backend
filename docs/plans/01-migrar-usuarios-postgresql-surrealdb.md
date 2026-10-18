# 01 — Migrar usuarios de PostgreSQL a SurrealDB

Fecha de creación: 2026-10-14  
Estado: Plan; todavía no implementado  
Ámbito: Backend, incremento 0 de la transición a SurrealDB  
Relacionada con: [producto y prototipo](../../docs-diagrams/01-producto-y-prototipo.md), [fases](../../docs-diagrams/03-fases-y-diagramas.md), [fuentes](../../docs-diagrams/04-fuentes-y-validacion.md), [dominio y datos](../../docs-diagrams/06-dominio-y-datos-surrealdb.md), [arquitectura y contratos](../../docs-diagrams/07-arquitectura-y-contratos.md) y [decisiones y construcción](../../docs-diagrams/09-decisiones-y-plan-de-construccion.md)  
Revisión de referencia: `260096f28a116e88b8dae300ea30566457d018cb` en `feature/first-steps`, con cambios locales ya existentes

## Objetivo

Sustituir la persistencia PostgreSQL/SQLx de los casos de uso actuales de
usuarios por un adaptador SurrealDB, conservando el contrato público basado en
UUID y las operaciones `create_user`, `list_users` y `get_user`.

El incremento estará completo cuando:

1. el esquema de usuarios esté definido y versionado para SurrealDB;
2. las tres operaciones del puerto `IUserRepository` funcionen contra una
   instancia real;
3. `UserService` reciba el adaptador SurrealDB desde `backend_bootstrap`;
4. los casos de uso y endpoints actuales sigan cumpliendo sus contratos;
5. el servidor arranque sin necesitar PostgreSQL ni Qdrant para el recorrido de
   usuarios;
6. PostgreSQL/SQLx se retiren únicamente después de comprobar que ya no quedan
   usos y de decidir qué hacer con cualquier dato existente.

Este plan no implementa todavía catálogos, importación CSV, reglas ni
exportación TSV. Esos trabajos comienzan en el incremento 1 después de cerrar
esta base.

## Lectura mínima para retomar el trabajo

Un chat futuro debe leer, en este orden:

1. [01 — Producto y prototipo](../../docs-diagrams/01-producto-y-prototipo.md),
   para entender qué producto se quiere construir y que los diagramas describen
   un objetivo, no código terminado.
2. [09 — Decisiones y plan de construcción](../../docs-diagrams/09-decisiones-y-plan-de-construccion.md),
   especialmente el diagnóstico de la transición y la tabla de incrementos.
3. Este plan completo.
4. El código real enlazado en la sección siguiente, porque el plan no sustituye
   el estado vigente del repositorio.

## Punto de partida comprobado

### Contratos que deben conservarse

- [`IUserRepository`](../../crates/backend_application/src/user_ports/out/user/user_repository.rs)
  ya define `create_user`, `list_users` y `get_user`.
- [`CreateUserRecord`](../../crates/backend_application/src/user_ports/out/user/records/input/create_user_record.rs)
  entrega a infraestructura los datos ya preparados por aplicación, incluido
  `password_hash`.
- [`StoredUserRecord`](../../crates/backend_application/src/user_ports/out/user/records/output/stored_user_record.rs)
  mantiene el UUID público y devuelve los campos persistidos a aplicación.
- [`UserService`](../../crates/backend_application/src/user_application/user_service.rs)
  depende del puerto, no de SQLx ni de SurrealDB. Esa dirección de dependencias
  debe permanecer.
- Los [tests de aplicación](../../tests/user_application_tests.rs) comprueban
  que la contraseña se transforma antes de persistirse y que los casos de uso
  delegan en el puerto.
- Los [tests HTTP](../../tests/user_http_tests.rs) fijan las rutas y las formas
  actuales de las respuestas sin depender de una base real.

### Acoplamiento que debe sustituirse

- [`UserDataStore`](../../crates/backend_infrastructure/src/user_infrastructure/repositories/user_data_store.rs)
  ejecuta SQL para PostgreSQL mediante SQLx.
- [`UserRepository`](../../crates/backend_infrastructure/src/user_infrastructure/repositories/user_repository.rs)
  envuelve ese data store y transforma `UserEntity` en `StoredUserRecord`.
- [`UserEntity`](../../crates/backend_infrastructure/src/user_infrastructure/repositories/user_entity.rs)
  deriva `sqlx::FromRow` y representa la fila de PostgreSQL.
- [`build_user_service`](../../crates/backend_bootstrap/src/bootstrap/services/user.rs)
  exige actualmente un pool PostgreSQL.
- [`build_application_services`](../../crates/backend_bootstrap/src/bootstrap/services/mod.rs)
  entrega PostgreSQL al módulo de usuarios y no entrega el cliente SurrealDB.
- [`DatabaseResources`](../../crates/backend_bootstrap/src/bootstrap/resources/databases/mod.rs)
  inicializa obligatoriamente PostgreSQL, SurrealDB y Qdrant, aunque el
  recorrido de usuarios solo debería necesitar SurrealDB.
- El [test de integración actual](../../tests/database_integration_tests.rs)
  solo comprueba PostgreSQL. Su mensaje de omisión dice
  `RUN_DB_TESTS=false to enable it`, pero la condición real exige `true`; esa
  inconsistencia debe corregirse al separar las pruebas por proveedor.

### Base SurrealDB que ya existe

- La [conexión remota](../../crates/backend_infrastructure/src/database/surrealdb/connection.rs)
  crea un cliente WebSocket, autentica un usuario root y selecciona namespace y
  database.
- El [health check](../../crates/backend_infrastructure/src/database/surrealdb/health.rs)
  ya consulta `client.health()`.
- La [carga de configuración](../../crates/backend_config/src/configuration/loader/databases.rs)
  ya exige URL, namespace, database, usuario y contraseña de SurrealDB.
- El workspace declara y bloquea `surrealdb = 3.3.4` en
  [`Cargo.toml`](../../Cargo.toml) y `Cargo.lock`.

### Estado operativo observado el 2026-10-14

Antes de alinear las versiones, la revisión local, sin mostrar secretos,
comprobó:

- el contexto de Docker activo es `orbstack`;
- OrbStack escucha en el puerto `8000` del Mac y lo publica al contenedor
  `backend-surrealdb`;
- `GET http://127.0.0.1:8000/health` responde `200`;
- una consulta autenticada desde el host usando las variables actuales del
  proyecto responde correctamente;
- el servidor en ejecución era SurrealDB `3.0.5` para `linux/arm64`;
- [`docker-compose.yaml`](../../docker-compose.yaml) usa la etiqueta móvil
  `surrealdb/surrealdb:latest`, sin política automática de descarga.

Studio autenticó correctamente, pero su sincronización de esquema falló en
`3.0.5` con `Couldn't write to a read only transaction`. La documentación
oficial establece que las closures que escriben adquieren contexto de escritura
desde SurrealDB `3.3`; antes de esa versión producen ese error. La base no tenía
tablas, usuarios de base, funciones, parámetros ni otros objetos de aplicación
según `INFO FOR DB`. Con esa evidencia y autorización del usuario se alinearon
el servidor y el SDK en la versión exacta `3.3.4`, sin borrar el volumen.

El volumen `surrealdb_data` conserva usuarios root y datos entre recreaciones.
SurrealDB informó que ya existía el usuario root y que no volvería a crearlo con
los argumentos `--user`/`--pass`. Por tanto, cambiar esas variables después de
inicializar el volumen no cambia automáticamente la contraseña persistida.
Eliminar el volumen no se utilizará como arreglo rutinario porque también
eliminaría sus datos.

## Fuentes y decisiones

### Evidencia

- El [plan general](../../docs-diagrams/09-decisiones-y-plan-de-construccion.md)
  establece que el incremento 0 debe implementar un adaptador SurrealDB de
  `IUserRepository`, conservar UUID y contratos públicos, añadir esquema
  versionado, inyectar el adaptador y retirar SQLx solo después de eliminar sus
  usos.[^F1]
- El [modelo de datos propuesto](../../docs-diagrams/06-dominio-y-datos-surrealdb.md)
  limita los tipos del proveedor a infraestructura y exige tablas con campos
  definidos e índices adecuados.[^F2]
- La documentación oficial confirma que el usuario root inicial queda
  persistido, que un volumen conserva la base y que `/health` permite comprobar
  el servidor.[^F3]
- La documentación oficial de tablas e índices respalda `SCHEMAFULL` e índices
  únicos; la sintaxis exacta se verificará contra la versión fijada antes de
  escribir el primer archivo SurrealQL.[^F4]

### Comparación de versiones

Hay dos opciones plausibles:

1. **Alinear servidor y SDK en `3.3.4`.** Resuelve la capacidad de escritura
   requerida por la sincronización del Studio actual y mantiene una sola
   versión verificable en Compose, Cargo y lockfile.
2. **Conservar servidor y SDK en `3.0.5`.** Mantiene la conexión básica, pero la
   sincronización de esquema de Studio reproduce el fallo de transacción de
   solo lectura.

Decisión: elegir la opción 1 y fijar `surrealdb/surrealdb:v3.3.4` junto con el
crate `surrealdb = 3.3.4`. La base de aplicación estaba vacía antes del cambio.
No se actualizará un solo lado de la conexión ni se volverá a utilizar
`latest`.

### Modelo de usuario propuesto

El esquema inicial será mínimo y no intentará anticipar organizaciones,
membresías o autenticación comercial:

| Elemento | Decisión propuesta |
| --- | --- |
| Tabla | `user`, `SCHEMAFULL`. |
| Identidad | Record ID construido desde el UUID público; la conversión queda dentro de infraestructura y ningún `RecordId` cruza el puerto. |
| Datos | `first_name`, `last_name`, `email`, `username`, `password_hash`, `created_at` y `updated_at`. |
| Unicidad | Índices únicos independientes para `email` y `username`. |
| Contraseña | Solo hash; nunca texto plano ni valor incluido en la respuesta HTTP. |
| Lectura | `get_user` devuelve el mismo `StoredUserRecord`; la ausencia se transforma en un error controlado, no en un valor inventado. |

La sintaxis SurrealQL y los tipos exactos de UUID y fecha se confirmarán con la
documentación de la versión fijada antes de implementar. Si usar el UUID dentro
del record ID no conserva el contrato de manera simple y verificable, se
mantendrá un campo UUID explícito con índice único; no se filtrarán tipos de
SurrealDB hacia aplicación para forzar una opción.

### Esquema versionado

Los scripts SurrealQL pertenecerán al adaptador de SurrealDB y tendrán orden
explícito. Antes de elegir la ubicación definitiva se comprobará que no
interfiera con `sqlx::migrate!`, que todavía consume `migrations/` durante la
transición.

La primera migración debe poder distinguirse y comprobarse; no basta con pegar
consultas manuales en Studio. La aplicación tendrá un mecanismo pequeño y
explícito para aplicar en orden las migraciones pendientes y registrar cuáles
terminaron. Una migración aplicada no se editará retroactivamente: cualquier
cambio de esquema posterior tendrá otro número.

No se usará el arranque del contenedor como única fuente del esquema. Compose
prepara infraestructura local; el backend es responsable de saber qué esquema
necesita.

### Datos PostgreSQL existentes

Antes de retirar PostgreSQL se contará cuántos usuarios reales existen:

- **Si no hay datos que conservar:** se registra esa comprobación y se completa
  el cambio de adaptador sin inventar una migración de datos.
- **Si hay datos:** se detiene la retirada de PostgreSQL y se prepara un traslado
  de una sola vez que conserve UUID y hashes, compare conteos e identidades y
  mantenga una copia recuperable hasta verificar SurrealDB.

No habrá escritura doble permanente. Mantener dos fuentes de verdad introduciría
conflictos que este incremento no necesita.

## Incrementos

### 0.1 — Fijar y verificar el entorno

1. Fijar una versión exacta y compatible del servidor en Compose.
2. Confirmar que SDK, servidor y formato de almacenamiento son compatibles.
3. Comprobar `/health`, autenticación root, selección de namespace/database y
   una consulta mínima desde el host.
4. Documentar cómo recrear únicamente el contenedor sin borrar el volumen.

**Resultado observable:** el entorno deja de depender de qué versión resuelva
`latest` en una fecha futura.

### 0.2 — Añadir el esquema versionado de usuario

1. Elegir la ubicación que no interfiera con las migraciones SQLx existentes.
2. Añadir la primera migración SurrealQL para tabla, campos e índices.
3. Incorporar un ejecutor ordenado de migraciones en infraestructura.
4. Comprobar primera aplicación, segunda ejecución sin cambios y rechazo de
   duplicados de `email` y `username`.

**Resultado observable:** una base vacía adquiere el esquema requerido de forma
repetible y deja evidencia de la versión aplicada.

### 0.3 — Implementar el adaptador SurrealDB

1. Crear una entidad técnica serializable/deserializable propia de
   infraestructura.
2. Implementar `IUserRepository` usando `SurrealDbClient`.
3. Mapear UUID/record ID y nombres de campos únicamente en el adaptador.
4. Transformar errores del SDK a `CustomError` sin incluir credenciales,
   consultas completas ni hashes en logs o respuestas.
5. Conservar el comportamiento actual de `create_user`, `list_users` y
   `get_user`.

**Resultado observable:** los tres métodos pasan pruebas de integración contra
SurrealDB real sin modificar los casos de uso.

### 0.4 — Inyectar SurrealDB en usuarios

1. Entregar `resources.databases().surrealdb()` a `ServiceResources`.
2. Construir `UserService` con el nuevo repositorio.
3. Mantener temporalmente PostgreSQL disponible solo mientras se comprueba la
   transición y cualquier dato existente.
4. Ejecutar los tests actuales de aplicación y HTTP para demostrar que la
   frontera externa no cambió.

**Resultado observable:** las rutas actuales de usuario usan SurrealDB mediante
el mismo puerto de aplicación.

### 0.5 — Separar dependencias de arranque

1. Hacer que el recorrido de usuario no exija PostgreSQL.
2. Hacer Qdrant opcional porque no participa en este incremento.
3. Evitar que una dependencia ajena impida arrancar el servidor.
4. Comprobar el arranque con SurrealDB como única base requerida.

**Resultado observable:** detener PostgreSQL y Qdrant no impide crear, listar u
obtener usuarios.

### 0.6 — Retirar PostgreSQL/SQLx

Este paso comienza solamente después de aprobar 0.1–0.5 y resolver los datos
existentes:

1. buscar todos los usos de `DbPool`, `sqlx`, `DATABASE_URL` y migraciones SQL;
2. retirar el repositorio, entidad y data store PostgreSQL sustituidos;
3. eliminar configuración, inicialización y dependencias que ya no tengan
   consumidores;
4. actualizar ejemplos de entorno y Compose sin borrar volúmenes o datos por
   accidente;
5. ejecutar nuevamente todas las comprobaciones.

**Resultado observable:** el workspace compila y el servidor funciona sin SQLx
ni configuración PostgreSQL.

## Matriz de aceptación

| Riesgo o contrato | Evidencia requerida |
| --- | --- |
| El esquema no es reproducible | Aplicarlo sobre base vacía y volverlo a ejecutar sin redefiniciones accidentales. |
| UUID cambia al migrar | Crear y obtener el mismo usuario comparando el UUID del puerto con el persistido. |
| Duplicados | Intentar repetir `email` y `username`; ambos deben fallar de forma controlada. |
| Contraseña expuesta | Verificar que se persiste un hash y que las respuestas HTTP no incluyen hash ni texto plano. |
| Mapeo incorrecto | Crear, listar y obtener un usuario con todos los campos representativos. |
| Regresión de aplicación | Mantener correctos los tests con `FakeUserRepository`. |
| Regresión HTTP | Mantener rutas, validación y cuerpos actuales de los tests Warp. |
| Dependencias innecesarias | Arrancar y recorrer usuarios con PostgreSQL y Qdrant detenidos. |
| Pérdida de datos | Contar y comparar usuarios antes de retirar PostgreSQL; conservar respaldo si había datos. |
| Diferencia de versiones | Registrar versiones efectivas del SDK y servidor usadas en las pruebas. |

## Comprobación prevista

Las órdenes exactas podrán ajustarse al nombre final de los tests, pero el cierre
debe incluir como mínimo:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
RUN_SURREALDB_TESTS=true cargo test --test database_integration_tests --locked
```

Además:

1. levantar únicamente SurrealDB con Compose;
2. comprobar `/health`;
3. ejecutar la migración sobre una base de prueba vacía dos veces;
4. probar `crear → listar → obtener` mediante el repositorio real;
5. iniciar el servidor sin PostgreSQL ni Qdrant;
6. recorrer `POST /v1/users`, `GET /v1/users` y
   `GET /v1/users/{uuid}`;
7. ejecutar `git diff --check`;
8. registrar en la futura entrada de bitácora solo las comprobaciones
   efectivamente ejecutadas y sus resultados.

No se declarará `Implementado y verificado` por el solo hecho de que compile.

## Límites y continuidad

Queda fuera de este incremento:

- autenticación de clientes, sesiones y permisos;
- organizaciones, membresías y aislamiento comercial;
- importación de catálogos y almacenamiento de archivos;
- actualización general de SurrealDB a una versión posterior;
- Qdrant y búsquedas vectoriales;
- rediseño de Warp o de los siete crates;
- cambios de contrato HTTP no necesarios para sustituir la persistencia.

Después de cerrar este plan, el siguiente escalón es el incremento 1:
**proyecto y snapshot CSV**. No se crearán por anticipación todos los
repositorios de catálogo descritos en los diagramas.

Al iniciar la implementación se abrirá una sola entrada nueva en
`bitacora-construccion/` con estado `En curso`, relacionada con este plan. Esa
entrada registrará el código realmente cambiado, las fuentes efectivamente
consultadas y las comprobaciones ejecutadas; este documento seguirá siendo un
plan y no afirmará que el trabajo ya existe.

## Fuentes

[^F1]: [09 — Decisiones y plan de construcción](../../docs-diagrams/09-decisiones-y-plan-de-construccion.md),
    secciones «Diagnóstico de la transición a SurrealDB» e «Incrementos con
    dependencias». Código de referencia: `HEAD 260096f`.
[^F2]: [06 — Dominio y modelo de datos](../../docs-diagrams/06-dominio-y-datos-surrealdb.md),
    secciones «Diccionario mínimo» e «Invariantes del dominio».
[^F3]: SurrealDB, [Docker](https://surrealdb.com/docs/running/docker),
    secciones «Configuring authentication», «Run your first query» y «Running
    SurrealDB with Docker Compose», consultada el 2026-10-14.
[^F4]: SurrealDB, [tablas y campos](https://surrealdb.com/docs/learn/schema-management/tables-and-fields/tables)
    e [índices](https://surrealdb.com/docs/reference/query-language/statements/define/indexes),
    consultadas el 2026-10-14; registradas también como E9 y E10 en
    [04 — Fuentes y validación](../../docs-diagrams/04-fuentes-y-validacion.md).
