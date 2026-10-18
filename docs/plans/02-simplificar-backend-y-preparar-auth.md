# 02 — Simplificar el backend y preparar autenticación

Fecha de creación: 2026-10-17
Estado: Plan; refactorización y auth todavía sin implementar
Ámbito: Backend Rust; entrada principal para retomar el trabajo
Relacionada con: [auth](03-auth-firebase-surrealdb.md), [migraciones](04-esquema-y-migraciones-surrealkit.md), [antecedente PostgreSQL](01-migrar-usuarios-postgresql-surrealdb.md), [referencia Go](../fonts/auth/02-patron-auth-del-backend-go.md)
Revisión de partida: `14d8cc496b203a79872268cef0262c848816a7e7`
Rama de trabajo: `feature/auth`

## Objetivo y entrega actual

Reducir el coste de mantenimiento de los siete crates sin perder separación
entre reglas, transporte y proveedores. Implementar después el recorrido de
inicio de sesión Firebase y cuenta interna SurrealDB.

Esta entrega prepara Git, instala las skills y documenta la implementación.
No mueve archivos Rust, no sustituye dependencias del runtime, no configura
Firebase y no modifica datos ni volúmenes. La arquitectura siguiente es el
destino acordado de planificación, no la estructura ya implementada.

## Respaldo recuperable

