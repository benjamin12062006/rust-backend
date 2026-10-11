//! Evaluación recursiva del árbol `Expression`.
//!
//! La función recibe `&Expression`, una referencia compartida. Solo necesita
//! leer el árbol, por lo que no toma ownership ni lo modifica.
//!
//! Cada llamada resuelve un nodo:
//!
//! - `Number` es el caso base y devuelve directamente su valor;
//! - una operación evalúa primero sus hijos y después combina los resultados;
//! - `Divide` puede fallar y por eso toda la función devuelve `Result`.
//!
//! El operador `?` propaga hacia arriba cualquier error encontrado en una rama.
//! Así, una división por cero dentro de un árbol grande detiene toda la
//! evaluación sin usar `panic!`.

use super::{errores::ExpresionError, expresion::Expression};

pub(super) fn evaluar(expresion: &Expression) -> Result<f64, ExpresionError> {
    match expresion {
        // `numero` es `&f64` porque estamos haciendo match sobre `&Expression`.
        // El `*` copia el valor `f64`; no es una multiplicación.
        Expression::Number(numero) => Ok(*numero),
        Expression::Add(izquierda, derecha) => Ok(evaluar(izquierda)? + evaluar(derecha)?),
        Expression::Subtract(izquierda, derecha) => Ok(evaluar(izquierda)? - evaluar(derecha)?),
        Expression::Multiply(izquierda, derecha) => Ok(evaluar(izquierda)? * evaluar(derecha)?),
        Expression::Divide(izquierda, derecha) => {
            // Evaluar por separado permite comprobar el divisor antes de
            // realizar la operación.
            let dividendo = evaluar(izquierda)?;
            let divisor = evaluar(derecha)?;

            if divisor == 0.0 {
                return Err(ExpresionError::DivisionPorCero);
            }

            Ok(dividendo / divisor)
        }
    }
}
