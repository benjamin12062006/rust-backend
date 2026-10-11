//! Estado y navegación del parser.
//!
//! `Parser<'a>` mantiene dos vistas de la misma entrada:
//!
//! - `texto: &'a str` conserva el texto completo para extraer slices;
//! - `caracteres` mantiene la posición actual mientras avanza.
//!
//! El lifetime `'a` expresa que el parser no puede vivir más tiempo que el
//! texto prestado. No copia la entrada ni se convierte en dueño de ella.
//!
//! `CharIndices` produce pares `(posición_en_bytes, char)`. Las posiciones en
//! bytes son necesarias para cortar posteriormente `texto[inicio..final]`.
//! `Peekable` añade `peek`, que permite mirar el siguiente carácter sin
//! consumirlo.
//!
//! La lógica compleja se agrupa por responsabilidad:
//!
//! - `entrada.rs` valida la expresión completa;
//! - `gramatica.rs` reconoce la estructura y prioridad de las operaciones;
//! - `numeros.rs` convierte una porción del texto en `f64`.

use std::{iter::Peekable, str::CharIndices};

mod entrada;
mod gramatica;
mod numeros;

pub(super) struct Parser<'a> {
    texto: &'a str,
    caracteres: Peekable<CharIndices<'a>>,
}

impl<'a> Parser<'a> {
    /// Crea el cursor inicial sobre `texto`.
    pub(crate) fn new(texto: &'a str) -> Self {
        // fn new(...)          // solo módulo actual
        // pub(super) fn new()  // módulo padre
        // pub(crate) fn new()  // todo el crate
        // pub fn new()         // cualquier código que pueda acceder al tipo
        // pub(in ruta) fn new() // únicamente dentro de esa ruta
        Self {
            texto,
            // estudiar `char_indices()` y `peekable()` al final.
            caracteres: texto.char_indices().peekable(),
        }
    }
    // TODO
    /// Consume todos los espacios consecutivos desde la posición actual.
    fn omitir_espacios(&mut self) {
        while self
            .caracteres
            .next_if(|&(_, caracter)| caracter.is_whitespace())
            .is_some()
        {}
    }

    /// Devuelve el próximo símbolo útil sin retirarlo del iterador.
    ///
    /// Requiere `&mut self` porque antes puede consumir espacios.
    pub(super) fn actual(&mut self) -> Option<(usize, char)> {
        self.omitir_espacios();
        self.caracteres.peek().copied()
    }

    /// Consume un símbolo que otra función ya reconoció.
    pub(super) fn avanzar(&mut self) {
        self.caracteres.next();
    }
}
