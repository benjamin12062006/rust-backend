# Recorrido de `POST /v1/users` en el test

Este documento sigue el test `post_users_creates_user_through_inbound_port` desde la preparación hasta las comprobaciones finales. Se ejecutan las rutas y el controlador reales, con `FakeUserUseCase` como sustituto de application.

## Recorrido función por función

### 1. `fake()` prepara la dependencia del test

En `user_http_tests.rs`:

```rust
#[fixture]
fn fake() -> FakeUserUseCase {
    FakeUserUseCase::new(builders::user_output())
}
```

Primero, `builders::user_output()` fabrica un `UserOutput` de ejemplo. Después, `FakeUserUseCase::new(...)` lo guarda como respuesta preparada para crear, listar y obtener usuarios.

`#[fixture]` permite que `rstest` proporcione ese fake al test que lo recibe como argumento.

### 2. El test entrega su trabajo asíncrono a `block_on`

```rust
fn post_users_creates_user_through_inbound_port(fake: FakeUserUseCase) {
    block_on(async {
        // Preparar rutas, simular petición y comprobar resultados.
    });
}
```

`async { ... }` crea un future que representa todo ese trabajo.

El helper de `support/runtime.rs` crea Tokio y ejecuta ese future:

```rust
tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("test runtime should be created")
    .block_on(future)
```

Desde aquí empieza a ejecutarse el contenido del bloque `async`.

### 3. `routes_for(fake.clone())` prepara las rutas

Dentro del bloque:

```rust
let routes = routes_for(fake.clone());
```

El test conserva `fake` para inspeccionarlo después. El clon comparte su estado mediante el `Arc` interior.

La función llamada es:

```rust
fn routes_for(fake: FakeUserUseCase) -> /* filtro de Warp */ {
    http_server::routes(Services {
        user_service: fake.into_arc(),
    })
    .recover(middleware::error::handle_rejection)
}
```

Aquí ocurren tres cosas:

- `into_arc()` entrega el fake como `Arc<dyn IUserUseCase>`.
- `http_server::routes(...)` construye las reglas de las rutas.
- `.recover(...)` conecta el manejador que responderá si ocurre un rechazo.

Preparar una ruta todavía no ejecuta su controlador.

### 4. `http_server::routes()` establece quién atenderá el POST

```rust
let create_user = path_users_prefix()
    .and(path::end())
    .and(warp::post())
    .and(middleware::service::with_service(
        services.user_service.clone(),
    ))
    .and(warp::body::json())
    .and_then(user_controller::create_user);
```

Esta regla significa:

```text
Si la dirección es exactamente /v1/users
y el método es POST:
    proporcionar el servicio,
    interpretar el cuerpo JSON,
    llamar al controlador create_user.
```

`path_users_prefix()` define el prefijo `/v1/users`. `path::end()` exige que no haya más segmentos.

`with_service(...)` entrega un clon del servicio al controlador. Aquí clona el `Arc`, compartiendo el mismo objeto.

### 5. El test prepara la petición y la ejecuta contra las rutas

```rust
let response = warp::test::request()
    .method("POST")
    .path("/v1/users")
    .header("content-type", "application/json")
    .json(&valid_create_user_payload())
    .reply(&routes)
    .await;
```

`valid_create_user_payload()` prepara el JSON con nombre, email, usuario y contraseña.

Los métodos anteriores a `.reply(...)` configuran la petición. `.reply(&routes).await` hace que Warp la procese usando esas rutas y obtiene la respuesta. Esto sucede dentro del test, sin abrir un servidor de red.

### 6. Warp llama a `user_controller::create_user()`

El filtro JSON convierte el cuerpo en `CreateUserDto`. El controlador recibe ese DTO y el servicio:

```rust
pub async fn create_user(
    user_service: Arc<dyn IUserUseCase>,
    body: CreateUserDto,
) -> Result<impl Reply, Rejection> {
    let input = CreateUserInput::try_from(body)
        .map_err(reject::custom)?;

    match user_service.create_user(input).await {
        Ok(created_user) => {
            Ok(json(&UserResponseDto::from(created_user)))
        }
        Err(e) => Err(reject::custom(e)),
    }
}
```

Primero, `CreateUserInput::try_from(body)` comprueba campos vacíos y el email. Si son válidos, construye el `CreateUserInput`.

Después, `user_service.create_user(input).await` llama al puerto de entrada. En este test, `user_service` contiene el fake, por lo que se ejecuta su implementación.

### 7. `FakeUserUseCase::create_user()` registra y responde

```rust
async fn create_user(&self, input: CreateUserInput) -> Result<UserOutput> {
    let mut state = self.state.lock().expect("fake use case lock");

    state.created_inputs.push(input);
    state.create_result.clone()
}
```

El fake:

1. Obtiene el bloqueo del `Mutex`.
2. Guarda la entrada recibida.
3. Copia la respuesta que preparaste en `new(...)`.
4. Libera el bloqueo al salir.

