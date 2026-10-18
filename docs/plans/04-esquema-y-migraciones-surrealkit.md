# 04 — Esquema y migraciones de SurrealDB con SurrealKit

Fecha de creación: 2026-10-17
Estado: Plan; esquema y migraciones reales todavía sin aplicar
Ámbito: Cuenta Firebase, cambios de esquema y retirada de PostgreSQL
Relacionada con: [arquitectura](02-simplificar-backend-y-preparar-auth.md), [auth](03-auth-firebase-surrealdb.md), [antecedente](01-migrar-usuarios-postgresql-surrealdb.md), [SurrealDB](../fonts/auth/03-surrealdb-cuentas-identidad-y-transacciones.md), [SurrealKit verificado](../fonts/auth/04-surrealkit-version-y-comandos-verificados.md)
Revisión de referencia: `14d8cc496b203a79872268cef0262c848816a7e7`

## Decisión

Usar **SurrealKit 1.0.0-beta.6** como herramienta externa, con **SurrealDB
3.3.4** como servidor/SDK objetivo. Mantener las versiones fijadas y revisar
una actualización como cambio propio, no usar `latest`.

El esquema deseado se escribe en `.surql`. SurrealKit genera y ejecuta los
rollouts. La API no escribe su propio motor, tabla de migraciones o sistema
`up/down`, ni aplica DDL automáticamente en cada réplica al arrancar.
Esta decisión reemplaza el ejecutor casero propuesto en el plan 01.

La herramienta está en prerelease. La [ficha de comprobación](../fonts/auth/04-surrealkit-version-y-comandos-verificados.md)
registra el checksum, los comandos disponibles y diferencias con la skill
local beta.3. No asumir `surrealkit check` ni que `baseline` sea solo lectura.
No agregar SurrealKit a las dependencias del ejecutable Rust para este uso.

## Organización propuesta al implementar

```text
surrealkit.toml
database/
├── schema/
│   └── 01-account.surql
├── rollouts/
│   ├── <id>__initial_firebase_account.toml
│   └── <id>__initial_firebase_account/   # SQL congelado por beta.6
├── snapshots/                         # Snapshots generados, versionados
├── setup.surql                        # Metadatos generados por la herramienta
├── seed/                              # Solo datos ficticios si hacen falta
└── tests/                             # Pruebas de esquema si aportan valor
migrations/                            # SQLx legado durante la transición
```

`database/` no se mezcla con `migrations/` de SQLx. Las migraciones SQL
anteriores son evidencia histórica y no se ejecutan contra SurrealDB. No
crear módulos SurrealKit separados para una sola tabla.

Al iniciar la implementación se podrá ejecutar `surrealkit init --minimal --yes`
una sola vez en la raíz correcta, después de revisar archivos existentes.
No usar `--force` sobre trabajo previo. Versionar la configuración y el setup
generados, sin credenciales ni usuarios reales en seeds.

## Borrador de esquema

Este bloque es una propuesta para el futuro `database/schema/01-account.surql`.
Se ensayó su reconocimiento mediante `rollout plan` y `lint` en una carpeta
temporal; falta ejecutarlo contra SurrealDB 3.3.4 y verificar sus invariantes.

```surql
DEFINE TABLE account SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD account_id ON account TYPE uuid READONLY;
DEFINE FIELD firebase_project_id ON account TYPE string READONLY
    ASSERT string::len($value) > 0;
DEFINE FIELD firebase_uid ON account TYPE string READONLY
    ASSERT string::len($value) > 0 AND string::len($value) <= 128;
DEFINE FIELD created_at ON account TYPE datetime DEFAULT time::now() READONLY;
DEFINE INDEX account_id_unique ON account FIELDS account_id UNIQUE;
DEFINE INDEX firebase_identity_unique ON account
    FIELDS firebase_project_id, firebase_uid UNIQUE;
```

Las dos claves únicas cumplen propósitos diferentes: UUID público y vínculo
Firebase. La creación siempre proporciona un UUID y la identidad verificada.
El adaptador convierte el UUID al tipo de base; no envía un string esperando
que se convierta implícitamente ni filtra el Record ID al cliente.

La cuenta es inmutable para este recorrido; por eso no hay `updated_at` que
nunca se utilice. Si más adelante se añaden perfiles, sus campos y política de
actualización requerirán un nuevo cambio de esquema.

`PERMISSIONS NONE` niega acceso de registros/huéspedes, **no** limita al usuario
de sistema de la API. La API controla identidad y consultas. No se añadirá
`DEFINE ACCESS` de contraseña ni se configurará Firebase JWT como sesión DB
directa para resolver este login.

