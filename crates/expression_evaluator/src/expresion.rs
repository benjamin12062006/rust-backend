//! Representación interna de una expresión matemática.
//!
//! Este enum es un árbol de sintaxis abstracta, también llamado AST. Cada
//! variante representa el significado de una parte de la expresión, no los
//! caracteres originales.
//!
//! Por ejemplo, `2 + 3 * 4` se representa conceptualmente así:
//!
//! ```text
//! Add
//! ├── Number(2)
//! └── Multiply
//!     ├── Number(3)
//!     └── Number(4)
//! ```
//!
//! Las operaciones guardan otras dos `Expression`. Esa recursión tendría un
//! tamaño infinito si los hijos se guardaran directamente. `Box<Expression>`
//! coloca cada hijo en el heap y deja en el enum un puntero de tamaño conocido.
//!
//! `Debug` permite inspeccionar el árbol y `PartialEq` permite compararlo en
//! tests. `pub(super)` permite su uso interno desde la raíz del crate.

#[derive(Debug, PartialEq)]
pub(super) enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}
