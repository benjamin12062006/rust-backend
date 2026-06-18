# Bootstrap Mental Map

`backend_bootstrap` no contiene negocio. Su trabajo es componer la aplicacion:
configuracion, recursos tecnicos, servicios y servidor HTTP.

## Arranque

```text
src/main.rs
  -> backend_bootstrap::bootstrap::app::run()
      -> backend_config::configuration::loader::load_configuration()
      -> backend_bootstrap::bootstrap::resources::AppResources::initialize()
      -> backend_bootstrap::bootstrap::services::build_application_services()
      -> backend_http::http_server::Server::new()
      -> Server::run()
```

## Dependencias Por Capa

```text
backend_domain
  modelos y reglas puras

backend_application
  casos de uso y puertos

backend_infrastructure
  adapters concretos: Postgres, Qdrant, SurrealDB, repositorios

backend_http
  rutas, controllers, DTOs HTTP y middleware Warp

backend_config
  carga y tipos de configuracion

backend_bootstrap
  composicion: crea recursos y conecta adapters con puertos
```

## Regla De Imports

Dentro del mismo crate:

```rust
use crate::bootstrap::error::BootstrapError;
```

Entre crates:

```rust
use backend_http::http_server::Server;
use backend_config::configuration::loader::load_configuration;
```

No se usan rutas falsas tipo `shared` o `features` como compatibilidad. Cada
`lib.rs` expone solo modulos reales del crate.
