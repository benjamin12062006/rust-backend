# 03 — SurrealDB: cuentas, identidad y transacciones

Fecha de creación: 2026-10-17
Estado: Investigación
Tipo de fuente: Documentación oficial, skills locales y código propio
Fecha de consulta: 2026-10-17, America/New_York
Relacionada con: [auth](../../plans/03-auth-firebase-surrealdb.md), [migraciones](../../plans/04-esquema-y-migraciones-surrealkit.md)

## Versiones y fuentes

Servidor y SDK objetivo: **SurrealDB 3.3.4**. Coinciden con Compose y Cargo.lock
del checkpoint `14d8cc4`; `https://download.surrealdb.com` devolvió `v3.3.4`.
Esto no equivale a una comprobación de la versión del contenedor en ejecución.

| ID | Fuente y sección | Uso |
| --- | --- | --- |
| S1 | [DEFINE TABLE](https://surrealdb.com/docs/reference/query-language/statements/define/table), «Defining permissions» | `SCHEMAFULL`, permisos y distinción de usuarios. |
| S2 | [DEFINE FIELD](https://surrealdb.com/docs/reference/query-language/statements/define/field), tipos, `READONLY`, `DEFAULT`, `ASSERT` | Esquema de cuenta mínimo. |
| S3 | [DEFINE INDEX](https://surrealdb.com/docs/reference/query-language/statements/define/indexes), índice `UNIQUE` | Identidad externa y UUID únicos. |
| S4 | [Transactions](https://surrealdb.com/docs/learn/querying/concepts-and-guides/transactions), «Write conflicts and retries» y «Unique indexes under concurrent writes» | Carrera de sincronización y repetición de la operación completa. |
| S5 | [JWT access](https://surrealdb.com/docs/reference/query-language/statements/define/access/jwt), «Using tokens» | Alternativa de autenticación directa en la base y sus claims adicionales. |
| S6 | [Rust quickstart](https://surrealdb.com/docs/languages/rust) | SDK remoto, consultas y bindings. |

Skills consultadas: [surrealdb-docs](../../../.agents/skills/surrealdb-docs/SKILL.md),
[surrealql](../../../.agents/skills/surrealql/SKILL.md) y
[surrealkit](../../../.agents/skills/surrealkit/SKILL.md).
El índice oficial consultado fue [llms.txt](https://surrealdb.com/llms.txt).

## Evidencia y límites

S1 establece que `PERMISSIONS` filtra a usuarios de registro y huéspedes,
pero no a usuarios de sistema. Por tanto, `PERMISSIONS NONE` no restringe la
conexión de servicio del backend; la aplicación debe limitar las operaciones
a la identidad autenticada.

S4 explica que dos escrituras concurrentes pueden competir al confirmar la
transacción. Un índice único impide duplicados; una consulta previa de existencia
no lo garantiza. Hay que contemplar conflicto transaccional y duplicado al
releer o reintentar, sin equiparar cualquier fallo de almacenamiento a una carrera.

S5 demuestra que aceptar JWT directamente en SurrealDB es otra arquitectura.
No basta con pasarle un token Firebase ordinario y asumir una sesión con los
permisos de la aplicación. Se conserva la API Rust como frontera de identidad.

## Propuesta propia

Una tabla `account` contiene `account_id` UUID, `firebase_project_id`,
`firebase_uid` y `created_at`. La identidad única es el par proyecto/UID;
el correo no se usa para unir cuentas. El Record ID técnico permanece en el
adaptador y el HTTP expone el UUID propio.

No hay `password`, hashes, refresh tokens, estado, roles ni organización en esta
primera cuenta. Los tokens solo viajan en la petición y no se persisten.
El [plan de auth](../../plans/03-auth-firebase-surrealdb.md) fija la concurrencia,
los errores y el acceso a la cuenta actual.

El cliente actual elimina el esquema de la URL y conecta siempre como `Ws`
con usuario root. Al simplificarlo se debe preservar TLS según la URL, usar
un cliente compartido y separar las credenciales de ejecución de las de esquema.
El plan propone HTTP(S) remoto para compartir configuración con SurrealKit;
no incorpora esa modificación todavía.

No se consultaron datos de PostgreSQL o SurrealDB ni se aplicaron esquemas
en esta investigación. La existencia de usuarios antiguos sigue por comprobar.
