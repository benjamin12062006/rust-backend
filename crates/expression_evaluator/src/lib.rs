//! Evaluador de expresiones matemáticas.
//!
//! Copiado y adaptado de `PRACTICE-rs/crates/i/src/interpreters/exercise1`.
//! El parser transforma un `&str` en un árbol interno y el evaluador lo recorre.
//! La API principal no lee la consola ni depende del backend.
//!
//! ```
//! use expression_evaluator::{calcular, ExpresionError};
//!
//! assert_eq!(calcular("(10 + 5) * 2"), Ok(30.0));
//! assert_eq!(calcular("1 / 0"), Err(ExpresionError::DivisionPorCero));
//! ```
//!
//! Orden de estudio recomendado:
//!
//! 1. `expresion.rs`: qué es el árbol de una operación.
//! 2. `evaluacion.rs`: cómo se recorre ese árbol para obtener un resultado.
//! 3. `parse/`: cómo el texto se convierte en el árbol.
//! 4. `errores.rs`: cómo se describen los fallos del programa.
//! 5. `aplicacion.rs`: adaptador auxiliar de consola, no usado por el backend.

mod aplicacion;
mod errores;
mod evaluacion;
mod expresion;
mod parse;

pub use errores::ExpresionError;
use evaluacion::evaluar;
use parse::Parser;

/// Calcula una expresión sin depender de la consola.
///
/// Acepta números enteros y decimales, `+`, `-`, `*`, `/`, espacios y grupos
/// con `()`, `[]` o `{}`. No implementa signos unarios, variables ni funciones.
///
/// El operador `?` detiene la función si el parser devuelve `Err`. Cuando el
/// texto es válido, toma prestado el árbol con `&expresion` para evaluarlo sin
/// transferir su ownership.
pub fn calcular(texto: &str) -> Result<f64, ExpresionError> {
    let mut parser = Parser::new(texto);
    let expresion = parser.parsear()?;
    evaluar(&expresion)
}

pub use aplicacion::run_exercise1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respeta_la_prioridad_de_operadores() {
        assert_eq!(calcular("2 + 3 * 4"), Ok(14.0));
    }

    #[test]
    fn respeta_los_grupos() {
        assert_eq!(calcular("(2 + 3) * 4"), Ok(20.0));
    }

    #[test]
    fn acepta_parentesis_corchetes_y_llaves() {
        assert_eq!(calcular("{ (10 + 2) * [8 / (3 + 1)] } + 5"), Ok(29.0));
    }

    #[test]
    fn resta_y_divide_de_izquierda_a_derecha() {
        assert_eq!(calcular("20 / 2 / 2 - 1"), Ok(4.0));
    }

    #[test]
    fn detecta_division_por_cero() {
        assert_eq!(
            calcular("8 / (3 - 3)"),
            Err(ExpresionError::DivisionPorCero)
        );
    }

    #[test]
    fn detecta_cierre_faltante() {
        assert_eq!(
            calcular("(2 + 3"),
            Err(ExpresionError::FaltaSimboloDeCierre(')'))
        );
    }

    #[test]
    fn detecta_entrada_vacia() {
        assert_eq!(calcular("   "), Err(ExpresionError::EntradaVacia));
    }

    #[test]
    fn detecta_numero_invalido() {
        assert_eq!(calcular("1..2"), Err(ExpresionError::NumeroInvalido));
    }

    #[test]
    fn detecta_caracter_invalido() {
        assert_eq!(
            calcular("2 + a"),
            Err(ExpresionError::CaracterInesperado('a'))
        );
    }

    #[test]
    fn detecta_expresion_incompleta() {
        assert_eq!(calcular("2 +"), Err(ExpresionError::FinInesperado));
    }
}
