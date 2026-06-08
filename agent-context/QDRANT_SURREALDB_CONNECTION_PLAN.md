# Plan Para Conectar Qdrant Cloud Y SurrealDB Cloud

Este documento planifica como conectar Qdrant y SurrealDB al backend actual sin romper la arquitectura hexagonal que ya tenemos.

Fecha de investigacion: 2026-04-24.

## Decision Principal

Mantendremos Postgres como base transaccional principal con SQLx.

Qdrant y SurrealDB no deben entrar por SQLx:

```text
Postgres  -> sqlx
Qdrant    -> qdrant-client
SurrealDB -> surrealdb SDK
```

SQLx sirve para bases SQL soportadas por SQLx, como Postgres, MySQL/MariaDB y SQLite. Qdrant es una base vectorial y SurrealDB usa su propio motor/SDK con SurrealQL, HTTP/WebSocket y modos embebidos. Aunque SurrealDB tenga sintaxis SQL-like, no se conecta con SQLx.

## Cloud O Contenedores

### Recomendacion Para Este Proyecto

Usar nube al inicio:

```text
Qdrant Cloud
SurrealDB Cloud
```

Motivos:

```text
- menos contenedores locales;
- menos espacio en disco;
- menos configuracion de volumenes;
- mas facil probar desde cualquier maquina;
- encaja con nuestro backend como cliente externo.
```

### Cuando Usar Contenedores

Usaria contenedores solo para:

```text
- desarrollo offline;
- pruebas locales de integracion;
- reproducir bugs sin tocar cloud;
- CI si queremos levantar servicios temporales.
```

Qdrant oficialmente puede correr local con Docker exponiendo:

```text
REST: 6333
gRPC: 6334
Web UI: 6333/dashboard
```

SurrealDB tambien puede correr con Docker exponiendo `8000`.

## Fuentes Oficiales Consultadas

- Qdrant local quickstart: https://qdrant.tech/documentation/quick-start/
- Qdrant Semantic Search 101 / Cloud: https://qdrant.tech/documentation/tutorials-basics/search-beginners/
- Qdrant SDKs oficiales: https://qdrant.tech/documentation/interfaces/
- SurrealDB Rust SDK quick start: https://surrealdb.com/docs/languages/rust/start
- SurrealDB Cloud via SDK: https://surrealdb.com/docs/build/deployment/surrealdb-cloud/connecting/via-sdk
- SurrealDB Rust `new`: https://surrealdb.com/docs/languages/rust/methods/new
- SurrealDB Rust `connect`: https://surrealdb.com/docs/languages/rust/methods/connect
- SurrealDB Rust embedding: https://surrealdb.com/docs/languages/rust/embedding
- SurrealDB Docker: https://surrealdb.com/docs/surrealdb/installation/running/docker

## Variables De Entorno Propuestas

Manteneremos `DATABASE_URL` para Postgres.

Agregar:

```env
# Qdrant Cloud
QDRANT_URL=https://xxxx.cloud.qdrant.io
QDRANT_API_KEY=...
QDRANT_COLLECTION=backend_documents
QDRANT_VECTOR_SIZE=1536
QDRANT_DISTANCE=cosine

# SurrealDB Cloud
SURREALDB_URL=wss://xxxx.surrealdb.cloud
SURREALDB_NAMESPACE=backend
SURREALDB_DATABASE=main
SURREALDB_USERNAME=...
SURREALDB_PASSWORD=...
```

Notas:

```text
QDRANT_VECTOR_SIZE depende del modelo de embeddings.
Si usamos OpenAI text-embedding-3-small puede ser 1536.
Si usamos otro modelo, debe coincidir con su dimension.
```

## Dependencias Cargo Planeadas

Agregar cuando pasemos de plan a implementacion:

```toml
qdrant-client = "1"
surrealdb = { version = "3", features = ["protocol-ws"] }
```

Nuestro proyecto ya tiene:

```toml
tokio = { version = "1", features = ["rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
```

Si queremos tests embebidos con SurrealDB en memoria:

```toml
surrealdb = { version = "3", features = ["protocol-ws", "kv-mem"] }
```

Pero para conectar solo a SurrealDB Cloud, `protocol-ws` es la pieza importante.

## Estructura De Carpetas Propuesta

Sin tocar todavia `features/user`.

```text
src/shared/
  configuration/
    configuration.rs
    loader.rs

  infrastructure/
    database/        # Postgres con sqlx
    qdrant/
      mod.rs
      client.rs
      collection.rs
    surrealdb/
      mod.rs
      client.rs
      schema.rs

  bootstrap/
    app.rs
```

### Por Que En `shared/infrastructure`

