//! Adaptador de entrada y salida por consola.
//!
//! Este archivo conoce `stdin`, `stdout`, `print!` y `println!`, pero no conoce
//! los detalles del parser ni del árbol. Su única responsabilidad es:
//!
//! 1. solicitar una expresión;
//! 2. leerla como `String`;
//! 3. llamar a `calcular`;
//! 4. presentar el `Ok` o el `Err`.
//!
//! Mantener la consola separada permite probar `calcular` directamente, sin
//! simular teclado ni capturar texto de la terminal.

use std::io::{self, Write};

use super::calcular;

/// Ejecuta el adaptador original de consola.
///
/// Para usar la biblioteca desde el backend, llamar a [`calcular`] en su lugar.
pub fn run_exercise1() {
    print!("Escribe una expresión: ");

    // `print!` no agrega salto de línea. El `flush` obliga a mostrar el mensaje
    // antes de que `read_line` se quede esperando la entrada del usuario.
    if let Err(error) = io::stdout().flush() {
        eprintln!("No se pudo mostrar el mensaje: {error}");
        return;
    }

    let mut entrada = String::new();

    // `read_line` necesita `&mut String` porque añade dentro de `entrada` los
    // caracteres leídos, incluido normalmente el salto de línea final.
    if let Err(error) = io::stdin().read_line(&mut entrada) {
        eprintln!("No se pudo leer la expresión: {error}");
        return;
    }

    match calcular(&entrada) {
        Ok(resultado) => println!("Resultado: {resultado}"),
        Err(error) => println!("Error: {error}"),
    }
}
