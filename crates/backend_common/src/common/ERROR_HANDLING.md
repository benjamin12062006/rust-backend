# Common Error Handling

Este documento explica el archivo `common/error.rs` y el papel que cumple dentro del backend.

## Objetivo

`common/error.rs` centraliza errores comunes de la aplicacion. La idea es no dejar que errores concretos de librerias externas, como `sqlx::Error` o errores de hashing, se filtren por todo el proyecto.

En lugar de eso, convertimos esos errores a un tipo propio:

```rust
CustomError
```

Asi el resto del backend trabaja con errores de la aplicacion, no con detalles internos de cada dependencia.

## Estructura Actual

Por ahora mantenemos todo en un solo archivo para aprender y avanzar sin crear demasiadas piezas prematuramente:

```text
common/
  mod.rs
  error.rs
```

Mas adelante, si el archivo crece demasiado, podemos separarlo asi:

```text
common/
  errors/
    mod.rs
    app_error.rs
    conversions.rs
    reject.rs
    result.rs
```

Por ahora no lo necesitamos. La version simple es suficiente.

## Result Alias

```rust
pub type Result<T> = std::result::Result<T, CustomError>;
```

Esto es un alias de tipo. No crea un tipo nuevo; solo evita repetir:

```rust
std::result::Result<T, CustomError>
```

Cuando escribimos:

```rust
Result<User>
```

Rust lo interpreta como:

```rust
std::result::Result<User, CustomError>
```

`T` representa el valor exitoso `Ok(T)`. `CustomError` representa el error `Err(CustomError)`.

## CustomError

```rust
#[derive(Debug, Clone, ThisError)]
pub enum CustomError {
    #[error("An error occurred during database interaction. {0}")]
    DatabaseError(String),

    #[error("An error occurred during http interaction. {0}")]
    HttpError(String),

    #[error("An error occurred while hashing password. {0}")]
    HashError(String),
}
```

`DatabaseError(String)`, `HttpError(String)` y `HashError(String)` son variantes tipo tupla. Cada una guarda un `String` interno con el detalle del error.

`Clone` permite duplicar el error si alguna parte del sistema necesita conservar una copia sin consumir el valor original.

`ThisError` viene del crate `thiserror`. Genera implementaciones utiles para errores, especialmente `Display` y `std::error::Error`.

`{0}` significa "usa el primer campo de esta variante". En `DatabaseError(String)`, ese primer campo es el `String`.

## From SqlxError

```rust
impl From<SqlxError> for CustomError {
    fn from(sqlx_error: SqlxError) -> Self {
        match sqlx_error.as_database_error() {
            Some(db_error) => CustomError::DatabaseError(db_error.to_string()),
            None => {
                eprintln!("error {:?}", sqlx_error);
                CustomError::DatabaseError(String::from("Unrecognized database error!"))
            }
        }
    }
}
```

`From<SqlxError> for CustomError` define una conversion:

```text
SqlxError -> CustomError
```

Esto no es herencia. Es una conversion explicita entre tipos.

`as_database_error()` pregunta si el error de `sqlx` viene directamente del motor de base de datos. Si devuelve `Some`, tenemos un error concreto de la DB, por ejemplo una restriccion unica, una tabla inexistente o una llave foranea fallida.

Si devuelve `None`, el error sigue siendo de `sqlx`, pero no es un error directo del motor SQL. Puede ser un timeout, una conexion fallida, un pool cerrado, `RowNotFound`, un error de decodificacion o un problema antes de que la base de datos respondiera formalmente.

`eprintln!` solo imprime el error en `stderr`. El valor que retorna la funcion es un `CustomError`.

## Into

Cuando existe:

```rust
impl From<SqlxError> for CustomError
```

Rust tambien permite usar:

```rust
sqlx_error.into()
```

`Into<CustomError>` significa "este valor puede convertirse en `CustomError`".

`From` define la conversion. `Into` usa la conversion.

## Warp Reject

```rust
impl warp::reject::Reject for CustomError {}
```

`warp` es un framework web para crear APIs HTTP.

`Reject` le dice a `warp` que `CustomError` puede usarse como causa de rechazo de una request.

El cuerpo esta vacio porque `Reject` funciona como marker trait en este caso. No exige metodos propios; solo marca el tipo como valido para `warp::reject::custom`.

En los controllers usamos directamente:

```rust
Err(reject::custom(error))
```

Eso transforma `CustomError` en un `Rejection`, que `warp` puede transportar dentro de su pipeline HTTP hasta el middleware de errores.

## HashError

`HashError(String)` existe para errores tecnicos del proceso de hashing de password.

Ejemplo de uso:

```rust
let hashed_password = bcrypt::hash(&new_user.password)
    .map_err(|error| CustomError::HashError(error.to_string()))?;
```

Esto evita usar `unwrap()`, que haria `panic` si el hash falla.

Por ahora `HashError` vive en `common/error.rs` porque aun no tenemos una capa `infrastructure/crypto` ni un puerto `PasswordHasher`.

Mas adelante, si escalamos la arquitectura, podriamos mover esta preocupacion a:

```text
ports/out/password_hasher.rs
infrastructure/crypto/bcrypt_password_hasher.rs
```

Asi `UserService` no dependeria directamente de `pwhash::bcrypt`.

## Resumen

`CustomError` es el error comun de la aplicacion.

`Result<T>` simplifica firmas que devuelven `Ok(T)` o `Err(CustomError)`.

`From<SqlxError>` permite convertir errores de `sqlx` a errores propios.

`Into<CustomError>` permite aceptar cualquier error convertible a `CustomError`.

`warp::reject::Reject` permite que `CustomError` participe en el sistema de errores HTTP de `warp`, usando `reject::custom(error)` desde los controllers.

`HashError` nos permite manejar fallos de hashing sin `unwrap()`.