## Configuración compartida y separación de credenciales

Propuesta de nombres únicos, alineados con SurrealKit:

| Variable | Ejemplo sin secretos | Sustituye en el loader Rust actual |
| --- | --- | --- |
| `SURREALDB_HOST` | `http://127.0.0.1:8000` local; HTTPS en remoto | `SURREALDB_URL` |
| `SURREALDB_NAMESPACE` | `handle_business` | Se conserva. |
| `SURREALDB_NAME` | `auth_dev` | `SURREALDB_DATABASE` |
| `SURREALDB_USER` | Nombre de cuenta de servicio | `SURREALDB_USERNAME` |
| `SURREALDB_PASSWORD` | Secreto local/gestor de secretos | Se conserva, nunca se imprime. |
| `SURREALDB_AUTH_LEVEL` | `database` para la cuenta de aplicación | El loader actual fuerza root; se hará explícito. |

Estos nombres requieren modificar el loader y el adaptador al implementar;
**no están activos todavía**. Revisar también las sustituciones de Compose
antes de renombrar variables, para no arrancar con una contraseña vacía.
HTTP(S) facilita usar los mismos endpoints con la CLI y el SDK remoto Rust;
el adaptador preservará el esquema y activará las features remotas necesarias.

El proceso API y el trabajo de esquema tienen credenciales de ejecución
separadas; sus nombres de variable pueden ser los mismos en entornos distintos.
Usar cuenta limitada a la base y rol suficiente para la API, y credencial de
esquema en el job de migración. Los roles de sistema son gruesos: no afirmar
que un rol `Editor` impide DDL ni que sustituye controles por cuenta.

No poner contraseñas en argumentos guardados en historial, `surrealkit.toml`,
manifests o logs. Evitar dependencias implícitas en el `.env` PostgreSQL legado:
SurrealKit 1.0 cambió aliases `DATABASE_*`; usar un entorno limpio con los
nombres SurrealDB documentados. Confirmar siempre host, namespace, base y
usuario previstos sin mostrar su contraseña.

## Bases nuevas: un primer rollout reproducible

Se elige esta ruta para el primer esquema, para que todos los entornos
compartan el mismo historial:

1. Inicializar el proyecto de esquema, con snapshots todavía vacíos.
2. Escribir `01-account.surql` y revisar que solo define objetos esperados.
3. Generar el rollout inicial localmente, sin conexión:

   ```bash
   surrealkit rollout plan --name initial_firebase_account
   surrealkit rollout lint
   ```

4. Revisar el manifest, SQL congelado y snapshots; versionar los tres juntos.
   El ensayo demostró que otro `plan` idéntico puede producir un manifest
   vacío: no ejecutar el generador por rutina ni versionar pasos vacíos.
5. En una base de prueba nueva, ejecutar `rollout start <id-real-generado>`,
   comprobar esquema/datos y completar según indique su estado. No inventar
   un ID copiado del ensayo temporal.
6. Usar los mismos archivos en los demás entornos. Si `rollout up` deja el
   último en `ready_to_complete`, verificar el corte antes de completarlo.

No hacer primero `sync` del esquema final y luego una `baseline` para ocultar
el primer rollout: esa sería una historia de adopción distinta. En una base
local desechable sí se podrá usar `sync --dry-run` y después `sync` para
iterar, sabiendo que los objetos administrados retirados de los archivos
pueden eliminarse. Ese entorno no demuestra el recorrido de rollouts.

## Base existente: adopción y cambios posteriores

No ejecutar `baseline` a ciegas. En beta.6 toma el catálogo desde los archivos,
escribe metadatos en la base y snapshots locales. No certifica que una base
arbitraria coincida con lo que acabamos de escribir.

Antes de adoptarla:

1. Respaldar y comprobar el destino; inventariar con `INFO FOR DB` y las
   definiciones pertinentes, usando credenciales apropiadas.
2. Conciliar el esquema real con archivos que describan **ese estado anterior**,
   no con el futuro esquema Firebase aún sin aplicar.
3. Revisar metadatos/historial de SurrealKit si ya existen. No sobrescribirlos
   ni usar `baseline` para saltarse rollouts pendientes.
4. Una vez conciliado el origen, ejecutar `rollout baseline`, revisar sus
   snapshots y guardarlos. Después editar el esquema objetivo y generar el
   nuevo rollout.

Para cambios posteriores, el manifest se genera contra snapshots, no contra
una introspección nueva del servidor. Por ello el despliegue debe confirmar
el estado del destino y detectar modificaciones manuales. Los archivos
congelados y hashes permiten aplicar la revisión que se evaluó, aunque
`database/schema/` ya tenga cambios posteriores.

