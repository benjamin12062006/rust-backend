# UserService

Este documento explica `application/user/user_service.rs` y la logica que estamos aprendiendo en esta parte.

## Rol

`UserService` implementa el caso de uso de usuario.

Conceptualmente queda en el centro:

```text
IUserUseCase -> UserService -> IUserRepository
```

`IUserUseCase` es el contrato de entrada. Es lo que un handler HTTP, CLI, test o job podria llamar para ejecutar una accion de usuario.

`IUserRepository` es el contrato de salida. Es lo que `UserService` necesita para leer o guardar usuarios en algun almacenamiento.

## Struct

```rust
pub struct UserService<R>
where
    R: IUserRepository,
{
    user_repostory: R,
}
```

`UserService` es generico sobre `R`.

`R: IUserRepository` significa que `R` puede ser cualquier tipo, pero debe implementar el contrato `IUserRepository`.

Esto permite que `UserService` no dependa de una base de datos concreta. El repositorio real podria ser SQL, Supabase, Firebase, Clerk, Redis o una implementacion fake para tests.

## Constructor

```rust
impl<R> UserService<R>
where
    R: IUserRepository,
{
    pub fn new(user_repostory: R) -> Self {
        Self { user_repostory }
    }
}
```

Este bloque solo construye `UserService`.

Para construir el struct basta con que `R` implemente `IUserRepository`. No necesitamos pedir `Send + Sync` aqui porque todavia no estamos prometiendo que `UserService` se usara como caso de uso concurrente.

## Implementar IUserUseCase

```rust
#[async_trait]
impl<R> IUserUseCase for UserService<R>
where
    R: IUserRepository + Send + Sync,
{
    // metodos async
}
```

Aqui `UserService<R>` empieza a cumplir el contrato `IUserUseCase`.

En este punto si pedimos:

```rust
R: IUserRepository + Send + Sync
```

porque `IUserUseCase` tambien exige:

```rust
pub trait IUserUseCase: Send + Sync
```

Como `UserService` contiene un repositorio `R`, Rust necesita que ese `R` tambien sea seguro para moverse y compartirse entre threads.

`Send + Sync` no se ejecuta en runtime. Son garantias que Rust verifica en compilacion.

## create_user

```rust
async fn create_user(&self, user: User) -> Result<User> {
    let mut new_user = user;
    let hashed_password = bcrypt::hash(&new_user.password)?;
    new_user.password = hashed_password;
    let user = self.user_repostory.create_user(&new_user).await?;
    Ok(user)
}
```

Este metodo orquesta el caso de uso de crear usuario.

El flujo es:

```text
recibir User
moverlo a new_user
hashear password
reemplazar password crudo por password hasheado
llamar al repositorio
esperar la operacion async
devolver el usuario creado
```

## let mut new_user = user

```rust
let mut new_user = user;
```

`user` se mueve a `new_user`.

No se copia. `new_user` pasa a ser el dueno del valor.

Usamos `mut` porque luego vamos a reemplazar:

```rust
new_user.password = hashed_password;
```

## bcrypt::hash

```rust
let hashed_password = bcrypt::hash(&new_user.password)?;
```

`bcrypt::hash` crea un hash de la contrasena.

Le pasamos:

```rust
&new_user.password
```

Eso significa que prestamos la contrasena como referencia de solo lectura.

No movemos el `String` fuera del struct. Solo permitimos que `bcrypt` lo lea para generar el hash.

Cuando termina la linea, el prestamo termina y podemos hacer:

```rust
new_user.password = hashed_password;
```

## Por Que No Usar unwrap

Antes se podria escribir:

```rust
let hashed_password = bcrypt::hash(&new_user.password).unwrap();
```

Pero `unwrap()` es peligroso en backend real. Si `bcrypt::hash` falla, `unwrap()` hace `panic` y puede tumbar la ejecucion.

La forma controlada es:

```rust
let hashed_password = bcrypt::hash(&new_user.password)?;
```

Esto funciona si el error de `bcrypt` puede convertirse a `CustomError`.

Para eso usamos en `common/error.rs` una variante como:

```rust
HashError(String)
```

Y, cuando corresponda, una conversion `From` desde el error de la dependencia hacia `CustomError`.

## await

```rust
let user = self.user_repostory.create_user(&new_user).await?;
```

`.await` espera a que el repositorio termine su trabajo async.

Esto puede representar una llamada a base de datos, API externa o cualquier operacion que no termina inmediatamente.

## ?

El operador `?` propaga errores.

Si la operacion devuelve `Ok(value)`, extrae el valor.

Si devuelve `Err(error)`, retorna ese error desde la funcion actual.

En este proyecto, las funciones usan:

```rust
Result<User>
```

que significa:

```rust
std::result::Result<User, CustomError>
```

## Referencias

```rust
self.user_repostory.create_user(&new_user).await?;
```

El repositorio recibe:

```rust
&new_user
```

Eso significa que solo prestamos el usuario para que el repositorio lo lea y lo guarde.

`UserService` sigue siendo dueno de `new_user` durante esta funcion.

En `get_user`:

```rust
async fn get_user(&self, id: &Uuid) -> Result<User> {
    let user = self.user_repostory.get_user(id).await?;
    Ok(user)
}
```

`id` ya es una referencia porque la firma dice:

```rust
id: &Uuid
```

Por eso no escribimos `&id`. Si lo hicieramos, seria una referencia a una referencia: `&&Uuid`.

## Runtime Y Concurrencia

Las lineas del `impl IUserUseCase` no "consumen" `Send + Sync` en runtime.

`Send + Sync` son reglas de compilacion.

Cuando el servidor este funcionando, varias requests pueden llamar estos metodos al mismo tiempo mediante tasks async. Para permitir ese diseno de forma segura, Rust exige que los tipos involucrados cumplan las restricciones correctas.

Resumen:

```text
async fn = comportamiento que se ejecuta cuando llega el caso de uso.
.await = espera una operacion async.
? = propaga error controlado.
Send = permite mover entre threads.
Sync = permite compartir referencias entre threads.
```

## Mejora Futura

Ahora `UserService` usa directamente:

```rust
pwhash::bcrypt
```

Mas adelante, para una arquitectura hexagonal mas estricta, podemos crear un puerto de salida:

```text
ports/out/password_hasher.rs
```

Y una implementacion concreta:

```text
infrastructure/crypto/bcrypt_password_hasher.rs
```

Asi `UserService` no dependeria de la libreria `pwhash` directamente.
