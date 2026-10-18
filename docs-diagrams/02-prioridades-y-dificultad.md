# 02 — Prioridad comercial y dificultad

Fecha de creación: 2026-10-12  
Estado: Investigación y propuesta.  
Relacionado: [producto](./01-producto-y-prototipo.md), [fases](./03-fases-y-diagramas.md), [fuentes](./04-fuentes-y-validacion.md).

## Orden comercial: de mayor a menor prioridad inicial

Este orden estima la **oportunidad de conseguir los primeros clientes del segmento elegido**, considerando problema recurrente, evidencia de uso y posibilidad de cobrar. No es un ranking de ventas de Channable: no encontramos cifras públicas comparables de ingresos por función. Las referencias E1–E7 están en el [registro de fuentes](./04-fuentes-y-validacion.md).

| Orden | Capacidad | Por qué la priorizamos | Resultado para el cliente | Dificultad |
| --- | --- | --- | --- | --- |
| 1 | Preparación del feed: mapeo, reglas y validación | Es el núcleo de ofertas comerciales y casos de uso observados. E1, E2, E3. | Archivo para el canal y reporte de errores. | 3/5 |
| 2 | Actualización programada y alertas | Resuelve el trabajo repetitivo; los proveedores diferencian planes por frecuencia. E1, E2. | URL actualizada, historial y avisos. | 4/5 |
| 3 | Espacios de agencias y reglas reutilizables | Facilita administrar más clientes; existen planes específicos de agencias. E1, E2, E3. | Proyectos separados y plantillas compartidas. | 3/5 |
| 4 | Segmentación por margen y rendimiento | Conecta el catálogo con decisiones económicas, pero exige datos fiables adicionales. E4. | Etiquetas y selección de productos para campañas. | 4/5 |
| 5 | Publicación y sincronización en un marketplace | Amplía canales de venta; tiene oferta y módulos de pago establecidos. Se aleja del comprador inicial. E5. | Publicaciones, estado de sincronización y, después, pedidos. | 5/5 |
| 6 | Sugerencias de títulos y atributos con IA | Aparece en ofertas actuales; la dejaríamos como mejora porque primero hacen falta datos correctos. E1, E2. | Sugerencias revisables antes de exportar. | 3/5 |
| 7 | Imágenes dinámicas de producto | Puede aportar valor a negocios visuales; añade otro flujo y más trabajo de interfaz. E6. | Imágenes PNG y sus URLs en el feed. | 4/5 |

El orden 4–7 tiene menor confianza para nuestro público. Lo revisaremos con solicitudes y pagos reales; una función posterior puede adelantarse si varios clientes la necesitan.

## Orden por dificultad: de menor a mayor

Estimación para una versión pequeña pero utilizable: 1 = baja; 5 = muy alta. Incluye backend, interfaz mínima, errores e integración, no solo escribir endpoints.

| Nivel | Capacidades, orden aproximado dentro del nivel | Principal dificultad |
| --- | --- | --- |
| 3/5 | Feed básico → sugerencias IA → espacios de agencias | Reglas de datos; evaluación de sugerencias; aislamiento y permisos entre clientes. |
| 4/5 | Actualización programada → segmentación → imágenes | Reintentos y publicación consistente; unión de métricas; plantillas y procesamiento visual. |
| 5/5 | Marketplace con sincronización de pedidos | APIs externas, límites, duplicados y conciliación de inventario/pedidos. |

La función de agencias necesita la identidad y el aislamiento básicos previstos antes del piloto. La segmentación presupone un catálogo estable. La prioridad comercial no elimina estas dependencias.

## Capacidades aplazadas

Automatizar campañas PPC, repricing y servicios CSS no forman parte del plan inicial. PPC implica gestionar campañas mediante APIs; repricing añade precios competitivos y límites de margen; CSS exige un modelo operativo específico. No construiremos todo el catálogo de Channable. [Referencia PPC, E7](./04-fuentes-y-validacion.md).