Qdrant y SurrealDB son recursos transversales.

No pertenecen a `user` todavia porque todavia no hay caso de uso concreto que diga:

```text
user necesita buscar vectores
user necesita guardar grafo/documentos
```

Primero se conectan como clientes de infraestructura compartida. Luego, cuando exista un caso de uso real, se crean ports concretos en el feature correspondiente.

## Tipos De Configuracion Propuestos

En `shared/configuration/configuration.rs`:

```rust
pub struct QdrantConfiguration {
    pub url: String,
    pub api_key: String,
    pub collection: String,
    pub vector_size: u64,
    pub distance: String,
}

pub struct SurrealDbConfiguration {
    pub url: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

pub struct Configuration {
    pub database: DatabaseConfiguration,
    pub server: ServerConfiguration,
    pub qdrant: Option<QdrantConfiguration>,
    pub surrealdb: Option<SurrealDbConfiguration>,
}
```

Recomendacion:

```text
usar Option al principio
para que el backend siga arrancando aunque aun no configuremos Qdrant/SurrealDB.
```

Cuando ya sean obligatorios, cambiamos `Option<T>` por `T`.

## Qdrant: Conexion Planeada

Fuente oficial Qdrant muestra el cliente Rust asi:

```rust
use qdrant_client::Qdrant;

let client = Qdrant::from_url(QDRANT_URL)
    .api_key(QDRANT_API_KEY)
    .build()?;
```

Para local Docker, el Rust client usa gRPC:

```rust
let client = Qdrant::from_url("http://localhost:6334").build()?;
```

### Archivo Propuesto

`src/shared/infrastructure/qdrant/client.rs`

```rust
use qdrant_client::Qdrant;

use crate::shared::configuration::configuration::QdrantConfiguration;
use crate::shared::common::error::Result;

pub type QdrantClient = Qdrant;

pub fn create_qdrant_client(config: &QdrantConfiguration) -> Result<QdrantClient> {
    Qdrant::from_url(&config.url)
        .api_key(&config.api_key)
        .build()
        .map_err(|err| /* convertir a CustomError */)
}
```

### Setup De Coleccion

Qdrant no usa SQL migrations.

Su equivalente inicial es:

```text
ensure collection exists
```

Archivo:

```text
src/shared/infrastructure/qdrant/collection.rs
```

Responsabilidad:

```text
- revisar si la coleccion existe;
- crearla si falta;
- usar vector_size y distance desde config.
```

Ejemplo oficial de creacion:

```rust
use qdrant_client::qdrant::{CreateCollectionBuilder, VectorParamsBuilder};

client
    .create_collection(
        CreateCollectionBuilder::new(collection_name)
            .vectors_config(VectorParamsBuilder::new(384, Distance::Cosine)),
    )
    .await?;
```

## SurrealDB: Conexion Planeada

SurrealDB Cloud via SDK usa WebSocket seguro:

```rust
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;

let db = any::connect("wss://<INSTANCE_ENDPOINT>").await?;
db.use_ns("DEMO namespace").use_db("DEMO database").await?;
db.signin(Root {
    username: "",
    password: "",
}).await?;
```

Para local:

```rust
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
```

### Archivo Propuesto

`src/shared/infrastructure/surrealdb/client.rs`

```rust
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;

use crate::shared::configuration::configuration::SurrealDbConfiguration;
use crate::shared::common::error::Result;

pub type SurrealDbClient = Surreal<Any>;

pub async fn create_surrealdb_client(
    config: &SurrealDbConfiguration,
) -> Result<SurrealDbClient> {
    let db = any::connect(&config.url)
        .await
        .map_err(|err| /* convertir a CustomError */)?;

    db.use_ns(&config.namespace)
        .use_db(&config.database)
        .await
        .map_err(|err| /* convertir a CustomError */)?;

    db.signin(Root {
        username: &config.username,
        password: &config.password,
    })
    .await
    .map_err(|err| /* convertir a CustomError */)?;

    Ok(db)
}
```

Nota de implementacion:

```text
validar el tipo exacto Surreal<Any> contra la version real del crate
cuando agreguemos la dependencia.
```

## SurrealDB: Schema O Migraciones

SurrealDB no usa SQLx migrations.

Para mantener schema inicial podemos crear una funcion idempotente:

```text
ensure_surrealdb_schema()
```

Archivo:

```text
src/shared/infrastructure/surrealdb/schema.rs
```

Ejemplo conceptual:

```rust
pub async fn ensure_surrealdb_schema(db: &SurrealDbClient) -> Result<()> {
    db.query("
        DEFINE TABLE IF NOT EXISTS project SCHEMALESS;
    ")
    .await?;

    Ok(())
}
```

