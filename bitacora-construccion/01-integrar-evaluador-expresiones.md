# 01 — Integrar el evaluador de expresiones como dependencia

Fecha de creación: 2026-10-11
Última actualización: 2026-10-12
Estado: Registro histórico; biblioteca retirada del backend actual
Ámbito: Backend y biblioteca `expression_evaluator`
Relacionada con: [Integración en main](02-integrar-version-reciente-en-main.md)
Revisión de partida: `d2b04c4c40a00e2e59f8d6474b6548ace5f24874` en `main`.

## Evolución posterior

Este documento conserva la integración histórica del evaluador en el backend.
El backend actual ya no depende de esa biblioteca ni contiene sus pruebas de
consumo. Las rutas del evaluador que aparecen aquí corresponden al snapshot
histórico del tag `v1.5.0` de este repositorio, no a archivos del árbol actual.
Se pueden consultar sin cambiar de rama con
`git show v1.5.0:crates/expression_evaluator/src/lib.rs`.

## Necesidad y punto de partida

Copiar el evaluador propio de `PRACTICE-rs/crates/i` dentro de este repositorio
y dejarlo disponible como dependencia local del backend. No modificar el original,
no cambiar de rama ni integrar automáticamente el código del tag `v1.4.1`.

## Evidencia e interpretación

El ejercicio ya separa el parser, el árbol, la evaluación y la consola.
La función `calcular` coordina el proceso sin entrada/salida, pero es privada;
`ExpresionError` también tiene visibilidad restringida.[^F1]
Esta es la frontera reutilizable para el backend: texto → árbol → resultado.

La consigna de memoria propone recibir `&str` y devolver un `Result`; su alcance
inicial es menor que el código actual, que también implementa resta, división,
corchetes y llaves.[^F2] La copia se basa en los archivos actuales, no en suponer
que el commit del proyecto de prácticas contiene esos cambios.

## Alternativas y decisión

- **Elegido:** copiar únicamente el evaluador a `crates/expression_evaluator`,
  hacerlo miembro de un workspace y usar una dependencia `path` relativa.
- **Descartado:** depender de la ruta absoluta del proyecto de prácticas; impediría
  reproducir este backend en otra máquina.
- **Descartado:** copiar todo `practice_i`; el ejercicio de iteradores no forma
  parte del evaluador.

Propuesta propia: nombre funcional, API pública `calcular` y `ExpresionError`,
parser y AST internos. Se conserva el adaptador de consola como API auxiliar
`run_exercise1`, pero no se llama desde el servidor.
No se añade un endpoint ni se cambia el comportamiento HTTP.

## Implementación y funcionamiento

| Decisión | Archivo y símbolo | Adaptación |
| --- | --- | --- |
| Dependencia reproducible dentro del repositorio | `v1.5.0:Cargo.toml`, `workspace.members` y `dependencies.expression_evaluator` | Ruta relativa `crates/expression_evaluator`; lockfile actualizado sin nuevas dependencias externas. |
| Nombre funcional e independencia del workspace original | `v1.5.0:crates/expression_evaluator/Cargo.toml` | Paquete `expression_evaluator`, versión `0.1.0`, edición `2024`, publicación deshabilitada. |
| Frontera pública sin consola[^F1] | `v1.5.0:crates/expression_evaluator/src/lib.rs`, `calcular` | El antiguo `exercise1/mod.rs` pasa a ser la raíz de la biblioteca y conserva los diez tests. |
| Errores distinguibles por los consumidores[^F1] | `v1.5.0:crates/expression_evaluator/src/errores.rs`, `ExpresionError` | Enum público, reexportado desde la raíz, con `Display` original y `std::error::Error` añadido. |
| Mantener el algoritmo original[^F1] | `v1.5.0:crates/expression_evaluator/src/parse/mod.rs` y `v1.5.0:crates/expression_evaluator/src/evaluacion.rs` | Solo se adaptan rutas y visibilidad interna; la gramática y evaluación no cambian. |
| Verificar consumo desde el backend | `v1.5.0:tests/expression_evaluator_tests.rs` | Importación directa del crate, resultado, variante de error y compatibilidad con el trait estándar. |

El flujo histórico era `calcular(&str)` → `Parser::parsear` → árbol interno →
`evaluar` → `Result<f64, ExpresionError>`. El backend podía usar esa función
sin inicializar bases de datos ni simular entrada por consola.

El adaptador original se conservaba en
`v1.5.0:crates/expression_evaluator/src/aplicacion.rs`, como `run_exercise1`,
para uso manual; no se invocaba desde el servidor.

