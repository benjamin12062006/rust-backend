# Application Layer

Este documento resume la idea general de la capa `application` en este backend.

## Rol De Application

`application` es la capa de casos de uso. Su trabajo es orquestar una accion del negocio de punta a punta sin depender directamente de HTTP, SQL, Redis, Firebase, Clerk, Supabase u otra tecnologia concreta.

La capa `application` no es el dominio puro. Tampoco es infraestructura. Vive en el medio:

```text
ports/in -> application -> ports/out
```

Ejemplo del flujo de usuario:

```text
HTTP handler -> IUserUseCase -> UserService -> IUserRepository -> DB adapter
```

## Diferencia Con Domain

`domain` define los modelos y reglas centrales del negocio.

`application` coordina pasos para cumplir un caso de uso.

Por ejemplo, `domain` puede definir que es un `User`. `application` puede decir:

```text
recibir User
hashear password
pedir al repositorio que lo guarde
devolver User creado
```

## Diferencia Con Infrastructure

`application` no deberia saber si el usuario se guarda en SQL, Supabase, Firebase, Clerk, Redis o cualquier otro proveedor.

Para eso usa contratos:

```text
IUserRepository
```

La implementacion real del repositorio vive fuera de `application`, normalmente en infraestructura.

## Ports In

`ports/in` representa lo que otros usan para pedirle algo a la aplicacion.

Ejemplos:

- HTTP handler.
- CLI.
- App desktop.
- Job o cron.
- Tests.

En este proyecto:

```text
IUserUseCase = contrato de entrada para operaciones de usuario.
```

## Ports Out

`ports/out` representa lo que la aplicacion necesita del exterior.

Ejemplos:

- Base de datos.
- Redis.
- Firebase.
- Clerk.
- Supabase.
- Email.
- Storage.
- APIs externas.

En este proyecto:

```text
IUserRepository = contrato de salida para leer/guardar usuarios.
```

## UserService

`UserService` vive en `application/user` porque implementa el caso de uso de usuario.

Su posicion conceptual es:

```text
IUserUseCase (in) -> UserService -> IUserRepository (out)
```

Esto significa:

```text
IUserUseCase = lo que UserService ofrece hacia afuera.
IUserRepository = lo que UserService necesita para guardar/leer usuarios.
```

## Constructor

El constructor de `UserService` solo necesita recibir un repositorio:

```rust
impl<R> UserService<R>
where
    R: IUserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}
```

Aqui no es necesario pedir `Send + Sync` todavia. Para construir el struct basta con que `R` sea un repositorio.

`Send + Sync` se vuelve importante cuando `UserService<R>` implementa el puerto de entrada `IUserUseCase`, porque ese contrato esta pensado para un backend concurrente.

## Send And Sync En Application

Cuando implementamos un caso de uso async para backend web, solemos pedir:

```rust
R: IUserRepository + Send + Sync
```

`Send` permite mover ownership entre threads.

`Sync` permite compartir referencias entre threads de forma segura.

Esto no se "ejecuta" en runtime como una funcion. Es una garantia de compilacion. Rust revisa antes de correr que las implementaciones sean seguras para un entorno concurrente.

## Async En Application

Los metodos de casos de uso suelen ser async porque pueden llamar repositorios, DBs, APIs externas o servicios remotos.

`async` permite pausar y continuar trabajo sin bloquear un thread.

`.await` espera a que una operacion async termine.

`?` propaga errores de forma controlada si la funcion devuelve `Result<T>`.

## Regla Practica

`application` debe coordinar el caso de uso, no guardar datos directamente ni responder HTTP directamente.

Si `application` necesita salir al mundo exterior, lo hace por `ports/out`.

Si el mundo exterior quiere pedirle algo a la aplicacion, entra por `ports/in`.
