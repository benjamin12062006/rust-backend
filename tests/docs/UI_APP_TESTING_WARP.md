# Tests de aplicaciones UI: referencia de Warp

Fecha: 2026-09-15.

Referencia para estudiar la organización de tests de una aplicación UI en Rust.
La respuesta siguiente se conserva tal cual, incluida su adaptación ilustrativa
a un backend. Esa adaptación no describe el árbol real de Warp.
Para el modelo general por crate, consulta [TEST_ARCHITECTURE_PLAN.md](TEST_ARCHITECTURE_PLAN.md).

---

Sí: **podemos seguir convenciones de Rust y prácticas verificables de Warp**. Revisé su repositorio: combina tests cerca del código con una suite separada para probar la aplicación integrada.

Aclaro también mi propuesta anterior: el árbol que te di era una adaptación para tu proyecto, no un estándar obligatorio.

**Así los organiza Warp**

Su guía establece que los tests unitarios se escriban en archivos separados llamados `nombre_tests.rs` o `mod_test.rs`, junto al módulo que prueban. Ese módulo los incorpora así:

```rust
#[cfg(test)]
#[path = "nombre_tests.rs"]
mod tests;
```

Aunque estén en otro archivo, siguen siendo tests unitarios internos del módulo. Esta es una convención explícita de [la guía de ingeniería de Warp](https://github.com/warpdotdev/warp/blob/master/AGENTS.md).

Para las pruebas integradas de su interfaz gráfica tienen un crate específico. La estructura publicada incluye:

```text
crates/
└── integration/
    ├── Cargo.toml
    ├── src/                 # Infraestructura para ejecutar las pruebas
    ├── assets/
    └── tests/
        ├── integration.rs  # Entrada de la suite
        ├── integration/    # Escenarios organizados en módulos
        ├── common/         # Utilidades compartidas
        ├── data/           # Datos de prueba
        └── INTEGRATION_TESTING.md
```

Puedes verlo directamente en [sus tests de integración](https://github.com/warpdotdev/warp/tree/master/crates/integration/tests). Ese framework atiende necesidades de una aplicación gráfica; nuestro backend necesita principalmente HTTP y PostgreSQL.

Además, documentan ejecución con `cargo nextest`, doc tests y comprobaciones de formato y Clippy antes de entregar cambios. La organización de carpetas es solo una parte de su disciplina de testing. [Guía de Warp](https://github.com/warpdotdev/warp/blob/master/AGENTS.md).

**Cómo aplicaría esas convenciones a tu backend**

Como estructura objetivo, recomiendo **que cada crate sea dueño de sus pruebas y dejar en la raíz los recorridos que conectan todo el backend**:

```text
backend/
├── Cargo.toml
│
├── crates/
│   ├── backend_domain/
│   │   ├── Cargo.toml
│   │   └── src/user/
│   │       ├── user_model.rs
│   │       └── user_model_tests.rs
│   │
│   ├── backend_application/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── ...          # Código y tests unitarios de módulos
│   │   └── tests/
│   │       ├── user_use_cases.rs
│   │       ├── user_service.rs
│   │       └── support/
│   │           ├── mod.rs
│   │           ├── builders.rs
│   │           └── fake_user_repository.rs
│   │
│   ├── backend_http/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── ...          # Código y tests locales de DTOs
│   │   └── tests/
│   │       ├── user_routes.rs
│   │       ├── middleware.rs
│   │       └── support/
│   │           ├── mod.rs
│   │           ├── builders.rs
│   │           └── fake_user_use_case.rs
│   │
│   └── backend_infrastructure/
│       ├── Cargo.toml
│       ├── src/
│       └── tests/
│           ├── postgres_user_repository.rs
│           └── support/
│               ├── mod.rs
│               └── database.rs
│
└── tests/
    ├── user_e2e.rs          # Recorrido completo por HTTP y PostgreSQL
    ├── support/
    │   ├── mod.rs
    │   ├── database.rs
    │   └── server.rs
    └── docs/               # Tus guías actuales
```

Es un esquema propuesto; omite carpetas de producción para mostrar únicamente dónde pertenecen las pruebas.

La regla para decidir dónde poner un archivo sería:

| Qué prueba | Dónde pertenece |
|---|---|
| Una regla o función interna | Junto al módulo, mediante `#[cfg(test)]` |
| La API pública de un crate | En `tests/` de ese crate |
| El flujo completo del backend | En `tests/` del paquete raíz |
| Preparación reutilizable | En `tests/support/` de la suite que la utiliza |

Esta distinción entre tests internos, pruebas externas de la API pública y módulos compartidos está respaldada por [las convenciones oficiales de Cargo](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests).

**Seguimos con la misma arquitectura hexagonal.** `FakeUserRepository` continuaría ayudando a probar application y `FakeUserUseCase` continuaría ayudando a probar HTTP. Cambiaría su ubicación para que cada dependencia de prueba quede cerca de quienes la necesitan.

Tu estructura actual centralizada también es válida. Para acercarnos a esta organización, moveríamos las suites gradualmente, ajustando sus dependencias de desarrollo. Separaríamos los dos fakes por su consumidor y conservaríamos los tests de cada comportamiento sin duplicarlos dentro y fuera del crate.