Aquí no llama a un repositorio. Devuelve, por ejemplo, `Ok(usuario_preparado)`.

### 8. El controlador convierte el resultado en una respuesta HTTP

La ejecución continúa después de `.await`:

```rust
Ok(created_user) => Ok(json(&UserResponseDto::from(created_user)))
```

`UserResponseDto::from(...)` convierte el `UserOutput` en el DTO HTTP. `json(...)` lo serializa como JSON.

Warp construye la respuesta y termina el `.await` del test. Ahora `response` contiene la respuesta HTTP concreta.

### 9. El test comprueba el resultado y termina

```rust
assert_eq!(response.status(), StatusCode::OK);

let created_inputs = fake.created_inputs();
assert_eq!(created_inputs.len(), 1);
assert_eq!(created_inputs[0].email, "benji@example.com");
```

El test comprueba tanto lo que HTTP devolvió como lo que entregó al fake. `created_inputs()` consulta el estado compartido bajo el `Mutex` y devuelve una copia del historial.

Cuando terminan las comprobaciones, finaliza el bloque `async`, `block_on` retorna y el test termina.

## Diagrama del proceso

```text
TEST: crear usuario por HTTP
|
|  fake() -> FakeUserUseCase::new(usuario)
|           Guarda una respuesta preparada.
|
v
block_on(async { ... })
|
|  Crea Tokio y empieza a ejecutar el bloque.
|
|  +---------------------------------------------------+
|  | BLOQUE ASINCRONO DEL TEST                          |
|  |                                                   |
|  |  routes_for(fake.clone())                         |
|  |       |                                           |
|  |       +--> Prepara las rutas con el fake.          |
|  |                                                   |
|  |  Peticion: POST /v1/users + JSON                   |
|  |       |                                           |
|  |       v                                           |
|  |  .reply(&routes).await                            |
|  |       |                                           |
|  |       |  El test necesita la respuesta            |
|  |       |  para continuar con las comprobaciones.   |
|  |       v                                           |
|  |  RUTA: coincide POST /v1/users                     |
|  |       |                                           |
|  |       v                                           |
|  |  CONTROLADOR: create_user(servicio, body)          |
|  |       |                                           |
|  |       +--> Valida y convierte el JSON recibido.   |
|  |       |    CreateUserDto -> CreateUserInput        |
|  |       |                                           |
|  |       v                                           |
|  |  servicio.create_user(input).await                |
|  |       |                                           |
|  |       v                                           |
|  |  FAKE: create_user(input)                         |
|  |       |                                           |
|  |       +--> Mutex: obtener acceso al estado.       |
|  |       +--> Guardar input en created_inputs.       |
|  |       +--> Copiar create_result.                  |
|  |       +--> Liberar el bloqueo.                    |
|  |       |                                           |
|  |       |  Devuelve Ok(UserOutput).                 |
|  |       v                                           |
|  |  CONTROLADOR: continua despues del .await         |
|  |       |                                           |
|  |       +--> UserOutput -> UserResponseDto -> JSON   |
|  |       |                                           |
|  |       v                                           |
|  |  WARP: entrega la respuesta HTTP                  |
|  |       |                                           |
|  |       v                                           |
|  |  TEST: continua despues de .reply(...).await       |
|  |       |                                           |
|  |       +--> Comprueba estado HTTP y JSON.          |
|  |       +--> Consulta fake.created_inputs().        |
|  |       +--> Comprueba los datos registrados.       |
|  |                                                   |
|  +---------------------------------------------------+
|
v
El future del bloque termina.
block_on devuelve el control.
El test termina.
```

## Estado compartido del fake

`Arc` mantiene conectado el estado que usan las rutas con el que consulta el test:

```text
TEST conserva un fake ------+
                            |
                            +----> MISMO ESTADO
                            |      protegido por Mutex
RUTAS reciben su clon ------+

Las rutas escriben:     created_inputs.push(input)
El test lee despues:    fake.created_inputs()
```

## Qué ocurre en `.await`

```text
Llamar a una funcion async
            |
            v
      Obtener un Future
            |
            v
    .await lo hace avanzar
            |
            v
   +----------------------+
   | Esta listo el        |
   | resultado?           |
   +----------------------+
          |         |
         SI         NO
          |         |
          v         v
   Continuar    Suspender esta tarea.
   con el       Tokio puede ejecutar
   resultado.   otras tareas listas.
                    |
                    v
                Algo permite avanzar
                (por ejemplo, llegan datos).
                    |
                    v
                Tokio vuelve a hacer
                avanzar la tarea.
```

En este fake normalmente no hay una espera asíncrona real: la respuesta ya está en memoria. Una consulta real a base de datos sí puede suspenderse mientras espera la respuesta. `async` no hace que la consulta sea más rápida ni crea paralelismo automáticamente; permite que el programa aproveche ese tiempo de espera para avanzar otras tareas.
