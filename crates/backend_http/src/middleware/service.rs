use std::convert::Infallible;

use warp::Filter;

/// Crea un filtro de Warp que inyecta una copia de `service` en cada request.
///
/// Tipo retornado:
/// - `Extract = (T,)`: extrae el servicio para el siguiente filtro/handler.
/// - `Error = Infallible`: este filtro no falla por si mismo.
pub fn with_service<T>(service: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone
where
    T: Clone + Send,
{
    warp::any().map(move || service.clone())
}

/*
Notas de estudio

Este archivo existe para resolver un problema comun en handlers HTTP:
necesitamos pasar dependencias, por ejemplo `Arc<dyn IUserUseCase>`, a los
controllers sin usar variables globales.

`with_service` recibe una dependencia generica `T` y devuelve un `Filter` de
Warp que, en cada request, extrae una copia de esa dependencia.

La idea es:
- capturar una instancia compartida de servicio cuando se arma la ruta;
- entregar un clon por request al handler que corresponda.

Por que no falla (`Infallible`)

El filtro devuelto por `warp::any().map(...)` no tiene pasos que retornen error
de aplicacion:
- `warp::any()` siempre matchea;
- `map` solo transforma datos y aqui devuelve `service.clone()`.

Por eso su tipo de error es `Infallible`.

Conceptos de Rust usados aqui

1. `impl Filter<...> + Clone` en retorno:
   la funcion retorna "algun tipo concreto" que implementa `Filter`, pero no
   expone el tipo interno exacto de Warp.

2. `Extract = (T,)`:
   Warp maneja extracciones como tuplas. `(T,)` es una tupla de un elemento que
   representa "este filtro entrega un valor de tipo T".

3. `T: Clone + Send`:
   `Clone` permite entregar una copia por request. `Send` permite que la
   dependencia pueda moverse de forma segura entre hilos/tareas del runtime
   async.

4. `move` en closure:
   `move` captura `service` por ownership dentro de la closure. Asi el filtro
   puede vivir despues de que termine la funcion `with_service`.

Arc y concurrencia

En practica, `T` suele ser `Arc<dyn ...>`. Clonar `Arc` no duplica toda la
implementacion del servicio; solo incrementa su contador de referencias, por lo
que es barato y seguro para concurrencia.

Flujo de ejecucion

1. Arranque: se construye el servicio, por ejemplo `Arc<dyn IUserUseCase>`.
2. Wiring de rutas: `with_service(service.clone())` crea un filtro que guarda
   el servicio capturado con `move`.
3. Request: el filtro ejecuta `service.clone()` y extrae `(T,)`.
4. Handler: Warp inyecta ese `T` como argumento del controller.

Ejemplo:

```rust,ignore
let list_users = warp::get()
    .and(path_users_prefix())
    .and(path::end())
    .and(middleware::service::with_service(self.services.user_service.clone()))
    .and_then(user_controller::list_users);
```

En ese flujo, `list_users` recibe `Arc<dyn IUserUseCase>` como primer parametro
gracias al `Extract = (T,)` de este archivo.
*/
