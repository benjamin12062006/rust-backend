//! Reconocimiento y conversión de números.
//!
//! Esta responsabilidad necesita tanto el iterador como el texto original:
//!
//! 1. guarda el índice inicial del número;
//! 2. consume dígitos y puntos consecutivos;
//! 3. guarda el índice exclusivo donde termina;
//! 4. extrae el slice `&texto[inicio..final_exclusivo]`;
//! 5. delega en `parse::<f64>()` la validación final.
//!
//! El parser acepta provisionalmente los puntos mientras recorre el texto.
//! Después, `f64::parse` decide si el conjunto es válido. Por eso `1..2`
//! termina como `NumeroInvalido`.

use super::Parser;
use crate::{errores::ExpresionError, expresion::Expression};

impl<'a> Parser<'a> {
    pub(super) fn parse_number(&mut self) -> Result<Expression, ExpresionError> {
        // `actual` devuelve una posición en bytes, compatible con el slicing de
        // `str`. El carácter ya fue reconocido como dígito o punto.
        let inicio = self
            .actual()
            .map(|(posicion, _)| posicion)
            .ok_or(ExpresionError::FinInesperado)?;
        let mut final_exclusivo = inicio;

        while let Some((posicion, caracter)) = self
            .caracteres
            .next_if(|&(_, caracter)| caracter.is_ascii_digit() || caracter == '.')
        {
            // Los rangos de Rust excluyen el extremo derecho. `len_utf8`
            // desplaza ese extremo hasta después del carácter consumido.
            final_exclusivo = posicion + caracter.len_utf8();
        }

        // `map_err` traduce el error técnico de `f64` al error propio del
        // dominio, evitando exponer detalles internos al resto del programa.
        let numero = self.texto[inicio..final_exclusivo]
            .parse::<f64>()
            .map_err(|_| ExpresionError::NumeroInvalido)?;

        Ok(Expression::Number(numero))
    }
}