El estado recibido se guardó completo en `feature/first-steps`, commit
`14d8cc4`, y se publicó en
[GitHub](https://github.com/benjamin12062006/rust-backend/tree/feature/first-steps).
Incluye cambios staged, unstaged y los dos archivos sin rastrear revisados.
No se creó una release ni se movieron tags anteriores.

Se volvió a `main` (`260096f`), se creó `feature/auth` y se incorporó el
checkpoint mediante fast-forward. `main` no se movió; la nueva rama contiene
los últimos archivos y documentación. No hace falta volver a copiar las skills
desde la rama de respaldo.

Para consultar el estado anterior sin tocar el trabajo actual:

```bash
git show 14d8cc4:Cargo.toml
git diff 14d8cc4..feature/auth -- src crates Cargo.toml
```

Un checkout recupera archivos rastreados, no bases de datos. No ejecutar
`reset --hard` ni restauraciones de volúmenes para consultar ese respaldo.

## Diagnóstico del código existente

La separación actual permite reutilizar controladores, casos de uso, validación,
pruebas y la conexión SurrealDB. El problema operativo es la dispersión de una
misma funcionalidad entre siete crates, varias jerarquías de DTO/records y un
bootstrap repartido en recursos y servicios.

Además hay acoplamientos concretos:

- `backend_common::CustomError` implementa `warp::reject::Reject`; aplicación
  depende de ese crate y arrastra transporte indirectamente.
- El middleware devuelve `custom_error.to_string()` al cliente, incluido
  contenido de errores técnicos.
- `DatabaseResources::initialize` exige tres bases para arrancar; los usuarios
  solo usan PostgreSQL. La configuración de auth es declarativa, todavía no un
  verificador Firebase conectado a las rutas.
- `surrealdb/connection.rs` quita el esquema de la URL y fuerza `Ws`; no debe
  reutilizarse ese tratamiento para una URL HTTPS/WSS de producción.

Estos hechos se verificaron en el código del checkpoint. La dirección elegida
se apoya en el [módulo Go consultado](../fonts/auth/02-patron-auth-del-backend-go.md),
adaptado a este lenguaje y alcance.

## Arquitectura propuesta: un crate, módulos por funcionalidad

```text
Cargo.toml                  # Un paquete backend; no siete paquetes internos
src/
├── main.rs                 # Entrada y terminación del proceso
├── lib.rs                  # Exportaciones necesarias para las pruebas
├── app.rs                  # Crear clientes compartidos y componer módulos
├── config.rs               # Configuración validada del proceso
├── http.rs                 # Router, CORS y errores públicos generales
├── database.rs             # Conectar y comprobar SurrealDB remoto
└── auth/
    ├── mod.rs              # Modelo Account, identidad y exportaciones mínimas
    ├── service.rs          # Sync/current y dos contratos necesarios
    ├── http.rs             # Filtro Bearer, DTO HTTP pequeño y handlers
    ├── firebase.rs         # Verificación del ID token y caché de claves
    └── surreal.rs          # Persistencia de cuentas, bindings y errores DB
database/                   # Esquema y rollouts de SurrealKit, fuera del binario
tests/                      # Pruebas adaptadas y verificaciones críticas
```

Es un mapa de destino, no una orden de crear archivos vacíos. Los módulos de
catálogo se añaden cuando se construya su recorrido; no se crean ahora
`product`, `catalog`, colas ni puertos futuros.

El servicio conoce `AccountRepository` y `TokenVerifier`, no Warp, clientes
Firebase ni `RecordId`. Los adaptadores implementan esos contratos. HTTP puede
usar un servicio concreto parametrizado por los contratos; no se necesita una
interfaz de caso de uso que repita todos sus métodos. Los contratos caben en
`service.rs`, sin árboles `ports/in/out/records/input/output`.

Conservar Warp por ahora evita simultanear la simplificación con una migración
de framework. Loco/Axum no forman parte de este incremento. Se reutilizan
validaciones y tests que sigan siendo aplicables; el hash de contraseñas no
pertenece al nuevo flujo Firebase.

## Mapa de reutilización

| Origen actual | Destino o tratamiento propuesto |
| --- | --- |
| `backend_bootstrap` y `src/main.rs` | `app.rs` y entrada pequeña; composición explícita. |
| `backend_config` | `config.rs`; solo Firebase, SurrealDB, puerto y orígenes requeridos. |
| `backend_http` | `http.rs` general y `auth/http.rs`; mantener Warp. |
| `backend_domain` + `backend_application` | Modelo y servicio dentro de la funcionalidad; conservar invariantes útiles. |
| `backend_infrastructure` | `database.rs`, `auth/firebase.rs`, `auth/surreal.rs`. |
| `backend_common` | Errores propios por funcionalidad y traducción HTTP en el borde; sin crate común. |
| SQLx, PostgreSQL y migraciones SQL | Retirar del runtime después de inventariar y resolver los datos; conservar respaldo. |
| Qdrant | Fuera del arranque de auth; retirar dependencia sin consumidor, sin borrar datos. |
| `pwhash`, secreto local y configuración Clerk | Retirar cuando el flujo legado deje de tener consumidores. |

La base se operará con SDK remoto. Investigar al implementar la eliminación
de `kv-mem` del binario API, que hoy activa un motor local y sus dependencias
pese a que la conexión es remota. Los tests de integración usarán un servidor
de pruebas separado; no justificarán cargar un motor embebido en producción.

## Orden de implementación posterior

1. **Establecer una compilación de referencia.** Resolver dependencias y ejecutar
   los tests existentes antes de atribuir fallos al refactor.
2. **Consolidar el código mecánicamente en un crate.** Conservar temporalmente
   el recorrido legado para comparar comportamiento. Adaptar imports y pruebas;
   no borrar el origen hasta que el destino compile. No desplegar ese puente
   como la nueva API autenticada.
3. **Preparar esquema y migraciones** siguiendo el [plan 04](04-esquema-y-migraciones-surrealkit.md),
   sobre base de prueba. Resolver el inventario PostgreSQL antes de retirarlo.
4. **Añadir verificador y cuenta Firebase** según el [plan 03](03-auth-firebase-surrealdb.md),
   con dependencias compartidas y `/v1/auth/sync` + `/v1/auth/me`.
5. **Cerrar el corte.** Retirar endpoints públicos de usuarios y credenciales
   locales cuando el nuevo recorrido esté verificado y los datos resueltos.
   La ausencia de `/v1/users` será una ruptura explícita, no una supuesta
   conservación del contrato del plan anterior.
6. **Retirar piezas sin uso.** SQLx, Qdrant del recorrido de auth, crates viejos
   y configuración que ya no consume nadie. Conservar los volúmenes.

Cada paso tendrá un commit recuperable. Las pruebas de comportamiento se
conservan en el paso mecánico; las de contraseñas pasan a ser históricas cuando
el nuevo contrato sustituya al anterior.

## Criterios de aceptación y riesgos

- Un paquete Rust y módulos que se puedan recorrer sin saltar siete crates.
- Auth arranca con Firebase configurado y SurrealDB, sin necesitar PostgreSQL
  ni Qdrant ni un secreto JWT propio.
- `cargo fmt --all -- --check`, `cargo check --all-targets --locked` y
  `cargo test --locked` pasan en la implementación final.
- No aparecen contraseñas, tokens ni errores internos en respuestas o logs.
- El usuario propio solo se obtiene desde identidad validada.

El monolito de un crate pierde la barrera de compilación entre capas; se
compensa con visibilidad privada y revisión de imports. No se compensa creando
otra infraestructura de DI o nuevas capas ceremoniales.

## Comprobación de esta preparación

`cargo check --workspace --all-targets --locked --offline` falló porque
`diskann`, dependencia del motor local de SurrealDB 3.3.4, no estaba en el
índice/caché local. No demuestra un error de nuestros módulos, pero tampoco
permite afirmar que el checkpoint compila. No se cambiaron versiones para
ocultarlo. Al implementar se repetirá con acceso al registro y se revisarán
las features del cliente remoto.

El checkpoint tenía avisos de espacios finales en Markdown al ejecutar
`git diff --cached --check`. Se conservaron como parte del respaldo exacto;
los documentos nuevos se comprueban por separado.

No se hizo aún refactor ni se prometen tests funcionales de Firebase. Para
comenzar, leer este plan, el 03 y el 04; después contrastar sus rutas con el
código real y abrir la siguiente entrada de implementación.
