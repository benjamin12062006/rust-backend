# CONTEXTO DDD HEXAGONAL ACTUAL V2

Este archivo resume el estado real actual del backend para que un nuevo chat
entienda rapido donde estamos, que ya funciona y que decisiones de arquitectura
ya tomamos.

## 1. Proyecto actual

Ruta principal:

```text
/Users/benji/Documents/RUST/backend
```

Referencia/laboratorio usado durante el aprendizaje:

```text
/Users/benji/Documents/RUST/aprendisajeRS/rust-api-ddd
```

## 2. Estado general

El backend ya arranca en local y responde por HTTP.

Hoy el sistema hace esto al iniciar:

```text
load_configuration
-> conecta Postgres
-> check Postgres
-> corre migraciones SQLx
-> conecta SurrealDB
-> check SurrealDB
-> crea cliente Qdrant
-> check Qdrant
-> monta rutas HTTP
```

La respuesta `{"message":"Route not found"}` en `/` es normal. Las rutas estan
bajo versionado tipo:

```text
/v1/users
```

## 3. Estructura actual importante

Arquitectura actual feature-first:

```text
src/
  features/
    user/
      domain/
      application/
      ports/
      infrastructure/
      presentation/
  shared/
    common/
    configuration/
    infrastructure/
    bootstrap/
```

Regla mental usada:

```text
presentation -> application -> ports -> infrastructure
```

`bootstrap` es el composition root. Ahi se conectan implementaciones reales.

## 4. Bootstrap actual

Archivo clave:

```text
src/shared/bootstrap/app.rs
```

Responsabilidad:

```text
leer config
crear conexiones
hacer health checks
correr migraciones de Postgres
instanciar repositorios/servicios
levantar servidor HTTP
```

Importante:

- `app.rs` si une infraestructura real, pero no debe tener logica de negocio.
- El feature `user` sigue funcionando hoy sobre Postgres/SQLx.
- SurrealDB y Qdrant ya estan conectados y chequeados, pero todavia no manejan
  casos de uso del feature.

## 5. Configuracion actual

La configuracion vive en:

```text
src/shared/configuration/
```

Ahora `Configuration` tiene agrupacion por bases de datos:

```text
Configuration
  databases: ConfigurationDatabases
    postgres
    surrealdb
    qdrant
  server
```

Esto se hizo para escalar mejor a futuro con mas grupos como:

```text
auth
ai
email
cache
storage
```

## 6. Infraestructura de bases de datos

Separacion actual:

```text
src/shared/infrastructure/database/
  postgres/
    connection.rs
    health.rs
  surrealdb/
    connection.rs
    health.rs
  qdrant/
    connection.rs
    health.rs
```

Idea actual:

- Postgres: pool SQLx + migraciones.
- SurrealDB: cliente remoto por WebSocket + signin + namespace/database.
- Qdrant: builder local/cloud + health check async posterior.

## 7. Variables de entorno importantes

Hoy se esperan al menos estas:

```text
DATABASE_URL
PORT
SURREALDB_URL
SURREALDB_NAMESPACE
SURREALDB_DATABASE
SURREALDB_USERNAME
SURREALDB_PASSWORD
QDRANT_URL
QDRANT_COLLECTION
```

`QDRANT_API_KEY` es opcional porque:

- en Docker local normalmente no hace falta
- en Qdrant Cloud si puede hacer falta

## 8. Docker/local actual

Hay `docker-compose.yaml` para:

```text
qdrant
surrealdb
```

Postgres se esta manejando aparte en otro contenedor del usuario.

Endpoints locales usados:

```text
Qdrant REST   -> http://localhost:6333
Qdrant gRPC   -> http://localhost:6334
SurrealDB     -> ws://localhost:8000
Backend HTTP  -> http://127.0.0.1:8080
```

## 9. Estado del feature user

El feature `user` existe mas como plantilla de arquitectura y aprendizaje que
como decision final de producto.

Importante:

- El usuario del proyecto probablemente usara servicios externos como Clerk o
  Firebase para autenticacion real.
- Aun asi no se elimina el codigo `user` porque sirve para estudiar:
  DTOs, puertos, adapters, servicios, tests y flujo hexagonal.

## 10. Testing actual

Hay capa de tests ya creada. La idea fue:

- tests unitarios por capa
- tests HTTP
- tests de bootstrap/middleware
- test de DB opt-in

El test de DB solo corre si el usuario activa:

```text
RUN_DB_TESTS=true
```

## 11. Comandos utiles

Desde:

```text
/Users/benji/Documents/RUST/backend
```

comandos base:

```bash
cargo run
cargo check
cargo test
```

Aliases de cargo definidos en:

```text
.cargo/config.toml
```

Actuales:

```text
cargo start
cargo c
cargo t
```

`cargo dev` requiere tener instalado `cargo-watch`.

## 12. Decision arquitectonica importante

Postgres no fue eliminado.

La direccion actual es:

- mantener Postgres funcionando
- dejar SurrealDB y Qdrant ya integrados a nivel de conexion/check
- migrar adapters concretos por feature solo cuando haya un caso de uso real

Traduccion mental:

```text
shared/configuration
  sabe leer settings

shared/infrastructure
  sabe crear clientes reales

bootstrap
  sabe conectar todo

features/*
  decide cuando usar cada proveedor en casos de uso concretos
```

## 13. Si un nuevo chat entra

Lo primero que debe asumir:

1. El usuario esta aprendiendo Rust + arquitectura hexagonal paso a paso.
2. Quiere explicaciones claras y muy aterrizadas.
3. El backend ya levanta y responde.
4. La siguiente evolucion natural no es rehacer todo, sino consolidar adapters
   por feature y seguir estudiando la estructura actual.
