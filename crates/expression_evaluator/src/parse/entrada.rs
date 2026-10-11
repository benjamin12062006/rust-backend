//! Validación de una entrada completa.
//!
//! Las funciones de la gramática pueden terminar correctamente al encontrar un
//! símbolo que no pertenece a su nivel. Eso es necesario dentro de un grupo,
//! pero en la entrada principal no debe sobrar nada.
//!
//! `parsear` cumple por eso tres pasos:
//!
//! 1. rechaza una entrada vacía;
//! 2. delega la construcción del árbol a `parse_expression`;
//! 3. comprueba que el cursor haya llegado al final.
//!
//! Si queda un carácter conocido en una posición incorrecta se devuelve
//! `TokenInesperado`. Si el carácter ni siquiera pertenece al lenguaje se
//! devuelve `CaracterInesperado`.

use super::Parser;
use crate::{errores::ExpresionError, expresion::Expression};

impl<'a> Parser<'a> {
    // TODO 2: estudiar este método después de `gramatica.rs`.
    pub(crate) fn parsear(&mut self) -> Result<Expression, ExpresionError> {
        if self.texto.trim().is_empty() {
            return Err(ExpresionError::EntradaVacia);
        }

        // `?` devuelve inmediatamente el error si la gramática no puede crear
        // una expresión válida.
        let expresion = self.parse_expression()?;

        // Después de una expresión completa solo se permiten espacios y el
        // final del texto. `actual` ya omite esos espacios.
        if let Some((posicion, caracter)) = self.actual() {
            return if caracter.is_ascii_digit() || "+-*/()[]{}".contains(caracter) {
                Err(ExpresionError::TokenInesperado(posicion))
            } else {
                Err(ExpresionError::CaracterInesperado(caracter))
            };
        }

        Ok(expresion)
    }
}
