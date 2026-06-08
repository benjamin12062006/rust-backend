# Testing Layer

Esta carpeta prueba el backend desde fuera del crate, igual que lo haria otro
consumidor de la libreria. La idea principal es validar comportamiento sin
acoplar los tests a una base de datos real.

## Estructura

```text
tests/
  support/
    builders.rs   # Datos de prueba reutilizables.
    fakes.rs      # Implementaciones fake de ports.
    runtime.rs    # Runtime Tokio para tests async sin macros.

  user_application_tests.rs       # Casos de uso con FakeUserRepository.
  user_http_tests.rs              # Rutas HTTP con FakeUserUseCase.
  bootstrap_middleware_tests.rs   # Error middleware y service injection.
  database_integration_tests.rs   # DB real solo con RUN_DB_TESTS=true.
```

## Regla De Arquitectura

La mayoria de tests usa fakes detras de los puertos:

```text
application -> IUserRepository -> FakeUserRepository
http         -> IUserUseCase    -> FakeUserUseCase
```

Asi demostramos que `user` funciona por contrato sin depender de Postgres.

## Tests Con DB Real

Por defecto no corren contra DB real. Para habilitarlos:

```bash
RUN_DB_TESTS=true cargo test -p backend database_connection_ping_is_opt_in
```

Ese test requiere `DATABASE_URL` configurado.
