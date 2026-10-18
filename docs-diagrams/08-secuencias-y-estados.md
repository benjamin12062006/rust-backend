# 08 — Secuencias, estados y consistencia

Fecha de creación: 2026-10-12  
Estado: Diseño propuesto; recuperación avanzada en fase B.  
Relacionado: [requisitos](./05-analisis-y-requisitos.md), [datos](./06-dominio-y-datos-surrealdb.md), [contratos](./07-arquitectura-y-contratos.md).

## Importación y evaluación

Importar y evaluar son operaciones distintas: el operador define las columnas sobre una entrada ya conservada. En A habrá un trabajador local; B agrega recuperación tras reinicios y leases.

```mermaid
sequenceDiagram
    actor U as Operador
    participant API as API y casos de uso
    participant FS as ArtifactStore
    participant DB as SurrealDB por repositorios
    participant W as Trabajador
    participant D as Motor de dominio
    U->>API: Subir CSV y elegir columna identificadora
    API->>API: Validar contexto, formato y limites
    API->>FS: Finalizar archivo original privado
    FS-->>API: Referencia y checksum
    API->>DB: Crear snapshot y trabajo de importacion
    DB-->>API: Identificadores persistidos
    API-->>U: 202 y URL del trabajo
    W->>DB: Reclamar trabajo
    W->>FS: Leer original por bloques
    W->>DB: Guardar filas y completar snapshot
    U->>API: Guardar version de mapeo y reglas
    API->>DB: Crear ruleset_version
    U->>API: Evaluar snapshot con esa version
    API->>DB: Crear trabajo de evaluacion
    W->>DB: Reclamar y leer versiones fijadas
    W->>D: Normalizar, aplicar reglas y validar
    D-->>W: Filas evaluadas, problemas y conteos
    W->>FS: Guardar resultados e informe completos
    W->>DB: Confirmar evaluacion y terminar trabajo
    U->>API: Consultar resultado
    API-->>U: Resumen, muestra y errores
```

El snapshot solo pasa a `ready` cuando termina la importación y se comprueba unicidad de IDs. Un error de parseo o un duplicado lo deja `failed`, con informe y sin permitir evaluación. Si el archivo se guarda pero la creación de registros falla, queda un artefacto huérfano: una limpieza posterior verifica referencias antes de retirarlo.

## Publicación consistente

Secuencia objetivo de B. En A se genera el mismo archivo como descarga privada. La aprobación identifica la evaluación, checksum y exclusiones aceptadas.

```mermaid
sequenceDiagram
    actor U as Operador autorizado
    participant APP as PublicarFeed
    participant FS as ArtifactStore
    participant DB as SurrealDB por FeedRepository
    participant C as Merchant Center
    U->>APP: Confirmar evaluacion y generacion esperada
    APP->>DB: Leer evaluacion y revision del feed
    APP->>APP: Comprobar aprobacion y productos validos
    APP->>FS: Escribir TSV nuevo, finalizar y verificar
    FS-->>APP: Referencia inmutable y checksum
    APP->>DB: Transaccion: version y cambio condicional del puntero
    alt Generacion y autorizacion siguen vigentes
        DB-->>APP: Commit de nueva version publicada
        APP-->>U: Version, fecha y resultado
    else Conflicto o fallo
        DB-->>APP: No cambiar version vigente
        APP-->>U: Conflicto o fallo recuperable
    end
    C->>APP: GET URL estable del feed
    APP->>DB: Leer puntero publicado
    APP->>FS: Leer archivo de esa version
    APP-->>C: TSV completo de version vigente
```

Una transacción SurrealDB no incluye el disco ni un servidor externo. Por eso se finaliza el archivo antes de cambiar el puntero. Una lectura ya iniciada puede devolver la versión anterior completa; nunca mezcla dos versiones. El archivo previo se conserva mientras sea referenciado. [Transacciones, E11](./04-fuentes-y-validacion.md).

En B, la actualización automática requiere una política aprobada: versiones de reglas/perfil, exclusiones permitidas y límite de caída de productos. Si falla esa política, queda pendiente de revisión y se mantiene la publicación anterior. El umbral se acordará con el piloto; no será una cifra oculta.

## Ciclo de vida del trabajo

```mermaid
stateDiagram-v2
    [*] --> queued
    queued --> running: reclamo exclusivo
    running --> succeeded: resultado completo y token vigente
    running --> retry_wait: fallo temporal o lease vencido
    retry_wait --> queued: espera y limite de intentos
    running --> failed: entrada invalida o fallo definitivo
    retry_wait --> failed: intentos agotados
    succeeded --> [*]
    failed --> [*]
```

Cada reclamo tiene `lease_until` y un token de ejecución. El trabajador renueva el lease y solo puede terminar/publicar con el token vigente. Recuperar un trabajo invalida el token anterior. El procesamiento puede repetirse: **no prometemos ejecución exactamente una vez**, controlamos sus efectos mediante idempotencia y escritura condicional.

En B, una clave de idempotencia se limita a organización, proyecto y operación; repetirla con el mismo contenido devuelve el trabajo existente, y con contenido distinto produce conflicto. El programador usa una clave por ejecución prevista. Reintentos tienen espera creciente y máximo configurable; errores de datos requieren corrección, no reintentos automáticos.
