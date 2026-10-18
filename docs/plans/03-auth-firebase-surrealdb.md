# 03 — Autenticación Firebase y cuenta interna en SurrealDB

Fecha de creación: 2026-10-17
Estado: Plan; sin endpoints de auth implementados
Ámbito: Identidad, cuenta propia y contrato HTTP mínimo
Relacionada con: [arquitectura](02-simplificar-backend-y-preparar-auth.md), [migraciones](04-esquema-y-migraciones-surrealkit.md), [Firebase y Rust](../fonts/auth/01-firebase-auth-y-verificacion-rust.md), [referencia Go](../fonts/auth/02-patron-auth-del-backend-go.md), [SurrealDB](../fonts/auth/03-surrealdb-cuentas-identidad-y-transacciones.md)
Revisión de referencia: checkpoint `14d8cc496b203a79872268cef0262c848816a7e7`

## Objetivo y alcance

Permitir que una persona inicie sesión mediante Firebase en el cliente,
sincronice su cuenta interna y consulte su propia identidad desde la API Rust.
Firebase gestiona el inicio de sesión; SurrealDB conserva la cuenta de la
aplicación. No se implementa un segundo login de contraseña en Rust.

Primer proveedor propuesto: **Google**, como en el proyecto Go y la skill de
Firebase. Antes del primer ensayo real se confirmará que sea el proveedor
habilitado por el usuario. El contrato usa Firebase ID tokens y permite
adaptar esa política sin cambiar la identidad persistida.

Fuera del alcance: estados `active/banned/suspended`, roles, organizaciones,
facturación, gestión Admin de usuarios, sesiones propias, cookies de sesión,
recuperación de contraseña propia y múltiples identidades externas por cuenta.
No se conectará el navegador directamente a SurrealDB.

## Flujo propuesto

```mermaid
sequenceDiagram
    actor U as Usuario
    participant UI as Cliente web
    participant F as Firebase Auth
    participant API as API Rust
    participant DB as SurrealDB
    U->>UI: Iniciar sesión
    UI->>F: Google mediante Firebase SDK
    F-->>UI: Sesión Firebase
    UI->>UI: user.getIdToken()
    UI->>API: POST /v1/auth/sync + Bearer ID token
    API->>API: Verificar firma, proyecto, tiempo y proveedor
    API->>DB: Obtener o crear cuenta para proyecto/UID
    DB-->>API: Cuenta única y estable
    API-->>UI: 201 si creó; 200 si ya existía
    UI->>API: GET /v1/auth/me + Bearer ID token
    API->>API: Verificar identidad
    API->>DB: Leer la cuenta de esa identidad
    API-->>UI: 200 cuenta propia
```

El cliente renueva tokens con el SDK de Firebase. Si expiran, obtiene uno nuevo
y puede repetir una petición una vez; no entra en un bucle de renovación ante
un token de otro proyecto o un proveedor rechazado. `signOut()` cierra la sesión
local. La API no emite ni almacena refresh tokens.

## Contratos mínimos dentro de `src/auth`

Firmas conceptuales que se convertirán a Rust al implementar:

| Contrato | Entrada | Salida |
| --- | --- | --- |
| `TokenVerifier::verify` | ID token | `VerifiedIdentity { project_id, uid }` o error de identidad. |
| `AccountRepository::sync` | Identidad ya verificada | `Account` y marca interna `created`; idempotente. |
| `AccountRepository::current` | Identidad ya verificada | Cuenta o `AccountNotSynced`; no crea registros. |
| `AuthService::sync/current` | Identidad del filtro HTTP | Resultado del caso de uso, sin tipos de proveedor. |

El servicio recibe identidad comprobada, no un UID arbitrario. No duplicar
estos contratos en interfaces inbound/outbound con los mismos métodos.
`VerifiedIdentity` no se deserializa desde el body y sus campos se construyen
solo en el verificador o en dobles de pruebas controlados.

Errores propios: `InvalidToken`, `IdentityProviderUnavailable`,
`AccountNotSynced`, `StorageUnavailable` e `Unexpected`. Los detalles del SDK,
certificados y consultas quedan en el adaptador; HTTP devuelve códigos estables.

## Verificación del token en Rust

No hay SDK Admin oficial Rust en la matriz de Firebase consultada. Se seguirá
su ruta documentada de biblioteca JWT externa, con el verificador aislado en
`auth/firebase.rs`. [Fuente y límites](../fonts/auth/01-firebase-auth-y-verificacion-rust.md).

Biblioteca candidata: `jsonwebtoken`, familia 11 en el código consultado,
con backend criptográfico elegido explícitamente; cliente HTTP con TLS,
timeouts y JSON. La versión exacta se resolverá contra el toolchain y se
guardará en Cargo.lock al implementar, sin actualizar todas las dependencias
del proyecto por este motivo. No agregar un proceso Node/Go para verificar
cada petición ni escribir criptografía propia.

