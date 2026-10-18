# 04 — SurrealKit: versión y comandos verificados

Fecha de creación: 2026-10-17
Estado: Investigación y ensayo local sin base de datos
Tipo de fuente: Repositorio oficial, código fuente y CLI oficial
Fecha de consulta: 2026-10-17, America/New_York
Relacionada con: [plan de migraciones](../../plans/04-esquema-y-migraciones-surrealkit.md), [plan anterior](../../plans/01-migrar-usuarios-postgresql-surrealdb.md)

## Procedencia

- [Repositorio oficial](https://github.com/surrealdb/surrealkit), README,
  «Sync vs Rollouts» y «How schema files are applied».
- [Release v1.0.0-beta.6](https://github.com/surrealdb/surrealkit/releases/tag/v1.0.0-beta.6),
  publicada el 2026-10-02; coincide con la versión máxima consultada en crates.io.
- Tag beta.6 resuelto al commit `92ebc3cc89b99f00d21c7d2cbf5136d728367061`.
  Se contrastó también con main `fd7b075c7619138cf5b8704d9f1938a63e3c977f`;
  el archivo de rollouts consultado coincide en ambas revisiones.
- [run_baseline](https://github.com/surrealdb/surrealkit/blob/92ebc3cc89b99f00d21c7d2cbf5136d728367061/crates/surrealkit/src/rollout.rs),
  líneas 934–958; `run_plan`, desde 962.
- [CLI](https://github.com/surrealdb/surrealkit/blob/92ebc3cc89b99f00d21c7d2cbf5136d728367061/crates/surrealkit/src/main.rs),
  `RolloutCommands` y despacho de `Baseline`.

Se descargó el binario oficial `aarch64-apple-darwin` únicamente en `/tmp`,
sin instalarlo globalmente. Su SHA-256 coincidió con el checksum publicado:

```text
a6d34069851982a5da1d7633f803ae42edffe40e80f0e12e0a55063a64c4ee3f
```

Los nombres de rollouts generados por la herramienta usan UTC. Los documentos
de esta carpeta usan la fecha del usuario en America/New_York.

## Hallazgos frente a las skills locales

La skill local está redactada para beta.3. La documentación y el binario beta.6
se contrastaron antes de convertir sus ejemplos en un plan:

| Afirmación que requiere revisión | Evidencia comprobada |
| --- | --- |
| `surrealkit check` está disponible | El binario beta.6 devuelve `unrecognized subcommand 'check'`; tampoco figura en el enum CLI del código consultado. No será una puerta de CI asumida. |
| `baseline` es solo lectura | `run_baseline` ejecuta setup, reemplaza metadatos administrados y guarda snapshots. Puede escribir en la base. No es una auditoría inocua ni verifica por sí solo que archivos y esquema real coincidan. |
| Basta versionar manifest y snapshots | beta.6 también congela el SQL en `database/rollouts/<id>/`. Hay que conservar los tres. |
| Repetir `plan` sin cambios no crea nada | El ensayo creó un segundo manifest con hashes iguales y `steps = []`. Revisar el diff antes de versionar; no generar migraciones vacías por rutina. |

Se conserva la skill importada sin reescribir el material del proveedor.
Este documento registra las discrepancias verificadas.

## Ensayo realmente ejecutado

En una carpeta temporal nueva se ejecutaron:

1. `surrealkit --version`: `1.0.0-beta.6`.
2. Ayuda de `init`, `sync`, `rollout baseline`, `plan`, `up` y `lint`.
3. `surrealkit init --minimal --yes`: creó configuración, setup y directorios.
4. Se escribió el borrador de `account` del plan de migraciones en esa carpeta.
5. `surrealkit rollout plan --name initial_firebase_account`: creó manifest,
   copia congelada y snapshots.
6. `surrealkit rollout lint`: aceptó el primer rollout.
7. Segundo `plan` sobre el mismo esquema: creó un rollout vacío; `lint` aceptó
   la cadena de los dos.

El ensayo demuestra generación, reconocimiento de definiciones y coherencia
de manifests. **No demuestra que el servidor ejecute el SurrealQL, que se
cumplan los índices ni que el repositorio funcione.** No se conectó a ninguna
base, ni se ejecutaron `sync`, `baseline`, `up`, `start` o `complete`.
Los artefactos temporales no son las migraciones reales del proyecto.

## Decisión propuesta

Fijar beta.6 como herramienta externa de esquema, separada del binario API.
No agregarla como librería para ejecutar migraciones en cada arranque.
Usar sincronización solo en bases desechables y rollouts revisados en bases
compartidas. Su condición de prerelease exige mantener la versión fijada y
volver a contrastar CLI y plan antes de actualizar.
