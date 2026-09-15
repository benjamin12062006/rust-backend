# Plan general de arquitectura de tests en Rust

Fecha de actualización: 2026-09-15. Estado: modelo de organización para futuros proyectos y reorganizaciones.

## 1. Plan inicial: cada crate es dueño de sus pruebas

Organizar los tests siguiendo los límites del código: pruebas internas junto al
módulo, pruebas de la API pública dentro del crate correspondiente y recorridos
del sistema completo en una suite superior.

Este documento define una estructura reutilizable. Los nombres del árbol son
ilustrativos y no representan el inventario ni las funcionalidades implementadas
en un backend concreto. La documentación del sistema actual permanece en
[la guía de estudio](README.md) y [el recorrido HTTP](HTTP_POST_USER_FLOW.md).

### Estructura objetivo

```text
project/
├── Cargo.toml
├── src/                         # Composición del paquete raíz, si existe
├── crates/
│   ├── domain/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── entity/
│   │           ├── mod.rs
│   │           ├── model.rs
│   │           └── model_tests.rs
│   │
│   ├── application/
│   │   ├── Cargo.toml
│   │   ├── src/                 # Casos de uso y pruebas locales de módulos
│   │   └── tests/
│   │       ├── use_cases.rs
│   │       ├── service.rs
│   │       └── support/
│   │           ├── mod.rs
│   │           ├── builders.rs
│   │           └── fake_repository.rs
│   │
│   ├── http/
│   │   ├── Cargo.toml
│   │   ├── src/                 # Rutas, controladores y tests locales de DTOs
│   │   └── tests/
│   │       ├── routes.rs
│   │       ├── middleware.rs
│   │       └── support/
│   │           ├── mod.rs
│   │           ├── builders.rs
│   │           └── fake_use_case.rs
│   │
│   └── infrastructure/
│       ├── Cargo.toml
│       ├── src/
│       └── tests/
│           ├── postgres_repository.rs
│           └── support/
│               ├── mod.rs
│               └── database.rs
│
└── tests/                       # Recorridos entre crates del paquete raíz
    ├── system.rs
    ├── e2e.rs
    ├── support/
    │   ├── mod.rs
    │   ├── database.rs
    │   └── server.rs
    └── docs/
        ├── README.md
        ├── TEST_ARCHITECTURE_PLAN.md
        └── UI_APP_TESTING_WARP.md
```

En una aplicación UI, el crate de entrada puede ser `ui` y sus pruebas cubrir
eventos, estado de vistas y renderizado. Los nombres `http` y `postgres_repository`
solo aplican cuando existen esas tecnologías.

Los tests raíz requieren un paquete raíz con `[package]` y sus dependencias.
En un workspace virtual que solo tiene `[workspace]`, colocar los recorridos
completos en un miembro dedicado, por ejemplo `crates/integration/tests/`, con su
propio `Cargo.toml`. No crear un crate adicional únicamente para imitar otro proyecto.

### Regla de ubicación

| Qué se comprueba | Ubicación |
| --- | --- |
| Una regla o función interna | Archivo de tests junto al módulo |
| La API pública de un crate | `crates/<crate>/tests/` |
| Conexión de varios crates y recorrido completo | `tests/` del paquete raíz o crate de integración |
| Preparación reutilizable dentro de una suite | `tests/support/mod.rs` y sus submódulos |
| Datos estáticos de entrada o respuestas esperadas | `tests/data/` cuando resulten necesarios |
| Explicaciones y estrategia | `tests/docs/` |

## 2. Convenciones de Rust y práctica observada en Warp

### Tests unitarios en archivos separados

El módulo de producción incorpora su archivo de pruebas:

```rust
// Al final de model.rs:
#[cfg(test)]
#[path = "model_tests.rs"]
mod tests;
```

El archivo `model_tests.rs` puede usar `use super::*;` para acceder al módulo
padre. Sigue siendo parte del test interno del crate aunque viva en otro archivo.

