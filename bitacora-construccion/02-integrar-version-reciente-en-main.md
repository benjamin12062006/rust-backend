# 02 — Integrar la versión reciente en main y etiquetar el estado

Fecha de creación: 2026-10-11
Estado: Implementado y verificado
Ámbito: Integración Git y workspace del backend
Relacionada con: [Primera integración del evaluador](01-integrar-evaluador-expresiones.md)
Revisión de partida de `main`: `4b9b6527b22c1e266a96f1f7555a5d35821ba543`.
Origen: rama `v1.4.1`, commit `b4e746e25361590cc16150865eb5bac24f1f0312`.

## Necesidad y evidencia

El usuario pidió que `main` incorpore el backend reciente de `v1.4.1` y conserve
la biblioteca `expression_evaluator`, sus tests y documentación. Después solicitó
un tag de estado sobre el resultado integrado en `main`.

La inspección de Git demuestra que ambos historiales no comparten ancestro:
`git merge-base --all refs/heads/main refs/heads/v1.4.1` no encuentra una base;
el repositorio no es superficial y `v1.4.1` procede del commit raíz `8710962`.
Esta es evidencia del historial real, no una inferencia basada en nombres de ramas.

La versión reciente ya contiene un workspace con siete crates de backend,
dependencias comunes y tests que importan esos crates.[^F1] El avance de `main`
añade el evaluador y su prueba de consumo.[^F2]

## Decisión autorizada

- Fusionar explícitamente historiales independientes, sin reescribirlos.
- Usar `v1.4.1` como base del código y de la estructura del backend.
- Conservar el crate `expression_evaluator`, sus tests y las áreas documentales
  de `main`; adaptar el manifest a ambos avances.
- No conservar los módulos antiguos `src/features/` y `src/shared/` como una
  segunda implementación junto a sus reemplazos en los crates.
- Mantener archivos locales de entorno y del IDE en disco, pero fuera del nuevo
  snapshot versionado, conforme al `.gitignore` de la versión reciente.
- Preservar el tag `v1.4.1`. Crear `v1.5.0` sobre el commit final de `main`;
  incremento MINOR por la biblioteca añadida respecto al tag anterior, sin
  presentar el número del paquete Cargo (`0.1.0`) como versión del repositorio.
- No publicar ni crear la rama `feature/start-first-step`; esta solicitud de
  integración directa sustituye al destino que se había propuesto en la entrada 01.

## Implementación

Se ejecutó `git merge --allow-unrelated-histories --no-commit --no-ff -X theirs
refs/heads/v1.4.1`: el origen prevalece en los conflictos de contenido, pero
se revisó todo el árbol resultante antes de cerrar el commit.
El commit de integración conserva como padres ambos avances; no es un reemplazo
de refs ni una reescritura de los historiales.

El [manifest raíz](../Cargo.toml) conserva los siete crates del backend reciente
y añade `expression_evaluator` a los miembros del workspace y a las dependencias
del paquete raíz. `Cargo.lock` solo añade el evaluador respecto al origen.
Se mantienen íntegros los archivos del [evaluador](../crates/expression_evaluator/src/lib.rs),
su [test de consumo](../tests/expression_evaluator_tests.rs) y la entrada 01.

Los módulos viejos de `src/features/` y `src/shared/` se retiran del árbol
versionado porque sus reemplazos están en `crates/backend_*`. La documentación
de tests se toma de `tests/docs/`, sin conservar una copia antigua de
`tests/TESTING_LAYER.md`.

Se usó `git rm --cached` para los archivos de entorno y `.idea` previamente
rastreados. No se borraron del disco: se verificó su conservación mediante hashes,
sin exponer contenidos. Esto los retira del nuevo snapshot, no de commits históricos.

Solo se corrigieron seis líneas con whitespace final en `tests/support/fakes.rs`
y una línea vacía al final de `tests/docs/UI_APP_TESTING_WARP.md`, heredadas del
origen, para que el diff preparado pase `git diff --cached --check`.
No se reformateó todo el proyecto ni se cambiaron esos comportamientos.

No se añaden endpoints, dependencias externas nuevas ni cambios de comportamiento
al evaluador. La versión de estado elegida para el commit integrado es `v1.5.0`;
se etiqueta después del commit y se comprueba que el tag y `main` coincidan.

## Comprobación

| Comprobación ejecutada antes del commit | Resultado |
| --- | --- |
| `RUN_DB_TESTS=false cargo test --workspace --offline --locked` | Suite correcta: 55 registros de tests y 2 doctests sin fallos. El test de base de datos retorna sin conectarse por ser opt-in. |
| `cargo check --workspace --all-targets --offline --locked` | Compilan los nueve paquetes y sus targets, sin advertencias. |
| `cargo fmt -p expression_evaluator -- --check` | Correcto. |
| `rustfmt --edition 2024 --check tests/expression_evaluator_tests.rs` | Correcto. |
| `git diff --check` y `git diff --cached --check` | Correctos tras la corrección mínima de whitespace. |
| `cargo metadata --offline --locked --no-deps --format-version 1` | Nueve miembros: paquete raíz, siete crates de backend y evaluador; dependencia local correcta. |
| Comparación de archivos con los dos commits de partida | 120 archivos del origen conservados exactamente antes de corregir whitespace; tras la corrección, solo esos dos difieren en formato. Los 15 archivos conservados del evaluador y documentación previa no cambian. |
| Índice Git y hashes de archivos locales | Sin `.env`, `.idea`, módulos obsoletos ni documento duplicado en el índice; nueve archivos locales conservados íntegros. |

No se inició el servidor ni se comprobaron conexiones reales a PostgreSQL,
SurrealDB o Qdrant. No se ejecutó CI remoto ni se publicó el resultado.

## Continuidad

`main` pasa a ser la línea de trabajo que contiene ambos avances. El tag anterior
`v1.4.1` conserva su commit original y sigue permitiendo visitar el punto previo.
La rama `v1.4.1` también se conserva. Futuras versiones pueden etiquetar commits
de `main` sin crear una rama por número de versión.

## Fuentes

[^F1]: Código propio consultado el 2026-10-11 mediante
    `git show refs/heads/v1.4.1:Cargo.toml` y
    `git show refs/heads/v1.4.1:tests/database_integration_tests.rs`.
    Revisión: `b4e746e25361590cc16150865eb5bac24f1f0312`.
    Localizadores: `workspace.members`, `workspace.dependencies` y test
    `database_connection_ping_is_opt_in`.

[^F2]: [Entrada 01](01-integrar-evaluador-expresiones.md),
    [manifest del evaluador](../crates/expression_evaluator/Cargo.toml) y
    [pruebas de consumo](../tests/expression_evaluator_tests.rs).
    Revisión de respaldo: `4b9b6527b22c1e266a96f1f7555a5d35821ba543`.
