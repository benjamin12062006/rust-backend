# 01 — Firebase Auth y verificación de identidad en Rust

Fecha de creación: 2026-10-17
Estado: Investigación
Tipo de fuente: Skills oficiales, documentación oficial y código de biblioteca
Fecha de consulta: 2026-10-17, America/New_York
Relacionada con: [plan de auth](../../plans/03-auth-firebase-surrealdb.md), [referencia Go](02-patron-auth-del-backend-go.md)

## Necesidad

Autenticar el backend Rust con Firebase sin reproducir un sistema de contraseñas,
sin estados de cuenta y sin asumir que existe el mismo SDK Admin que usamos en Go.

## Fuentes verificadas

| ID | Fuente y localizador | Qué respalda |
| --- | --- | --- |
| F1 | [Firebase: Verify ID Tokens](https://firebase.google.com/docs/auth/admin/verify-id-tokens), secciones «Retrieve ID tokens on clients» y «Verify ID tokens using a third-party JWT library» | Token que debe enviar el cliente y verificaciones obligatorias. |
| F2 | [Firebase: Admin setup](https://firebase.google.com/docs/admin/setup), matriz de lenguajes | Rust no figura entre los SDK Admin oficiales publicados. |
| F3 | [Firebase: Manage User Sessions](https://firebase.google.com/docs/auth/admin/manage-sessions), introducción y detección de revocación | Duración del ID token y diferencia entre firma válida y revocación. |
| F4 | [Firebase: Google Sign-In web](https://firebase.google.com/docs/auth/web/google-signin), flujo de popup | Inicio de sesión en el cliente. |
| F5 | [Firebase: Authentication emulator](https://firebase.google.com/docs/emulator-suite/connect_auth), diferencias con producción | Los tokens del emulador no representan la verificación criptográfica de producción. |
| F6 | [jsonwebtoken](https://github.com/Keats/jsonwebtoken), README «Decoding» y «Validation»; [Validation](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/struct.Validation.html) | Biblioteca JWT candidata; las opciones predeterminadas no equivalen al contrato Firebase. |
| F7 | [Skill instalada](../../../.agents/skills/firebase-auth-basics/SKILL.md), «Tokens» y «Workflow»; [referencia web](../../../.agents/skills/firebase-auth-basics/references/client_sdk_web.md) | Orientación de integración y proveedores. |

Las 13 skills de `firebase/agent-skills` se instalaron en el proyecto mediante
`npx --yes skills add firebase/agent-skills --skill '*' --agent codex --copy --yes`.
[skills-lock.json](../../../skills-lock.json) conserva origen y hashes por skill.
El HEAD remoto consultado fue `306cc1844c47839bd4f2c7dba7cc3c394e4f3e0b`;
el lock registra hashes de contenido, no un pin de commit para futuras actualizaciones.
No se instalaron globalmente ni se configuró o desplegó un proyecto Firebase.

## Evidencia e interpretación

F1 admite una biblioteca JWT externa cuando el lenguaje no tiene SDK Admin.
Exige firma RS256, `kid`, proyecto correcto en `aud` e `iss`, expiración,
`iat`, `auth_time` y sujeto válido. Las claves públicas se obtienen del endpoint
publicado por Google y se renuevan según su `Cache-Control`.

F3 distingue la comprobación criptográfica de la consulta de revocación. El ID
token normalmente dura una hora; cerrar sesión en el navegador no invalida de
inmediato un token ya emitido.

F6 ofrece decodificación con verificación de firma. Su `required_spec_claims`
solo contempla determinados claims estándar; añadir `iat` o `auth_time` a esa
lista no implementa su comprobación. El adaptador deberá exigirlos mediante
claims tipados y comprobarlos expresamente después de verificar la firma.
No se instalaron dependencias Rust en esta investigación.

## Adaptación propuesta

Un `TokenVerifier` devuelve una identidad comprobada y errores propios del
módulo. Un adaptador Rust combina una biblioteca JWT y un cliente HTTP para
las claves públicas. No se escriben algoritmos criptográficos propios ni se
presenta una biblioteca comunitaria como SDK oficial.

Usar Google como primer proveedor, siguiendo Go, es una **propuesta del
proyecto**, pendiente de concordar con el proveedor que habilite el usuario.
El backend recibe el Firebase ID token de `user.getIdToken()`, no el Google
OAuth access token mostrado en algunos ejemplos de la skill web.

Para esta primera entrega se propone verificar tokens sin consultar revocación
en cada petición. No copiaremos `VerifyIDTokenAndCheckRevoked` de Go como si
fuera una función Rust. La limitación y sus pruebas se fijan en el plan de auth.
No se necesita una clave privada de cuenta de servicio para verificar una
firma con las claves públicas de Google. Una futura operación Admin sí puede
requerir credenciales y otro alcance.

Las instrucciones de provisioning de F7 no se ejecutan: el usuario administra
Firebase. Las reglas de Firestore/Storage tampoco protegen SurrealDB; no se
incorporarán esos productos por haber instalado sus skills.
