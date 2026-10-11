//! Gramática de las expresiones matemáticas.
//!
//! Este parser usa descenso recursivo: cada método representa un nivel de la
//! gramática y llama al nivel inmediatamente más fuerte.
//!
//! ```text
//! expression = term (("+" | "-") term)*
//! term       = primary (("*" | "/") primary)*
//! primary    = number | grupo
//! grupo      = apertura expression cierre
//! ```
//!
//! Esta jerarquía crea la precedencia sin comparar números de prioridad.
//! `parse_expression` solicita términos completos, por lo que `3 * 4` ya está
//! agrupado antes de que se construya `2 + (3 * 4)`.
//!
//! Los bucles producen asociatividad de izquierda a derecha. Por ejemplo,
//! `20 / 2 / 2` se transforma en `(20 / 2) / 2`.

use super::Parser;
use crate::{errores::ExpresionError, expresion::Expression};

impl<'a> Parser<'a> {
    // Suma y resta tienen la prioridad más baja.
    pub(super) fn parse_expression(&mut self) -> Result<Expression, ExpresionError> {
        // Todo lo que exista a la izquierda debe ser como mínimo un término
        // válido. Esto también permite expresiones sin suma ni resta.
        let mut izquierda = self.parse_term()?;

        loop {
            match self.actual() {
                Some((_, '+')) => {
                    self.avanzar();
                    let derecha = self.parse_term()?;
                    // El árbol anterior pasa a ser el hijo izquierdo del nuevo
                    // nodo. Así se conserva la asociación de izquierda a derecha.
                    izquierda = Expression::Add(Box::new(izquierda), Box::new(derecha));
                }
                Some((_, '-')) => {
                    self.avanzar();
                    let derecha = self.parse_term()?;
                    izquierda = Expression::Subtract(Box::new(izquierda), Box::new(derecha));
                }
                _ => break,
            }
        }

        Ok(izquierda)
    }

    // Multiplicación y división se agrupan antes que suma y resta.
    fn parse_term(&mut self) -> Result<Expression, ExpresionError> {
        // Un término siempre comienza con una unidad indivisible en este nivel:
        // un número o un grupo.
        let mut izquierda = self.parse_primary()?;

        loop {
            match self.actual() {
                Some((_, '*')) => {
                    self.avanzar();
                    let derecha = self.parse_primary()?;
                    izquierda = Expression::Multiply(Box::new(izquierda), Box::new(derecha));
                }
                Some((_, '/')) => {
                    self.avanzar();
                    let derecha = self.parse_primary()?;
                    izquierda = Expression::Divide(Box::new(izquierda), Box::new(derecha));
                }
                _ => break,
            }
        }

        Ok(izquierda)
    }

    // Un primario es un número o una expresión agrupada.
    fn parse_primary(&mut self) -> Result<Expression, ExpresionError> {
        // `actual` mira sin consumir. La función elegida será responsable de
        // consumir todos los caracteres que formen ese primario.
        match self.actual() {
            Some((_, caracter)) if caracter.is_ascii_digit() || caracter == '.' => {
                self.parse_number()
            }
            Some((_, '(')) => self.parse_group(')'),
            Some((_, '[')) => self.parse_group(']'),
            Some((_, '{')) => self.parse_group('}'),
            Some((posicion, caracter)) if "+-*/)]}".contains(caracter) => {
                Err(ExpresionError::TokenInesperado(posicion))
            }
            Some((_, caracter)) => Err(ExpresionError::CaracterInesperado(caracter)),
            None => Err(ExpresionError::FinInesperado),
        }
    }

    fn parse_group(&mut self, cierre_esperado: char) -> Result<Expression, ExpresionError> {
        // El carácter actual es la apertura que `parse_primary` ya reconoció.
        self.avanzar();

        // Dentro del grupo vuelve a permitirse una expresión completa. Esta
        // llamada recursiva es lo que admite grupos anidados.
        let expresion = self.parse_expression()?;

        match self.actual() {
            Some((_, caracter)) if caracter == cierre_esperado => {
                // El cierre pertenece al grupo actual; se consume antes de
                // devolver el árbol interior.
                self.avanzar();
                Ok(expresion)
            }
            _ => Err(ExpresionError::FaltaSimboloDeCierre(cierre_esperado)),
        }
    }
}
