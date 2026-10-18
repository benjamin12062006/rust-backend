# 01 — Producto y prototipo inicial

Fecha de creación: 2026-10-12  
Estado: Plan inicial; funcionalidades propuestas, todavía sin implementar.

Esta carpeta establece la dirección del producto y es el punto de entrada para futuras implementaciones. **Persistencia objetivo: SurrealDB**, por decisión del usuario; los diagramas describen el sistema propuesto, no funcionalidades ya implementadas.

## Mapa de lectura

| Documento | Pregunta que responde |
| --- | --- |
| [02 — Prioridades y dificultad](./02-prioridades-y-dificultad.md) | ¿Qué vale la pena construir y qué cuesta más? |
| [03 — Fases](./03-fases-y-diagramas.md) | ¿En qué orden lo construiremos? |
| [04 — Fuentes](./04-fuentes-y-validacion.md) | ¿Qué evidencia respalda las decisiones? |
| [05 — Análisis y requisitos](./05-analisis-y-requisitos.md) | ¿Quién usa el sistema, para qué y dentro de qué límites? |
| [06 — Dominio y datos](./06-dominio-y-datos-surrealdb.md) | ¿Qué conceptos existen y qué reglas deben cumplir? |
| [07 — Arquitectura y contratos](./07-arquitectura-y-contratos.md) | ¿Qué responsabilidad tiene cada módulo y cómo se comunican? |
| [08 — Ejecución y estados](./08-secuencias-y-estados.md) | ¿Qué sucede paso a paso y cómo se recuperan los fallos? |
| [09 — Decisiones y construcción](./09-decisiones-y-plan-de-construccion.md) | ¿Qué está decidido y qué debe verificarse antes de ampliar? |

## Qué construiremos

Un servicio para **preparar y mantener catálogos de productos destinados a canales de venta y publicidad**. El cliente configura las reglas una vez; el sistema transforma su catálogo, explica los errores y genera el archivo que necesita el canal.

**Comprador inicial propuesto:** una agencia pequeña de comercio electrónico cuyos clientes reciben catálogos CSV de proveedores y necesitan adaptarlos para Google Merchant Center. Empezaremos con una tienda, un país y una moneda por proyecto. Este segmento es una hipótesis que debemos validar.

**Problema:** corregir y exportar manualmente los mismos campos cada vez que cambian productos, precios o disponibilidad. Hay productos comerciales que resuelven esta categoría; nuestra entrada propuesta es la configuración asistida de catálogos difíciles y un reporte claro por producto. Su diferenciación todavía no está demostrada. [Evidencia y límites](./04-fuentes-y-validacion.md).

## Qué recibe exactamente el cliente

| Entrada o acción | Resultado previsto |
| --- | --- |
| Subir `catalogo.csv`, elegir la columna identificadora y mapear los demás campos | Catálogo normalizado y vista previa, conservando identificadores como texto. |
| Ejecutar reglas y validaciones | `errores.csv`: fila, producto, campo, problema y corrección sugerida; resumen JSON para la interfaz. |
| Revisar y confirmar la exportación | `productos.tsv`: archivo delimitado por tabuladores para Merchant Center. |
| Activar automatización en la siguiente fase | URL estable del feed, historial de actualizaciones y avisos cuando falle una ejecución. |

La exportación debe respetar los requisitos del país y categoría elegidos. **Superar nuestras validaciones no garantiza la aprobación de Google**: también revisa la cuenta, el sitio y sus políticas. [Formato admitido](https://support.google.com/merchants/answer/12631822?hl=es), [especificación de atributos](https://support.google.com/merchants/answer/7052112?hl=es).

## Cómo se vendería

Hipótesis: cobrar una configuración inicial y una suscripción por tienda, volumen de productos y frecuencia de actualización. El servicio recurrente mantiene el catálogo actualizado y permite corregir problemas; el archivo es uno de sus resultados. No fijaremos precios hasta probar un piloto pagado y medir soporte e infraestructura.

## Alcance de la primera entrega

CSV de entrada, mapeo manual, tres reglas predefinidas —limpiar texto, construir título y excluir productos por condición—, validación por fila, vista previa y exportación TSV. Una interfaz sencilla permitirá completar el recorrido.

Conectores de tiendas, otros canales, pedidos, repricing, campañas publicitarias e imágenes quedan para decisiones posteriores. El [plan de construcción](./03-fases-y-diagramas.md) separa el prototipo local de la primera versión comercial.
