# 04 — Fuentes y validación comercial

Fecha de creación y consulta: 2026-10-12  
Estado: Investigación inicial.  
Tipo de fuentes: páginas oficiales de proveedores, documentación del canal y código local.  
Relacionado: [producto](./01-producto-y-prototipo.md), [prioridades](./02-prioridades-y-dificultad.md), [fases](./03-fases-y-diagramas.md).

## Evidencia consultada

| ID | Fuente y localización | Hecho observado y aplicación |
| --- | --- | --- |
| E1 | [Channable: Feed Management](https://www.channable.com/products/product-feed-management-tool), sección de planes Core | Comercializa feeds, reglas y controles de calidad; diferencia niveles por frecuencia y otras capacidades. Apoya comenzar con feeds y automatización. |
| E2 | [DataFeedWatch: precios](https://www.datafeedwatch.com/pricing), planes y servicios | Publica Shop a USD 64/mes y Agency a USD 239/mes, con distintos límites, actualizaciones y soporte. También ofrece gestión asistida. Confirma oferta comercial comparable, no nuestras ventas futuras. |
| E3 | [Caso Comptoir des Cotonniers e iProspect](https://www.channable.com/success-stories/boost-ctr-31-with-feed-solution), retos y resultados; publicado el 2024-07-23 | Describe necesidades de reglas, etiquetas y migración con una agencia; reporta +31% de CTR. Testimonio publicado por el proveedor, sin atribución causal independiente. |
| E4 | [Channable Insights](https://www.channable.com/products/insights-product-segmentation), datos de producto y negocio | Combina métricas publicitarias con margen y existencias para segmentar. Fundamenta una ampliación que requiere más datos. |
| E5 | [Marketplace Integrator](https://www.channable.com/products/marketplace-integrator), sincronización y módulos | Ofrece publicaciones, pedidos y repricing. Es una línea comercial existente con mayor complejidad operativa. |
| E6 | [Dynamic Image Editor](https://www.channable.com/products/dynamic-image-editor), preguntas frecuentes | Genera imágenes PNG a partir de plantillas y datos. Justifica identificar imágenes como capacidad separada. |
| E7 | [PPC Automation](https://www.channable.com/products/ppc-tool), creación de campañas | Automatiza campañas y anuncios desde el catálogo. Describe una expansión que aplazamos. |
| E8 | Google: [formatos de archivo](https://support.google.com/merchants/answer/12631822?hl=es) y [datos de producto](https://support.google.com/merchants/answer/7052112?hl=es) | Permite TSV/TXT y XML; define atributos y requisitos condicionales. Determina el contrato del primer exportador. |

## Fuentes técnicas para el diseño SurrealDB

Consultadas el 2026-10-12. Sustentan mecanismos disponibles; el esquema y los contratos de este proyecto son propuestas propias.

| ID | Fuente oficial y sección | Aplicación y límite |
| --- | --- | --- |
| E9 | [Rust SDK](https://surrealdb.com/docs/reference/rust) y [tablas](https://surrealdb.com/docs/learn/schema-management/tables-and-fields/tables), Schemafull tables | Cliente remoto y campos definidos. Confirmar compatibilidad del SDK instalado y del servidor antes de implementar. |
| E10 | [DEFINE INDEX](https://surrealdb.com/docs/reference/query-language/statements/define/indexes), Unique index y Composite index | Unicidad de identificadores dentro de una versión y revisiones dentro del proyecto. |
| E11 | [COMMIT](https://surrealdb.com/docs/reference/query-language/statements/commit), transacciones agrupadas | Confirmar cambios de registros juntos; el almacenamiento de archivos queda fuera de la transacción. |
| E12 | [Tablas](https://surrealdb.com/docs/learn/schema-management/tables-and-fields/tables), Defining permissions, y [autenticación](https://surrealdb.com/docs/learn/security/authentication/summary) | Distinguir usuarios de sistema y usuarios de registro. Autorización de clientes en la aplicación; los permisos de tabla no filtran una sesión root. |

Código contrastado: [manifiesto](../Cargo.toml), [conexión SurrealDB](../crates/backend_infrastructure/src/database/surrealdb/connection.rs), [recursos de bases](../crates/backend_bootstrap/src/bootstrap/resources/databases/mod.rs) y [repositorio de usuarios](../crates/backend_infrastructure/src/user_infrastructure/repositories/user_repository.rs). El diseño mantiene el patrón de puertos existente y documenta la transición pendiente. Las vistas de contexto, componentes, dominio, actividad, secuencia y estados se expresan en Mermaid y describen el sistema objetivo.

Precios y capacidades corresponden a la consulta y pueden cambiar. Estas fuentes demuestran ofertas y usos de la categoría; **no prueban qué función vende más, el tamaño de nuestro mercado accesible ni una ventaja frente a competidores**. El orden y las fases son propuestas propias.

## Validación antes de ampliar

1. Hablar con cinco agencias o tiendas del segmento propuesto: cómo resuelven hoy el catálogo, frecuencia de correcciones y coste del problema.
2. Conseguir dos catálogos autorizados para probar el recorrido y compararlo con la alternativa actual del cliente.
3. Intentar vender un piloto de configuración y mantenimiento a un cliente; registrar tiempo de soporte, errores corregidos y margen del servicio.
4. Avanzar a una capacidad nueva cuando exista necesidad repetida y disposición a pagar. Si los clientes prefieren su herramienta actual, revisar el segmento y la propuesta antes de extender el producto.

Estas cantidades son objetivos internos de aprendizaje. No son una validación estadística ni una garantía de demanda.