El adaptador deberá:

1. Aceptar exactamente un encabezado Bearer con token no vacío. Propuesta de
   límite del proyecto: 16 KiB; rechazar encabezados ambiguos antes de verificar.
2. Leer del header únicamente lo necesario para elegir clave; esa lectura no
   autentica al usuario. Solo aceptar `RS256` y un `kid` utilizable.
3. Verificar la firma usando claves del endpoint fijo de Google documentado:
   `https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com`.
   Nunca descargar desde un `jku` o URL incluida por el cliente.
4. Exigir `aud == FIREBASE_PROJECT_ID` e issuer
   `https://securetoken.google.com/<FIREBASE_PROJECT_ID>`, además de `exp`,
   `iat`, `auth_time` y `sub` válidos. UID no vacío, de hasta 128 caracteres.
   Exigir tipos correctos; no aceptar claims ausentes como ceros por defecto.
5. Configurar una tolerancia temporal explícita y pequeña; propuesta: 30 s.
   Comprobar `iat` y `auth_time` expresamente y `nbf` si existe. No asumir que
   configurar `required_spec_claims` realiza todas esas comprobaciones.
6. Solo después de verificar, aplicar la política inicial de proveedor
   `firebase.sign_in_provider == "google.com"`. No usar `user_id`, correo ni
   un body para sustituir al `sub` validado. Identity Platform tenants quedan
   fuera: rechazar un token con tenant hasta diseñar esa identidad explícitamente.

### Caché y disponibilidad

Crear un cliente HTTP y una caché de claves compartidos por proceso. Respetar
el `max-age` del servidor; controlar expiración con reloj adecuado, no con un
TTL inventado permanente. Propuesta: timeout de descarga 5 s, límite de tamaño
de respuesta y actualización concurrente única.

Un `kid` desconocido puede provocar una renovación controlada, con enfriamiento
para evitar que tokens arbitrarios fuercen una descarga cada vez. Propuesta:
una renovación forzada como máximo por minuto, compartida entre peticiones.
Después de una descarga correcta, `kid` inexistente es `InvalidToken`.
Si no se puede obtener una clave necesaria, es indisponibilidad; no se acepta
el token sin verificar ni se usan indefinidamente claves vencidas. Una clave
todavía vigente en caché puede usarse sin consultar Google en esa petición.

La conversión de certificados X.509 al formato de clave que admita la versión
elegida debe comprobarse con una clave pública real y una fixture firmada.
Un fallo de descarga/configuración no debe convertirse en un engañoso `401`.

### Garantía que cambia respecto de Go

Go consulta `VerifyIDTokenAndCheckRevoked`. La propuesta inicial Rust comprueba
firma y claims **sin consultar revocación o usuarios deshabilitados en cada
petición**. Por eso un token ya emitido puede seguir aceptándose hasta expirar,
aunque se cierre la sesión o cambie la cuenta en Firebase. Esta limitación
se conserva explícita; no es un sistema de baneos oculto.

La primera implementación no habilitará un atajo de emulador en el verificador
de producción. Las pruebas usarán verificadores falsos o claves de prueba;
la aceptación final requiere un ID token firmado del proyecto real. Una futura
integración de emulador tendrá modo separado y rechazo de su configuración
fuera de desarrollo.

## Cuenta SurrealDB y concurrencia

Persistir una sola tabla `account`:

| Campo | Regla propuesta |
| --- | --- |
| `account_id` | UUID propio, generado una vez; único e inmutable. |
| `firebase_project_id` | Proyecto validado por el verificador, inmutable. |
| `firebase_uid` | `sub` validado, inmutable. |
| `created_at` | Fecha de creación del registro. |

Índice único compuesto para `(firebase_project_id, firebase_uid)`; el correo
no identifica ni fusiona cuentas. Ningún cliente puede escoger la cuenta
consultada. No se guarda contraseña, hash, token completo, estado o perfil
innecesario. El Record ID técnico no aparece en el contrato HTTP.

`sync` consulta por la identidad compuesta. Si no existe, intenta crear la
cuenta con un UUID nuevo. Dos peticiones pueden llegar juntas: el índice y la
atomicidad deben dejar **una** cuenta. Ante conflicto reconocido, releer la
cuenta ganadora y, si hace falta, repetir la operación con un límite pequeño
(propuesta: tres intentos y espera breve con variación). No devolver al cliente
el UUID candidato de una escritura fallida.

No traducir `ON CONFLICT` de PostgreSQL literalmente ni creer que `UPSERT`
por una condición elimina la carrera. Distinguir error único, conflicto
transaccional y caída de conexión. Revisar los errores por sentencia de la
respuesta del SDK antes de anunciar éxito. Si se agotan los intentos o no hay
cuenta legible, devolver indisponibilidad, no inventar una cuenta.
La comprobación de dos sincronizaciones concurrentes es requisito del corte.

