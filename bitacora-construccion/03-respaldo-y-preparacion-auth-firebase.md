# 03 — Respaldo y preparación de auth Firebase

Fecha de creación: 2026-10-17
Estado: Investigación; respaldo y skills completados, auth y refactor sin implementar
Ámbito: Git, skills locales, fuentes y planes del backend
Relacionada con: [entrada anterior](02-integrar-version-reciente-en-main.md), [plan principal](../docs/plans/02-simplificar-backend-y-preparar-auth.md), [auth](../docs/plans/03-auth-firebase-surrealdb.md), [migraciones](../docs/plans/04-esquema-y-migraciones-surrealkit.md)
Revisión recibida: `260096f28a116e88b8dae300ea30566457d018cb`, rama `feature/first-steps`, con cambios staged, unstaged y sin rastrear
Checkpoint creado: `14d8cc496b203a79872268cef0262c848816a7e7`

## Necesidad y trabajo realizado

El usuario pidió conservar la arquitectura actual en GitHub antes de preparar
un auth más sencillo, usando el proyecto Go propio como referencia, Firebase
para identidad y SurrealDB para datos. Firebase lo configurará el usuario.

1. Se revisó el estado y el alcance del respaldo, preservando los cambios
   existentes y las eliminaciones ya preparadas. No se incluyeron `.env` ni
   claves privadas detectadas por la inspección acotada de los archivos candidatos.
2. Se creó y publicó `14d8cc4` en `feature/first-steps`. Es un checkpoint,
   no una release estable; no se alteraron tags.
3. Se volvió a `main`, se creó `feature/auth` y se incorporó el checkpoint por
   fast-forward. `main` siguió en `260096f`; no se publicó ni fusionó esa rama.
4. Se instalaron las 13 skills de `firebase/agent-skills` mediante el CLI
   solicitado, exclusivamente en `.agents/skills` del backend. Se conserva
   `skills-lock.json`; no hubo instalación global de skills.
5. Se leyó la implementación de Go sin modificar ese proyecto ni acceder a
   su archivo de credenciales. Se contrastaron Firebase, SurrealDB y SurrealKit
   con fuentes oficiales. No se atribuyeron a cursos decisiones de código propio.
6. Se prepararon tres planes y cuatro fichas en `docs/fonts/auth`, vinculando
   el plan 01 y los diagramas anteriores para evitar objetivos contradictorios.

## Decisiones documentadas

| Necesidad | Fuente | Decisión propuesta |
| --- | --- | --- |
| Menos dispersión | [Código Go](../docs/fonts/auth/02-patron-auth-del-backend-go.md) y petición del usuario | Un crate, módulo auth con servicio y adaptadores; conservar Warp. |
| Inicio de sesión | [Firebase y Rust](../docs/fonts/auth/01-firebase-auth-y-verificacion-rust.md) | Firebase ID token verificado, cuenta propia sin contraseña ni estados. |
| Cuenta única | [SurrealDB](../docs/fonts/auth/03-surrealdb-cuentas-identidad-y-transacciones.md) | UUID propio e índice proyecto/UID, con tratamiento explícito de carreras. |
| Esquema reproducible | [SurrealKit comprobado](../docs/fonts/auth/04-surrealkit-version-y-comandos-verificados.md) | CLI externa beta.6; manifests, SQL congelado y snapshots versionados. |

El enfoque de Go se adapta: no se copia su `status`, su esquema SQL, sus dos
tablas obligatoriamente ni su SDK Admin. La política inicial propuesta verifica
tokens sin consulta de revocación por petición; su garantía está declarada.

## Comprobación ejecutada

| Comprobación | Resultado |
| --- | --- |
| Estado Git y diferencias staged/unstaged | Revisados antes de guardar; 64 archivos en el checkpoint. |
| Publicación y `git ls-remote` de `feature/first-steps` | Coincide con `14d8cc496b203a79872268cef0262c848816a7e7`. |
| Instalador y lock de skills | 13 entradas Firebase; los 13 directorios locales contienen `SKILL.md`. |
| `cargo check --workspace --all-targets --locked --offline` antes del respaldo | No pudo resolver `diskann` en la caché/índice local. No se afirma compilación correcta del checkpoint. |
| `git diff --cached --check` del estado recibido | Avisos de espacios finales en Markdown existente; se preservó ese contenido. |
| Descarga CLI SurrealKit beta.6 | SHA-256 coincidente con el publicado; solo en carpeta temporal. |
| Ayuda CLI, init mínimo y generación/lint de rollout temporal | Comandos comprobados sin conexión; se detectó que `check` no existe y que un segundo plan idéntico produce pasos vacíos. |
| Código Rust del runtime en esta preparación | Sin modificaciones respecto del checkpoint. |
| Enlaces y secuencias documentales | 51 enlaces locales de los ocho documentos nuevos válidos; secuencias sin duplicados en planes, fuentes auth y bitácora. |
| Inspección acotada de credenciales en archivos nuevos | 97 archivos revisados; sin coincidencias de los marcadores comunes buscados. No es una auditoría exhaustiva de secretos. |

La validación documental comprobó enlaces locales, fechas y numeración de
esta entrega. No se probaron Firebase real, consultas de cuentas,
SQLx/SurrealDB ni ejecución de migraciones contra un servidor. No se borraron
datos, contenedores o volúmenes.

## Continuidad

El siguiente incremento es implementar los planes empezando por la referencia
de compilación y la simplificación. Se abrirá otra entrada para el código real
y sus resultados. Esta preparación no se cerrará como «auth implementada».