Cambiar una entidad existente puede exigir `--allow-modified`. No añadir
ese flag automáticamente: revisar efectos sobre datos y reversibilidad.
Al fusionar dos ramas que generaron planes desde el mismo origen, `lint`
debe detectar la cadena incompatible; replanificar sobre el origen combinado.

## Despliegue, corte y recuperación

El pipeline futuro ejecuta las migraciones como una tarea explícita antes de
arrancar la versión de la API que las necesita. No múltiples réplicas
ejecutando DDL por su cuenta.

```bash
# Con el entorno de destino ya seleccionado y archivos revisados:
surrealkit rollout lint
surrealkit rollout up --dry-run
surrealkit rollout up
# Verificar expansión y desplegar/cambiar la aplicación.
surrealkit rollout status
# Completar solo tras verificar el estado y la compatibilidad:
surrealkit rollout complete <id-real-pendiente>
```

Estos comandos **no se ejecutaron contra bases reales** en esta preparación.
`dry-run` de despliegue puede necesitar conexión para conocer la posición del
destino; no confundirlo con `lint`, que se verificó sin conectar.

`up` puede completar fases contract de rollouts anteriores al ponerse al día.
Antes de saltar varias versiones se revisarán sus eliminaciones y la versión
de API que seguirá ejecutándose. Para el primer corte se prefieren comandos
por ID y expansión aditiva, sin eliminación de cuentas.

Si falla antes del corte, conservar la API anterior y analizar el estado.
`rollout rollback <id>` solo aplica dentro de las condiciones de la herramienta;
no es una restauración universal de datos. Después de `complete`, o de una
transformación destructiva, la recuperación puede exigir un nuevo rollout y
el respaldo. No borrar manifests/snapshots manualmente ni marcar `repair` como
éxito para saltarse un error. Un rollback de Git no revierte una base.

## Transición de datos PostgreSQL

Separar dos problemas: **esquema SurrealDB**, gestionado por SurrealKit, y
**traslado/vinculación de datos PostgreSQL**, que necesita un inventario propio.
Esta investigación no consultó las bases y no afirma que estén vacías.

- Si no hay usuarios que conservar, registrar la evidencia y crear cuentas
  nuevas desde Firebase; no implementar un migrador de hashes para desecharlo.
- Si hay usuarios reales, mantener PostgreSQL y un respaldo recuperable.
  Determinar un mapeo autorizado entre UUID antiguo e identidad Firebase,
  conservar referencias existentes y verificar conteos/identidades antes
  del corte. **No vincular por correo automáticamente** ni inventar un UID.
- No importar hashes a la nueva tabla `account`. Cualquier migración de
  credenciales hacia Firebase sería otra operación, con alcance y compatibilidad
  de algoritmo propios; no está incluida en este login inicial.
- Retirar SQLx, configuración y endpoints legados solo cuando el destino y
  la política de datos estén resueltos. No borrar volúmenes ni bases antiguas
  por haber eliminado un servicio de Compose.

## Verificación prevista

| Comprobación | Evidencia necesaria antes del corte |
| --- | --- |
| Motor objetivo | Versión real de servidor 3.3.4 y SDK fijado/lockfile. |
| Primer despliegue | Base de prueba vacía adquiere cuenta, campos e índices usando el rollout inicial. |
| Repetición | Reejecutar el despliegue completado no duplica registros ni altera UUID. |
| Esquema | UUID inválido, identidad vacía y duplicados son rechazados; `created_at` se genera. |
| Concurrencia | Dos sync simultáneos terminan con una cuenta y el mismo UUID. |
| Aislamiento | `me` nunca permite seleccionar otra identidad; permisos de base no se confunden con controles HTTP. |
| Historial | `rollout lint` verifica cadena y hashes; cambios congelados alterados fallan. |
| Recuperación | Ensayar fallo de expansión y recuperación en una base de prueba, sin prometer restauración de datos por Git. |
| Dependencias | API inicia y auth funciona sin PostgreSQL/Qdrant, tras resolver el inventario. |

No se dispone de `surreal` instalado ni de `surrealkit check` en el artefacto
verificado. La validación de servidor se hará sobre una instancia de prueba
3.3.4, además de `rollout lint`. No presentar el lint de manifests como una
prueba semántica del esquema o de permisos.

## Estado actual y siguiente paso

Solo hay planes, fichas y un ensayo temporal sin conexión. El siguiente
incremento creará la configuración/esquema reales, ejecutará el primer rollout
en una base de prueba y registrará resultados en una nueva entrada de bitácora.
No se generó aquí un historial de migraciones ficticio dentro del repositorio.