La guía de Warp prescribe archivos `nombre_tests.rs` o `mod_test.rs`. Rust
también admite un bloque `#[cfg(test)] mod tests { ... }` dentro del archivo de
producción. Elegir una convención y mantenerla consistente; los archivos separados
resultan útiles cuando las pruebas hacen difícil leer el código de producción.
Fuentes: [guía de Warp](https://github.com/warpdotdev/warp/blob/master/AGENTS.md)
y [organización de tests en Rust](https://doc.rust-lang.org/book/ch11-03-test-organization.html).

### Pruebas externas por crate

Cada archivo de entrada en `tests/` es un objetivo de integración de Cargo.
Consume la API pública del crate y sus dependencias declaradas. Puede probar un
caso de uso aislado o varias piezas conectadas: la clasificación de compilación
y el alcance de la prueba son dimensiones distintas.

Cada suite incorpora su soporte con:

```rust
mod support;
```

Las utilidades de una suite no aparecen automáticamente en otra. Cada paquete
declara las herramientas de test en sus propias `dev-dependencies`; declarar una
dependencia en `workspace.dependencies` no la incorpora por sí solo a sus miembros.

### Submódulos cuando una suite crece

```text
tests/
├── integration.rs
├── integration/
│   ├── mod.rs
│   ├── create.rs
│   └── read.rs
└── support/
    └── mod.rs
```

`integration.rs` declara `mod integration;` y `mod support;`. A su vez,
`integration/mod.rs` declara `mod create;` y `mod read;`. Los escenarios pueden
acceder al soporte con `crate::support`.

Esta composición conserva una entrada detectada por Cargo. Mover archivos a
subcarpetas sin declararlos no los incorpora a la suite. Véase la
[referencia de Cargo](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests).

La guía complementaria [UI_APP_TESTING_WARP.md](UI_APP_TESTING_WARP.md) conserva
la explicación original de Warp, su suite GUI y el ejemplo de adaptación.

## 3. Lineamientos de diseño y arquitectura

1. Los tests siguen los límites de la aplicación. Si existe arquitectura hexagonal,
   utilizar sus puertos para sustituir dependencias. No crear una segunda
   arquitectura de servicios y repositorios exclusivamente para organizar tests.
2. Mantener real la unidad o capa que se desea comprobar.
3. Elegir el alcance mínimo que detecte el fallo con claridad y añadir pruebas
   amplias para comprobar las conexiones entre capas.
4. Escribir escenarios con preparación, ejecución y comprobación explícitas.
5. Comprobar comportamientos observables y efectos relevantes, evitando fijar
   detalles internos que puedan cambiar durante una refactorización.
6. Mantener los datos predecibles y cada escenario independiente del orden de
   ejecución. Sobrescribir en builders los campos que explican el caso.
7. Probar reglas de negocio en su capa responsable para que se cumplan también
   cuando la aplicación se invoque sin HTTP o sin UI.
8. Separar fakes según el consumidor. Un fake de repositorio ayuda a application;
   un fake de caso de uso ayuda al adaptador de entrada.
9. Mantener utilidades locales. Extraer un crate de soporte compartido solo si
   varias suites reutilizan realmente una preparación estable.
10. Evitar duplicar el mismo escenario en tests internos y externos. La repetición
    tiene sentido cuando verifica una frontera adicional.
11. Coordinar pruebas async con señales y límites de espera, sin pausas arbitrarias.
    Cerrar servidores y conexiones al finalizar.
12. Elegir herramientas según necesidades: `rstest` para fixtures y casos
    parametrizados; snapshots cuando una salida estructurada resulte más fácil de
    revisar así; nextest para necesidades de ejecución. Ninguna herramienta define
    por sí sola la calidad de las pruebas.

## 4. Qué corresponde comprobar en cada nivel

| Nivel | Código real | Dependencia sustituida | Ejemplos |
| --- | --- | --- | --- |
| Dominio | Reglas y constructores | Habitualmente ninguna | Valores válidos, inválidos y límites |
| Application | Casos de uso y servicios | Persistencia o servicios externos | Transformaciones, coordinación y errores |
| Entrada HTTP/UI | Rutas/controladores o componentes | Casos de uso | Conversión de datos, eventos y respuesta visible |
| Infraestructura | Adaptador y servicio de pruebas | Ninguna dentro del alcance declarado | SQL, mapeos y restricciones |
| Sistema en proceso | Capas conectadas | Según el alcance declarado | Un cambio se guarda y se puede consultar |
| End-to-end | Aplicación arrancada y cliente | Solo dependencias externas declaradas | Recorrido público y composición real |

Una prueba HTTP en proceso puede recorrer todas las capas y la base de datos,
pero no demuestra que el servidor abra un socket correctamente. Una prueba
end-to-end por red añade ese alcance. En UI, los tests del estado de una vista
tampoco sustituyen por sí solos una prueba de interacción y renderizado reales.

## 5. Contratos, fakes y errores

Implementar un trait demuestra compatibilidad de firmas. Las pruebas verifican
las promesas de comportamiento: resultados, errores y efectos.

Un stub configurable que devuelve respuestas preparadas no necesariamente
almacena datos ni reproduce la semántica completa de un repositorio. No exigirle
un escenario de «crear y recuperar lo creado» si no se diseñó para ello.

Una suite de contrato compartida aporta valor cuando varias implementaciones
prometen la misma semántica. Un repositorio funcional en memoria puede compartir
esos escenarios con el real. Los detalles SQL y las migraciones requieren pruebas
del adaptador real.

Definir explícitamente reglas sobre identificadores, inexistencia, duplicados y
orden de listas antes de probarlas. La traducción de errores se verifica en las
fronteras: error de infraestructura a significado de aplicación, y de aplicación
a respuesta HTTP o mensaje visible. Estados como 404 y 409 son decisiones del
contrato público, no garantías automáticas de usar una arquitectura determinada.

## 6. Infraestructura y aislamiento de datos

- Provisionar una base de pruebas y aplicar las migraciones reales.
- Usar bases o esquemas aislados, o transacciones que realmente incluyan las
  operaciones del adaptador.
- Abrir una transacción en el test no aísla consultas que el repositorio ejecuta
  por otras conexiones del pool.
- Evitar datos compartidos que colisionen al ejecutar en paralelo.
- Probar inserción, lectura, listas, restricciones y mapeos; un ping solo verifica
  disponibilidad.
- Usar puertos dinámicos y señales de disponibilidad para servidores de pruebas.
- Aislar pruebas de configuración global, por ejemplo mediante entradas inyectables
  o procesos separados.
- Limpiar recursos incluso cuando falle una comprobación.

## 7. Adopción gradual y ejecución

1. Identificar el dueño de cada prueba: módulo, crate o sistema.
2. Colocar las pruebas locales junto al código y las pruebas externas en su crate.
3. Separar el soporte por consumidor y ajustar las dependencias de desarrollo.
4. Comprobar que Cargo descubre las suites y que se conservan sus escenarios.
5. Añadir adaptadores reales y recorridos completos según los riesgos.
6. Integrar los grupos de pruebas en CI, con entornos reproducibles.

Los comandos exactos dependen de los nombres de los paquetes y suites. Como base:

```bash
cargo test --workspace
cargo test --workspace --doc
cargo fmt --all --check
cargo clippy --workspace --all-targets
```

Si se adopta nextest, ejecutar también los doc tests por separado. Los ejemplos
de otros proyectos no obligan a instalar todas sus herramientas.

Separar las pruebas rápidas de las que requieren infraestructura. Cuando se
selecciona una suite real, la ausencia de su base o configuración debe causar un
fallo visible. No presentar como validación real un retorno temprano que omitió
la prueba. Las features o tests ignorados requieren jobs que los activen.

En CI, ejecutar las suites obligatorias, publicar sus resultados y hacer que sus
fallos bloqueen la integración. Ni un porcentaje alto de cobertura ni una
distribución numérica de tests certifican por sí solos preparación para producción.

## 8. Referencias de backend: respuesta conservada

La siguiente recomendación se conserva como contexto para elegir proyectos de
estudio. Crates.io, Lemmy y Kanidm son referencias de backend; Warp es la
referencia complementaria de organización Rust y pruebas de UI. Sus estructuras
no son idénticas ni constituyen una única norma universal.

---

Sí, existen. **Para tu backend empezaría por `rust-lang/crates.io`**: es el código del registro oficial de paquetes de Rust, un servicio real con API y base de datos.

| Proyecto | Qué puedes aprender de sus tests |
|---|---|
| [crates.io](https://github.com/rust-lang/crates.io) | API, PostgreSQL, bases aisladas por prueba, migraciones y comprobación de respuestas. |
| [Lemmy](https://github.com/LemmyNet/lemmy) | Backend de una plataforma social: usuarios, permisos, persistencia y pruebas de API entre servidores. |
| [Kanidm](https://github.com/kanidm/kanidm) | Gestión de identidades: autenticación y pruebas que recorren desde almacenamiento hasta clientes y servidor. |

**Crates.io es el más cercano a lo que necesitas estudiar ahora.** Su guía explica que el entorno de tests crea bases aisladas y aplica migraciones automáticamente. También comprueba respuestas de API con *snapshots*: resultados esperados guardados para detectar cambios. [Guía de testing de crates.io](https://github.com/rust-lang/crates.io/blob/main/AGENTS.md#testing).

Lemmy también es una referencia útil con PostgreSQL, aunque sus pruebas externas de API utilizan TypeScript/Jest; no todas están escritas en Rust. [Directorio `api_tests`](https://github.com/LemmyNet/lemmy/tree/main/api_tests).

Kanidm tiene un enfoque especialmente interesante de pruebas del sistema completo, explicado en su [documentación de desarrollo](https://kanidm.github.io/kanidm/stable/developers/faq.html).

Mi elección sería: **crates.io para estudiar tests de backend y PostgreSQL; Warp como referencia complementaria de organización Rust**. No necesitamos copiar su aplicación entera: podemos seguir un endpoint, ver cómo prepara sus datos, ejecuta la petición y comprueba el resultado, y trasladar ese patrón a tus puertos.
