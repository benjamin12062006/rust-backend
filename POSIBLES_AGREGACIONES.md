# Posibles Agregados DDD para Proyecto Grande

Este documento lista directorios y capas que el ejemplo `rust-api-ddd` todavia no tiene completos, pero que podrian agregarse si el proyecto crece y queremos una arquitectura DDD/hexagonal mas fuerte.

No significa que haya que crear todo ahora. Es un mapa de crecimiento.

---

## 1) Domain mas expresivo

```text
src/domain/
  user/
    entities/
    value_objects/
    errors/
    events/
    services/
```

Para que serviria:

```text
entities
  Modelos con identidad propia. Ejemplo: User, Product, Order.

value_objects
  Tipos validos por construccion. Ejemplo: Email, Username, PasswordHash, Money.

errors
  Errores puros del dominio. Ejemplo: InvalidEmail, EmptyUsername.

events
  Eventos de negocio. Ejemplo: UserCreated, UserPasswordChanged.

services
  Logica de dominio que no pertenece claramente a una sola entidad.
```

---

## 2) Application mas completa

```text
src/application/
  user/
    dto/
      input/
      output/
    use_cases/
    policies/
    errors/
    services/
```

Para que serviria:

```text
dto/input
  Datos que entran a casos de uso. Ejemplo: CreateUserInput, LoginInput.

dto/output
  Datos que salen de casos de uso. Ejemplo: UserOutput, SessionOutput.

use_cases
  Acciones concretas. Ejemplo: create_user, list_users, get_user.

policies
  Reglas de aplicacion. Ejemplo: password_policy, session_policy.

errors
  Errores propios de application si common/error.rs crece demasiado.

services
  Fachadas u orquestadores que implementan ports/in.
```

---

## 3) Ports mas escalables

```text
src/ports/
  in/
    user_use_case.rs
    auth_use_case.rs
  out/
    user/
      user_repository.rs
      records/
    auth/
    cache/
    events/
    clock/
    id_generator/
    email/
    storage/
```

Para que serviria:

```text
ports/in
  Contratos de entrada a la aplicacion.

ports/out/user/records
  Records de frontera para repositorios: CreateUserRecord, StoredUserRecord.

ports/out/auth
  Contratos para Clerk, Firebase Auth, Supabase Auth u otros proveedores.

ports/out/cache
  Contratos para Redis, memoria local o cache externo.

ports/out/events
  Contratos para publicar eventos hacia Kafka, RabbitMQ, NATS, SQS, etc.

ports/out/clock
  Contrato para controlar la fecha/hora en tests sin usar Utc::now() directamente.

ports/out/id_generator
  Contrato para generar UUIDs/IDs sin acoplar application a una implementacion concreta.

ports/out/email
  Contrato para envio de emails.

ports/out/storage
  Contrato para archivos, blobs o buckets.
```

---

## 4) Infrastructure mas amplia

```text
src/infrastructure/
  database/
  repositories/
  auth/
  cache/
  email/
  events/
  storage/
  telemetry/
```

Para que serviria:

```text
database
  Pool, migraciones, conexion y configuracion de base de datos.

repositories
  Implementaciones reales de ports/out repositories.

auth
  Adaptadores concretos para Clerk, Firebase, Supabase Auth, etc.

cache
  Redis, Upstash o cache en memoria.

email
  SMTP, Resend, SendGrid u otro proveedor.

events
  Kafka, RabbitMQ, NATS, SQS u otro bus/cola.

storage
  S3, Blob storage, filesystem, buckets, etc.

telemetry
  tracing, logs, metricas y observabilidad tecnica.
```
structura típica a futuro:

database/mod.rs: re-exporta y define la API interna.
database/config.rs: DatabaseConfig (url, pool_size, timeouts).
database/pool.rs: creación de PgPool/MySqlPool y lifecycle.
database/migrations.rs: correr migraciones al boot.
database/tx.rs: helpers para transacciones.
database/health.rs: ping/healthcheck de DB.
database/errors.rs: errores técnicos de DB mapeables a CustomError.

---

## 5) Presentation mas separada

```text
src/presentation/
  http/
    routes/
    controllers/
    dto/
    middleware/
    errors/
  cli/
  grpc/
  jobs/
```

Para que serviria:

```text
http
  API HTTP: rutas, controllers, request DTOs, response DTOs y errores publicos.

cli
  Entrada por linea de comandos si el sistema necesita comandos internos.

grpc
  Entrada por gRPC si el sistema expone servicios RPC.

jobs
  Trabajos programados, workers o cron jobs que entran a application por ports/in.
```

---

## 6) Bootstrap o Composition

```text
src/bootstrap/
  mod.rs
  app.rs
  dependency_container.rs
```

O tambien:

```text
src/composition/
  mod.rs
  services.rs
  repositories.rs
```

Para que serviria:

```text
Crear DB pool.
Crear adaptadores concretos.
Crear services de application.
Conectar traits con implementaciones.
Crear router/server.
Evitar que main.rs se llene de configuracion y logica de armado.
```

---

## 7) Config

```text
src/config/
  mod.rs
  env.rs
  settings.rs
```

Para que serviria:

```text
Leer y validar DATABASE_URL, SERVER_PORT, tokens, flags y variables de entorno.
```

El ejemplo actual tiene `configuration`, que puede cumplir esta responsabilidad. En un proyecto grande podriamos renombrar o reforzar esta capa.

---

## 8) Observability

```text
src/observability/
  mod.rs
  tracing.rs
  metrics.rs
```

Para que serviria:

```text
Logs estructurados.
Tracing distribuido.
Metricas.
Correlacion de requests.
Diagnostico de errores.
```

En proyectos pequenos puede vivir dentro de `infrastructure/telemetry`.

---

## 9) Testing por capas

```text
tests/
  integration/
  e2e/

src/application/user/tests/
src/domain/user/tests/
```

Para que serviria:

```text
domain tests
  Pruebas puras de reglas de negocio.

application tests
  Pruebas de casos de uso usando repositorios fake/mock.

infrastructure tests
  Pruebas contra DB real o contenedor.

presentation tests
  Pruebas de HTTP/rutas/respuestas publicas.
```

---

## 10) Regla final

La arquitectura grande no se crea toda desde el dia uno.

Se agrega cuando aparece una responsabilidad real:

```text
Si aparece una regla de negocio fuerte -> domain/value_objects o domain/services.
Si aparece una accion grande -> application/use_cases.
Si aparece una dependencia externa -> ports/out + infrastructure.
Si aparece una salida publica distinta -> application/dto/output + presentation/http/dto.
Si main.rs crece demasiado -> bootstrap o composition.
Si common/error.rs crece demasiado -> errores por capa.
```

Objetivo:

```text
Una responsabilidad, una frontera clara, una dependencia en la direccion correcta.
```
