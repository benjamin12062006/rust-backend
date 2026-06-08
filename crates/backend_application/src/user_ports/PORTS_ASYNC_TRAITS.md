# Ports, Async Traits, Send And Sync

Este documento resume los conceptos usados en `ports/in/user_use_case.rs` y `ports/out/user_repository.rs`.

## Ports

En arquitectura hexagonal, un puerto es un contrato. Define que necesita la aplicacion sin decidir que tecnologia concreta lo hara.

`ports/in` contiene contratos de entrada. Representa lo que la aplicacion ofrece hacia fuera, por ejemplo casos de uso como crear, buscar o listar usuarios.

`ports/out` contiene contratos de salida. Representa lo que la aplicacion necesita del mundo externo, por ejemplo repositorios, servicios de tokens, hash de contrasenas o clientes HTTP.

La idea es que `application` dependa de contratos, no de SQL, Redis, Firebase, Clerk o cualquier proveedor concreto.

## In And Out

`in` es la entrada hacia la aplicacion. Lo llama algo externo para pedirle a la aplicacion que ejecute un caso de uso.

Ejemplos de entrada:

- HTTP handler.
- CLI.
- App desktop.
- Job o cron.
- Tests.

Flujo tipico:

```text
HTTP -> ports/in -> application
```

`out` es la salida desde la aplicacion hacia una dependencia externa. Lo llama la propia aplicacion para usar algo fuera de su nucleo.

Ejemplos de salida:

- Base de datos.
- Redis.
- Firebase.
- Clerk.
- Supabase.
- Email.
- Storage.
- APIs externas.

Flujo tipico:

```text
application -> ports/out -> database adapter
```

Regla mental:

```text
in = lo que otros usan para pedirle algo a tu app.
out = lo que tu app usa para pedirle algo al exterior.
```

En este backend:

```text
IUserUseCase (in) = "quiero ejecutar el caso de uso de usuario".
IUserRepository (out) = "necesito leer/guardar usuarios en algun almacenamiento".
```

## IUserUseCase

```rust
#[async_trait]
pub trait IUserUseCase: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User>;
    async fn get_user(&self, id: Uuid) -> Result<User>;
    async fn list_users(&self) -> Result<Vec<User>>;
}
```

Este trait define el contrato de entrada para operaciones de usuario. La capa HTTP no necesita saber como se guardan los usuarios; solo necesita invocar este contrato.

## async

`async` permite pausar y continuar trabajo sin bloquear un thread.

Sirve para operaciones que esperan algo externo, como base de datos, red, archivos, timers o sockets.

Una funcion async no devuelve el valor final inmediatamente. Devuelve un `Future`.

```rust
async fn get_user() -> Result<User>
```

Conceptualmente se parece a:

```rust
fn get_user() -> impl Future<Output = Result<User>>
```

## Future

Un `Future` es una tarea async que todavia no termino.

```rust
Future<Output = T>
```

significa: cuando este trabajo termine, producira un valor `T`.

`await` espera a que el `Future` termine y extrae su `Output`.

## async_trait

`async_trait` viene del crate `async-trait`.

Permite escribir `async fn` dentro de traits de forma sencilla:

```rust
#[async_trait]
pub trait IUserUseCase {
    async fn create_user(&self, user: User) -> Result<User>;
}
```

Por detras, el macro reescribe el metodo async a una forma basada en `Future`, normalmente parecida a:

```rust
fn create_user<'a>(
    &'a self,
    user: User,
) -> Pin<Box<dyn Future<Output = Result<User>> + Send + 'a>>;
```

No escribimos esto a mano porque es mas verboso y dificil de leer. `async_trait` nos deja expresar la intencion de forma limpia.

## Pin

`Pin` evita que el `Future` se mueva de lugar en memoria mientras esta en progreso.

Esto importa porque algunos futures generados por `async` pueden guardar referencias internas mientras esperan. Moverlos de memoria podria romper esas referencias.

## Box

`Box` guarda el valor en heap y deja un puntero de tamano conocido.

En este contexto ayuda a devolver un `Future` cuyo tipo concreto no queremos escribir manualmente.

## dyn

`dyn Future` significa que tratamos el future como un trait object.

No necesitamos conocer el tipo concreto exacto del future generado por cada implementacion. Solo necesitamos saber que cumple el contrato `Future`.

## Send

`Send` significa que el ownership de un valor puede moverse entre threads de forma segura.

En un backend async, el runtime puede mover tasks entre threads. Por eso muchas veces pedimos que los futures o servicios sean `Send`.

`Send` no crea threads. No ejecuta nada en paralelo por si solo. Solo autoriza al compilador a mover ese valor entre threads si el runtime lo necesita.

## Sync

`Sync` significa que es seguro compartir referencias a un valor entre threads.

La definicion practica es:

```text
T: Sync
```

significa que:

```text
&T: Send
```

O sea, una referencia compartida a `T` puede enviarse a otro thread de forma segura.

Cuando escribimos:

```rust
pub trait IUserUseCase: Send + Sync
```

decimos que cualquier implementacion de `IUserUseCase` debe poder moverse entre threads y tambien compartirse por referencia entre threads.

Esto es importante si luego usamos algo como:

```rust
Arc<dyn IUserUseCase>
```

`Arc` permite compartir ownership del mismo servicio entre muchas requests. Para que sea seguro en un servidor concurrente, el contenido debe cumplir `Send + Sync`.

## Thread

Un thread es un hilo de ejecucion del sistema operativo.

Puede ejecutar una funcion y terminar, esperar IO, dormir, bloquearse o correr un loop si el programa lo escribe. Un thread no es automaticamente un bucle infinito.

## Task

Una task es una unidad de trabajo async manejada por el runtime, por ejemplo Tokio.

Una task no es necesariamente un thread. Suele ser mas liviana y el runtime decide cuando pausarla, continuarla o moverla.

```text
async = permite pausar/continuar trabajo sin bloquear un thread.
Send = permite mover entre threads.
Sync = permite compartir referencias entre threads.
task = unidad de trabajo async que el runtime agenda.
thread = hilo real del sistema operativo.
```

## Bound

Un bound es una restriccion de tipo.

```rust
T: Send
```

significa: `T` puede ser cualquier tipo, pero debe cumplir `Send`.

Cuando vemos:

```rust
Send + Sync + 'static
```

el `+` significa "y tambien". No es suma. Es un AND de restricciones.

## Lifetime 'a

`'a` representa un tiempo de vida.

En:

```rust
Pin<Box<dyn Future<Output = T> + Send + 'a>>
```

`'a` significa que el future puede contener referencias que viven al menos `'a`.

No es una direccion de memoria. Es una regla de duracion para que Rust sepa hasta cuando esas referencias son validas.

## Resumen

`IUserUseCase` es un puerto de entrada. Define lo que la aplicacion puede hacer con usuarios.

`async_trait` permite que ese puerto tenga metodos async sin escribir manualmente tipos complejos de `Future`.

`Send + Sync` prepara el contrato para un backend concurrente, donde varias requests pueden compartir servicios y el runtime puede mover trabajo entre threads.

`ports/in` define lo que entra a la aplicacion.

`ports/out` define lo que la aplicacion necesita del exterior.
