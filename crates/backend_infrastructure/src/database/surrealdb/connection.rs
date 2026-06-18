//! Construccion del cliente SurrealDB.
//!
//! Usamos el cliente remoto por WebSocket, alineado con la documentacion
//! oficial de SurrealDB para conexiones locales/remotas.

use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

use backend_common::common::error::{CustomError, Result};
use backend_config::configuration::database::SurrealDbConfiguration;

pub type SurrealDbClient = Surreal<Client>;

/// Crea una conexion remota a SurrealDB, autentica root y selecciona ns/db.
pub async fn create_connection(config: &SurrealDbConfiguration) -> Result<SurrealDbClient> {
    let endpoint = config
        .url
        .trim_start_matches("ws://")
        .trim_start_matches("wss://")
        .trim_start_matches("http://")
        .trim_start_matches("https://");

    let db = Surreal::new::<Ws>(endpoint)
        .await
        .map_err(|error| CustomError::DatabaseError(error.to_string()))?;
    db.signin(Root {
        username: config.username.clone(),
        password: config.password.clone(),
    })
    .await
    .map_err(|error| CustomError::DatabaseError(error.to_string()))?;

    db.use_ns(config.namespace.as_str())
        .use_db(config.database.as_str())
        .await
        .map_err(|error| CustomError::DatabaseError(error.to_string()))?;
    Ok(db)
}

// pub type SurrealDbClient = Surreal<Client>;

// /// Crea una conexion remota a SurrealDB, autentica root y selecciona ns/db.
// pub async fn create_connection(config: &SurrealDbConfiguration) -> Result<SurrealDbClient> {
//     let endpoint = config
//         .url
//         .trim_start_matches("ws://")
//         .trim_start_matches("wss://")
//         .trim_start_matches("http://")
//         .trim_start_matches("https://");

//     let db = Surreal::new::<Ws>(endpoint).await?;
//     db.signin(Root {
//         username: config.username.clone(),
//         password: config.password.clone(),
//     })
//     .await?;
//     db.use_ns(config.namespace.as_str())
//         .use_db(config.database.as_str())
//         .await?;

//     Ok(db)
// }