Mas adelante, si SurrealDB crece mucho en el proyecto, podemos crear:

```text
surrealdb/surql/
  0001_define_tables.surql
  0002_define_indexes.surql
```

y un runner propio que registre versiones, parecido a migraciones, pero no mezclado con SQLx.

## Bootstrap Propuesto

En `shared/bootstrap/app.rs`, despues de Postgres:

```rust
let qdrant_client = match &config.qdrant {
    Some(qdrant_config) => {
        let client = create_qdrant_client(qdrant_config)?;
        ensure_qdrant_collection(&client, qdrant_config).await?;
        Some(client)
    }
    None => None,
};

let surrealdb_client = match &config.surrealdb {
    Some(surreal_config) => {
        let client = create_surrealdb_client(surreal_config).await?;
        ensure_surrealdb_schema(&client).await?;
        Some(client)
    }
    None => None,
};
```

No los meteria todavia en `Services` HTTP si ningun controller los necesita.

Primero:

```text
conectar y validar en bootstrap
```

Luego, cuando exista un caso de uso:

```text
crear ports/out concretos
inyectar adapters concretos
usar desde application
```

## Como Encaja Con Hexagonal

### No Hacer

```text
controller -> Qdrant directo
controller -> SurrealDB directo
use_case -> qdrant_client directo
use_case -> surrealdb SDK directo
```

### Hacer

```text
application -> port/out -> adapter Qdrant
application -> port/out -> adapter SurrealDB
```

Ejemplos futuros:

```text
features/search/ports/out/vector_search_repository.rs
features/search/infrastructure/qdrant_vector_search_repository.rs

features/knowledge/ports/out/knowledge_graph_repository.rs
features/knowledge/infrastructure/surrealdb_knowledge_graph_repository.rs
```

## Tests Propuestos

### Sin Nube

```text
unit tests con fake ports;
tests de config parser;
tests de bootstrap opcional sin env.
```

### Con Nube

No correr por defecto.

Usar flags:

```env
RUN_QDRANT_TESTS=true
RUN_SURREALDB_TESTS=true
```

Tests opt-in:

```text
qdrant_connection_is_opt_in
surrealdb_connection_is_opt_in
```

Esto sigue el mismo pensamiento que ya usamos con:

```env
RUN_DB_TESTS=true
```

## CLI Necesaria

### Qdrant

No necesitamos CLI para conectar desde Rust.

Opciones:

```text
Cloud: dashboard + API key + endpoint.
Local: Docker.
Codigo: qdrant-client.
```

### SurrealDB

No necesitamos CLI si usamos Cloud y SDK.

Pero para local conviene tener:

```text
surreal start
surreal sql
```

O Docker:

```bash
docker run --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start --user root --pass secret
```

## Contenedores De Referencia

### Qdrant Local

```bash
docker pull qdrant/qdrant
docker run -p 6333:6333 -p 6334:6334 \
  -v "$(pwd)/qdrant_storage:/qdrant/storage:z" \
  qdrant/qdrant
```

Rust local:

```rust
let client = Qdrant::from_url("http://localhost:6334").build()?;
```

### SurrealDB Local

Servidor en memoria:

```bash
surreal start --user root --pass secret
```

Docker simple:

```bash
docker run --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start --user root --pass secret
```

Docker persistente:

```bash
mkdir mydata
docker run --rm --pull always -p 8000:8000 \
  --user $(id -u) \
  -v $(pwd)/mydata:/mydata \
  surrealdb/surrealdb:latest start --user root --pass secret rocksdb:/mydata/mydatabase.db
```

## Orden De Implementacion Recomendado

1. Agregar config structs para Qdrant y SurrealDB.
2. Agregar env vars al loader como `Option`.
3. Agregar errores de infraestructura para Qdrant/SurrealDB en `CustomError` o un error especifico compartido.
4. Crear `shared/infrastructure/qdrant`.
5. Crear `shared/infrastructure/surrealdb`.
6. Conectar ambos en `bootstrap/app.rs` de forma opcional.
7. Crear tests opt-in con `RUN_QDRANT_TESTS=true` y `RUN_SURREALDB_TESTS=true`.
8. Solo despues crear ports/out por feature cuando exista un caso de uso real.

## Respuesta Conceptual Corta

```text
Qdrant y SurrealDB son compatibles con Rust, pero no con SQLx.
Se conectan con sus SDKs oficiales.
Cloud es buena opcion inicial para ahorrar espacio y configuracion local.
Docker sigue siendo util para desarrollo offline y pruebas opt-in.
La logica debe vivir en shared/infrastructure al principio.
Cuando haya negocio real, se expone por ports/out dentro de cada feature.
```
