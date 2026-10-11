//! Errores posibles durante el parsing y la evaluación.
//!
//! Usar un enum conserva información estructurada sobre cada fallo. El código
//! puede comparar variantes concretas en los tests, mientras que la persona
//! que usa el programa recibe un mensaje legible mediante `fmt::Display`.
//!
//! Algunas variantes pertenecen al parser (`EntradaVacia`,
//! `CaracterInesperado`, etc.) y `DivisionPorCero` pertenece al evaluador. Se
//! reúnen porque `calcular` devuelve un único `Result<f64, ExpresionError>`.

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ExpresionError {
    EntradaVacia,
    NumeroInvalido,
    CaracterInesperado(char),
    TokenInesperado(usize),
    FinInesperado,
    FaltaSimboloDeCierre(char),
    DivisionPorCero,
}

impl std::error::Error for ExpresionError {}

impl fmt::Display for ExpresionError {
    /// Convierte cada variante en el mensaje que se mostrará en la terminal.
    ///
    /// `formatter` es el destino proporcionado por Rust. `write!` escribe en
    /// ese destino y devuelve `fmt::Result`, que es el resultado exigido por
    /// el trait `Display`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntradaVacia => write!(formatter, "la entrada está vacía"),
            Self::NumeroInvalido => write!(formatter, "hay un número inválido"),
            Self::CaracterInesperado(caracter) => {
                write!(formatter, "el carácter '{caracter}' no es válido")
            }
            Self::TokenInesperado(posicion) => {
                write!(
                    formatter,
                    "hay un símbolo inesperado en la posición {posicion}"
                )
            }
            Self::FinInesperado => write!(formatter, "la expresión terminó antes de tiempo"),
            Self::FaltaSimboloDeCierre(simbolo) => {
                write!(formatter, "falta el símbolo de cierre '{simbolo}'")
            }
            Self::DivisionPorCero => write!(formatter, "no se puede dividir entre cero"),
        }
    }
}
