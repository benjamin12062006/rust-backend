# 02 — Patrón de autenticación del backend Go propio

Fecha de creación: 2026-10-17
Estado: Investigación
Tipo de fuente: Código propio y documentación histórica local
Origen: `/Users/benji/Documents/Analisis-Inversion/navigate-business-api`
Fecha de consulta: 2026-10-17, America/New_York
Relacionada con: [arquitectura simplificada](../../plans/02-simplificar-backend-y-preparar-auth.md), [auth](../../plans/03-auth-firebase-surrealdb.md)

## Localización y evidencia

Se leyó el código directamente; no se atribuye a un curso su implementación
final ni se considera que la documentación histórica sustituya al código.
El proyecto de referencia quedó sin modificaciones. No se le asigna un commit
Git que no se haya verificado.

| ID | Archivo y líneas consultadas | Mecanismo observado |
| --- | --- | --- |
| G1 | [verifier.go](/Users/benji/Documents/Analisis-Inversion/navigate-business-api/internal/auth/verifier.go), 8–15 | Puerto neutral `TokenVerifier`; errores de token y proveedor separados. |
| G2 | [firebase/verifier.go](/Users/benji/Documents/Analisis-Inversion/navigate-business-api/internal/auth/firebase/verifier.go), 17–44 | SDK Admin oficial Go, revocación y restricción a `google.com`. |
| G3 | [http/middleware.go](/Users/benji/Documents/Analisis-Inversion/navigate-business-api/internal/auth/http/middleware.go), 22–50 | Exige un Bearer token, verifica antes del handler y entrega el UID mediante contexto. |
| G4 | [service.go](/Users/benji/Documents/Analisis-Inversion/navigate-business-api/internal/auth/service.go), 5–37 | Repositorio `Sync/Get`; servicio separado del transporte. Incluye suspensión, que aquí se excluye. |
| G5 | [http/handler.go](/Users/benji/Documents/Analisis-Inversion/navigate-business-api/internal/auth/http/handler.go), 19–62 | Sincronización `201/200`, consulta actual y errores públicos con `Cache-Control: no-store`. |
| G6 | [postgres/account_repository.go](/Users/benji/Documents/Analisis-Inversion/navigate-business-api/internal/auth/postgres/account_repository.go), 21–79 | Identidad única, transacción y recuperación de una carrera sin dejar una cuenta huérfana. |
| G7 | [module/module.go](/Users/benji/Documents/Analisis-Inversion/navigate-business-api/internal/auth/module/module.go), 14–20; [app.go](/Users/benji/Documents/Analisis-Inversion/navigate-business-api/internal/app/app.go), 25–65 | Composición explícita de las dependencias. |
| G8 | [router.go](/Users/benji/Documents/Analisis-Inversion/navigate-business-api/internal/httpapi/router.go), 26–29 | `POST /api/v1/auth/sync` y `GET /api/v1/auth/me`. |

Anclas SHA-256 de la copia consultada:

```text
internal/auth/firebase/verifier.go
c229d4ee8a292610aad9546da6fb784dbe1e652073513fc64215789548431661
internal/auth/postgres/account_repository.go
e08366cdce57da70213d65b23250eb8cf5870fe5511a52959bd4bb93b09ecdd2
internal/auth/service.go
64dc8b54444ffa9c6ea7f29df5c4cbc9e432442a0f1b1eccbe33aad5be23bea5
```

También se consultaron las bitácoras `auth/03-modulo-auth-y-migraciones-estandar.md`
y `auth/04-contrato-neutral-verificacion-identidad.md`. Documentan la sustitución
de un ejecutor casero por una herramienta de migraciones y la neutralidad de
los errores; no prueban que Firebase/Cloud SQL estuvieran integrados en Rust.

## Qué trasladamos y qué adaptamos

| Mecanismo Go | Adaptación propuesta en Rust |
| --- | --- |
| Identidad externa verificada antes de persistir | Mismo orden; nunca confiar en un UID enviado en el body. |
| Módulo de funcionalidad con sus adaptadores cerca | `src/auth/`, dentro de un único crate. |
| `Sync` idempotente y `Current` de solo lectura | Conservar semántica; usar `/v1/auth/*` para respetar el prefijo Rust. |
| Cuenta e identidades en dos tablas | Una tabla `account` con identidad Firebase compuesta para el alcance actual. |
| `status`, `ErrSuspended` | Omitir ambos por petición expresa. |
| SDK Admin y comprobación de revocación | Verificación JWT en Rust; declarar el cambio de garantía. |
| Transacción y `ON CONFLICT` PostgreSQL | Unicidad y recuperación de carreras según SurrealDB; no traducir SQL literalmente. |
| Motor externo de migraciones | SurrealKit; no importar `golang-migrate` ni construir otro ejecutor. |

La tabla única es una propuesta propia para una sola identidad Firebase por
cuenta. No anticipa un sistema de múltiples proveedores independientes.
Si se necesita uno, se migrará de forma explícita más adelante.