## Comprobación

| Comprobación ejecutada | Resultado |
| --- | --- |
| `cargo test -p expression_evaluator --offline` | 10 tests originales y 1 doctest correctos. |
| `cargo test -p backend --test expression_evaluator_tests --offline --locked` | 3 tests de integración correctos. |
| `cargo check --workspace --all-targets --offline --locked` | Compilan biblioteca, binario y targets de pruebas del workspace. |
| `cargo fmt -p expression_evaluator -- --check` | Correcto. |
| `rustfmt --edition 2024 --check tests/expression_evaluator_tests.rs` | Correcto. |
| `git diff --check` | Sin errores de whitespace en el diff rastreado. |
| Comparación de archivos mediante Python | `evaluacion.rs`, `parse/gramatica.rs` y `parse/numeros.rs` coinciden con el original salvo la ruta de imports; los diez tests coinciden íntegramente. |
| `cargo metadata --offline --locked --no-deps --format-version 1` | `backend` y `expression_evaluator` pertenecen al workspace; la dependencia del backend se resuelve al crate copiado. |
| Inventario de bitácora | Entrada `01`, sin colisiones; siguiente prefijo `02`. |

La revisión independiente no encontró defectos en la integración y confirmó
las diferencias previstas frente al original. El revisor volvió a ejecutar
los tests del crate y los tres tests de integración, además de
`cargo clippy -p expression_evaluator --all-targets --offline -- -D warnings`,
todos correctos. Su comprobación `cargo fmt --all -- --check` encontró diferencias
de formato preexistentes exclusivamente fuera de los archivos de este incremento;
no se reformateó código ajeno.

La compilación del backend muestra una advertencia preexistente por `qdrant`
sin usar en `src/shared/bootstrap/features/user.rs:16`; no se modificó ese archivo.
No se ejecutó la suite completa del backend ni se inició el servidor o las bases
de datos. La comprobación HTTP queda fuera de este incremento porque no hay una
ruta nueva.

## Consecuencias y continuidad

En el estado histórico, la biblioteca estaba disponible desde los módulos del backend mediante
`use expression_evaluator::{calcular, ExpresionError};`.
La copia es independiente: cambios posteriores en el proyecto de prácticas no se
sincronizan automáticamente.

Se preservan los límites originales: números de punto flotante, sin operadores
unarios, variables o funciones. No se añadieron límites de longitud o profundidad;
antes de exponer entradas no confiables por HTTP habría que definir esos límites.
No se afirma que esta copia convierta el ejercicio en un motor seguro para entradas
arbitrarias.

El original no fue modificado. A petición del usuario, esta implementación se
conserva mediante un commit de respaldo en `main`, junto con los `.gitkeep`
de la estructura documental inicializada antes de este incremento.
El archivo ajeno `.env.local.example` queda fuera del commit.
No se crea un tag ni se publica este incremento.

El destino solicitado para continuar es una rama nueva `feature/start-first-step`
basada en la rama reciente `v1.4.1`, no en este `main` antiguo. Antes del traslado
se visitará el tag `v1.4.1` para comprobar la recuperación del código guardado.
Este commit conserva la primera integración y sus comprobaciones; no afirma que
el traslado a la versión reciente ya esté realizado.

## Fuentes

[^F1]: Código propio consultado el 2026-10-11:
    [exercise1/mod.rs](/Users/benji/Documents/RUST/aprendisajeRS/PRACTICE-rs/crates/i/src/interpreters/exercise1/mod.rs),
    líneas 25–47 y tests 49–109; `errores.rs`, líneas 13–50;
    `parse/{mod,entrada,gramatica,numeros}.rs`; `evaluacion.rs`, `expresion.rs`
    y `aplicacion.rs`.
    Ruta relativa a la biblioteca: `PRACTICE-rs/crates/i/src/interpreters/exercise1/`.
    HEAD del original: `cb105fe8a4f1a2b440d3248d8e16e8d82e5ad259`;
    el directorio `exercise1/` estaba sin rastrear. Esta referencia identifica
    el contexto Git, no una revisión que ya contenga el ejercicio.

[^F2]: [Consigna propia: evaluador de expresiones básico](/Users/benji/Documents/apps_uso_propio/organizar-tareas-ai/data/memoria/logicaRust/ejercicio-evaluador-expresiones.md).
    Secciones «Entrada», «Salida» y «Plan de implementación».
    Consultada mediante la skill `memo` el 2026-10-11.