## Contrato HTTP propuesto

Se conserva el prefijo Rust `/v1`, en lugar de copiar `/api/v1` de Go.

| Ruta | Operación | Resultado |
| --- | --- | --- |
| `POST /v1/auth/sync` | Sin body de identidad; Bearer requerido | `201` al crear, `200` si ya existía. |
| `GET /v1/auth/me` | Lee exclusivamente la identidad del token | `200`; `404 account_not_synced` si falta sincronizar. |

Cuerpo de cuenta: `{"account_id":"<uuid>","created_at":"<RFC3339>"}`.
La marca `created` sirve para seleccionar el status de sync, no se persiste
ni se añade un campo `status` por imitación del backend Go.

| Condición | Código y cuerpo mínimo |
| --- | --- |
| Falta Bearer, firma/claims/proveedor inválidos | `401 {"error":"invalid_token"}` |
| No hay claves utilizables por fallo del proveedor | `503 {"error":"auth_unavailable"}` |
| Cuenta inexistente en `me` | `404 {"error":"account_not_synced"}` |
| No se puede persistir o leer | `503 {"error":"storage_unavailable"}` |
| Fallo interno no clasificado | `500 {"error":"internal_error"}` |

Responder JSON y `Cache-Control: no-store`. No registrar Authorization, JWT,
claims completos ni consultas con datos. Registrar un identificador de petición
y categoría de error sin divulgarla al cliente. Incluir `WWW-Authenticate`
cuando corresponda a un `401` Bearer.

CORS admite los orígenes concretos del frontend, `Authorization`, GET, POST
y OPTIONS. El preflight no exige token. CORS no reemplaza la verificación
del token; clientes ajenos al navegador también deben autenticarse.

Las rutas públicas `/v1/users` actuales no son un login Firebase. Se retirarán
en el corte indicado por el plan 02; no se publicará un listado libre de cuentas
ni un `get_user/{uuid}` que permita consultar a otra persona.

## Configuración que hará falta

La tabla describe configuración **futura**; el loader actual aún exige variables
de los proveedores anteriores. No colocar credenciales reales en estos docs.

| Ámbito | Variable/campo | Uso |
| --- | --- | --- |
| API | `FIREBASE_PROJECT_ID` | Proyecto permitido; debe coincidir con el frontend. |
| API | `PORT`, `APP_ENV`, `CORS_ALLOWED_ORIGINS` | Puerto, entorno y orígenes explícitos. |
| API/SurrealKit | `SURREALDB_HOST`, `SURREALDB_NAMESPACE`, `SURREALDB_NAME`, `SURREALDB_USER`, `SURREALDB_PASSWORD`, `SURREALDB_AUTH_LEVEL` | Conexión; detalle de transición en plan 04. |
| Cliente web | `apiKey`, `authDomain`, `projectId`, `appId`, URL de la API | Configuración pública de la app web Firebase. No son una credencial Admin. |

No pedir `FIREBASE_CREDENTIALS_PATH`, `GOOGLE_APPLICATION_CREDENTIALS` ni una
clave privada para esta verificación pública. El loader Firebase actual sí
exige un archivo de credenciales: se simplificará al implementar. Retirar los
múltiples proveedores de configuración que no se usarán, en vez de agregar
una excepción sobre cada uno.

El usuario prepara el proyecto Firebase y el proveedor. La investigación no
ejecuta `firebase deploy`, no modifica consolas ni crea proyectos.

## Incrementos y aceptación

1. **Verificador aislado:** pruebas de firma válida, algoritmo distinto, `kid`
   desconocido, caducidad, proyecto ajeno y claims ausentes/futuros; caché y
   caída de descarga. Sin dependencia de una cuenta de servicio real.
2. **Esquema y repositorio:** aplicar en base de prueba según plan 04; sincronizar
   dos veces y en paralelo. Un UUID por identidad; otra identidad, otra cuenta.
3. **Servicio y HTTP:** dobles controlados para `201/200`, `me` sin efectos,
   errores `401/404/503`, headers ambiguos y ausencia de secretos.
4. **Composición:** solo Firebase configurado y SurrealDB requeridos; clientes
   compartidos, configuración incompleta falla antes de servir.
5. **Ensayo real:** login en el cliente, `getIdToken`, sync y me con el mismo
   `account_id`; rechazo de un token de otro proyecto. Después cerrar rutas
   antiguas y comprobar que PostgreSQL/Qdrant detenidos no bloquean auth.

Reutilizar las pruebas existentes que aporten cobertura al traslado mecánico;
añadir las verificaciones críticas de identidad/concurrencia como parte expresa
de este incremento. No copiar toda la suite de Go ni presentar la decodificación
de un JWT como prueba de autenticación.

Estado al redactar: no se ejecutaron login, endpoints ni pruebas Firebase.
Los criterios anteriores son trabajo pendiente, no resultados obtenidos.
