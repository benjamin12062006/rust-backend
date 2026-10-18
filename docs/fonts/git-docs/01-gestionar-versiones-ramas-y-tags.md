# 01 — Gestionar versiones, ramas y tags con Git

Fecha de creación: 2026-10-11
Tipo de fuente: Elaboración propia basada en el proceso ejecutado en este repositorio
Origen: Explicación del chat y verificaciones de Git del backend
Fecha de consulta: 2026-10-11
Relacionada con: [Integración del evaluador](../../../bitacora-construccion/01-integrar-evaluador-expresiones.md), [integración del backend reciente en main](../../../bitacora-construccion/02-integrar-version-reciente-en-main.md)
Revisión relacionada: `260096f28a116e88b8dae300ea30566457d018cb`

## 1. Qué conservamos y para qué

La regla del proceso es:

```text
guardar → integrar → comprobar → crear el commit final → etiquetarlo
```

Git conserva versiones de los archivos rastreados mediante commits. Una rama
señala una línea de trabajo; un tag identifica un punto concreto del historial.

| Concepto | Qué representa | Qué ocurre al continuar trabajando |
| --- | --- | --- |
| Commit | Un snapshot de los archivos rastreados, con sus padres e información del guardado | Su contenido no cambia. |
| Rama | Una referencia a un commit, por ejemplo `main` | Avanza al crear nuevos commits en esa rama. |
| Tag | Un nombre que identifica una versión, por ejemplo `v1.5.0` | No avanza con los nuevos commits; no debe moverse para representar otra versión. |
| Merge | Una integración de historiales | Un commit de merge conserva los commits integrados como padres. |

**Un tag pertenece al repositorio, no a una rama.** Decir que una versión está
«etiquetada en main» significa que su commit forma parte del historial de `main`.
Se puede consultar o visitar ese tag desde cualquier rama.

## 2. Estado guardado en este proyecto

Al completar la integración, las referencias quedaron así:

```text
main                    → 260096f ← tag v1.5.0
                             │
                             ├── padre 1: 4b9b652
                             └── padre 2: b4e746e ← rama v1.4.1 y tag v1.4.1
```

- `4b9b652`: respaldo del evaluador y la documentación sobre el `main` antiguo.
- `b4e746e`: backend reciente de la rama `v1.4.1`.
- `260096f`: integración que contiene el backend reciente y `expression_evaluator`.
- `v1.5.0`: tag anotado sobre el commit integrado.

Estos hechos se pueden recuperar de las revisiones indicadas y de la
[entrada 02 de la bitácora](../../../bitacora-construccion/02-integrar-version-reciente-en-main.md).
No implican que `main` vaya a seguir en ese mismo commit después de nuevos avances.

Este documento se creó después de etiquetar `v1.5.0`: **no forma parte del
snapshot de ese tag** salvo que se guarde posteriormente bajo otra revisión.

## 3. Cómo se realizó la integración

### Paso 1 — Guardar el trabajo existente

Primero se revisaron el árbol de trabajo y el índice:

```bash
git status --short --branch
git diff
git diff --cached
```

Se prepararon rutas explícitas del evaluador, manifests, pruebas y documentación
mediante `git add -- <rutas>`. La notación `<rutas>` es un marcador de explicación,
no un argumento para copiar literalmente. No se incluyó `.env.local.example`.

Después se creó el respaldo:

```bash
git commit -m 'feat: guardar evaluador de expresiones y estructura documental'
```

El resultado fue `4b9b652`. Ese commit permitió visitar otra versión sin perder
el trabajo que estaba pendiente.

### Paso 2 — Revisar las ramas y sus historiales

Entre las comprobaciones ejecutadas estuvieron:

```bash
git branch -vv
git worktree list
git log --oneline --graph --decorate --all -n 18
git merge-base --all refs/heads/main refs/heads/v1.4.1
git rev-parse --is-shallow-repository
```

`main` y `v1.4.1` no tenían un ancestro común; el repositorio no era superficial.
Por eso se pidió autorización para unir historiales independientes.

Este caso es una excepción del historial de este proyecto, **no una opción
que haya que usar en todas las fusiones**.

### Paso 3 — Fusionar sin cerrar inmediatamente el commit

Estando en `main`, se ejecutó:

```bash
git merge --allow-unrelated-histories --no-commit --no-ff \
  -X theirs refs/heads/v1.4.1
```

| Opción | Razón de su uso aquí |
| --- | --- |
| `--allow-unrelated-histories` | Permite unir los dos historiales independientes, con autorización. |
| `--no-commit` | Deja inspeccionar y comprobar el resultado antes de crear el commit de merge. |
| `--no-ff` | Solicita un commit de merge explícito; en este caso no había posibilidad de fast-forward. |
| `-X theirs` | Da prioridad al origen, `v1.4.1`, en los conflictos de contenido. |

**`-X theirs` no significa «reemplazar todo el proyecto por la otra rama».**
La fusión conservaba también archivos exclusivos del `main` anterior. Por eso
se revisó el resultado completo:

- Se mantuvieron los siete crates del backend reciente.
- Se conservó íntegro `expression_evaluator`, junto con su test de consumo.
- Se adaptó `Cargo.toml` para incluirlo en el workspace y como dependencia local.
- Cargo actualizó `Cargo.lock`.
- Se retiraron los módulos antiguos de `src/features/` y `src/shared/`, ya
  reemplazados por los crates.
- Se conservó la documentación de tests en `tests/docs/`, sin su copia antigua
  de `tests/TESTING_LAYER.md`.

Dos archivos procedentes de `v1.4.1` solo recibieron correcciones de whitespace:
`tests/support/fakes.rs` y `tests/docs/UI_APP_TESTING_WARP.md`.
No se cambió su lógica.

### Paso 4 — Conservar archivos locales sin versionarlos

Para `.env` y los archivos de `.idea` previamente rastreados se usó
`git rm --cached -- <rutas explícitas>`.

`--cached` retira los archivos del índice, **sin borrarlos del disco**. El
`.gitignore` del backend reciente evita agregarlos de nuevo por accidente.
Se verificó por hashes que esos archivos locales conservaran su contenido.

Esto los excluye del nuevo snapshot, pero **no los elimina de commits antiguos**.
Además, agregar una ruta a `.gitignore` no deja de rastrear automáticamente un
archivo que ya estaba versionado.

### Paso 5 — Comprobar antes de crear el commit final

Las verificaciones incluyeron:

```bash
RUN_DB_TESTS=false cargo test --workspace --offline --locked
cargo check --workspace --all-targets --offline --locked
git diff --cached --check
```

- La suite y la compilación fueron correctas.
- El test opt-in de base de datos retornó sin conectarse por `RUN_DB_TESTS=false`.
- Se compararon los archivos con ambos commits de partida.
- Se verificaron manifests, lockfile y el conjunto exacto de rutas del índice.
- Se comprobó que no quedaran archivos de entorno, del IDE ni módulos obsoletos
  en el snapshot preparado.

Los cambios se prepararon con rutas explícitas y se cerró el merge:

```bash
git commit -m 'merge: integrar v1.4.1 en main con el evaluador de expresiones'
```

El commit resultante fue `260096f`, con los dos avances como padres.

No se iniciaron el servidor ni conexiones reales a PostgreSQL, SurrealDB o
Qdrant. Tampoco se ejecutó CI remoto.

### Paso 6 — Etiquetar el commit integrado

Se eligió `v1.5.0`: un incremento MINOR respecto a `v1.4.1` por incorporar
la biblioteca del evaluador sin retirar funcionalidades del backend reciente.
La versión del paquete Cargo permaneció en `0.1.0`; no se confundió con esta
línea de versiones del repositorio.

Se verificó que el nombre del tag fuera válido y estuviera disponible:

```bash
git check-ref-format refs/tags/v1.5.0
git show-ref --verify refs/tags/v1.5.0
```

Antes de crear el tag, la segunda comprobación no encontraba esa referencia.
Un tag existente no se sobrescribe para guardar otro estado.

Se creó un tag anotado indicando explícitamente el commit final. El comando
siguiente está resumido; `<archivo-del-mensaje>` representa el archivo temporal
que contenía el resumen real del guardado:

```bash
git tag -a v1.5.0 260096f28a116e88b8dae300ea30566457d018cb \
  -F <archivo-del-mensaje>
```

Después se comprobó:

```bash
git cat-file -t refs/tags/v1.5.0
git rev-parse 'refs/tags/v1.5.0^{commit}'
git rev-parse refs/heads/main
git merge-base --is-ancestor 'refs/tags/v1.4.1^{commit}' refs/heads/main
git status --short --branch
```

El tipo del tag era `tag`, el commit desreferenciado coincidía con `main` y el
árbol de trabajo estaba limpio. El tag anterior `v1.4.1` no se movió.

## 4. Consultar, visitar y continuar desde una versión

### Consultar sin cambiar de rama

```bash
git show refs/tags/v1.5.0
git show refs/tags/v1.5.0:Cargo.toml
```

Solo muestra información. No cambia los archivos de trabajo ni la rama actual.

### Visitar el código del tag

Primero comprobar y guardar cualquier cambio pendiente:

```bash
git status --short --branch
```

Con el trabajo preservado y el árbol limpio:

```bash
git switch --detach refs/tags/v1.5.0
```

Los archivos rastreados pasan al estado de ese commit y quedas en **HEAD
separado**, sin estar en ninguna rama. No se mueve ni se hace retroceder `main`.
Tampoco entras en la rama donde se creó el tag.

Para regresar directamente a `main`:

```bash
git switch main
```

Para volver a la ubicación anterior, si no hubo cambios de ubicación intermedios:

```bash
git switch -
```

### Continuar desde el tag en una rama nueva

Con un nombre disponible y los cambios pendientes preservados:

```bash
git switch -c recuperacion-desde-v1.5.0 refs/tags/v1.5.0
```

La nueva rama parte del commit del tag y avanza con nuevos commits.
El tag se queda fijo. No importa desde qué rama empezaste.

### Hacer que una rama existente adopte el código anterior

Es una operación diferente de visitar el tag. Si una rama compartida debe volver
a un estado anterior de forma persistente, conviene estudiar una restauración
mediante un nuevo commit o reverts, conservando el historial.

No usar `reset --hard`, borrado de tags o force-push como atajos para una simple
consulta o recuperación. Primero decidir qué cambios deben conservarse.

## 5. Flujo recomendado para próximas versiones

Para este proyecto:

1. Usar `main` como línea principal del trabajo integrado.
2. Crear ramas temporales para cambios o experimentos cuando sean útiles, no
   una rama por cada número de versión.
3. Guardar avances mediante commits.
4. Integrar y comprobar el resultado.
5. Crear el tag sobre el commit final, después de la integración.

```text
feature temporal → integración en main → comprobaciones → tag de versión
```

La elección de versión depende del efecto sobre los consumidores:

| Incremento | Uso |
| --- | --- |
| PATCH | Corrección compatible. |
| MINOR | Funcionalidad nueva compatible. |
| MAJOR | Cambio incompatible. |

Para checkpoints sin cambio funcional o trabajo incompleto se debe respetar la
convención del proyecto; no fabricar una release estable por rutina.

## 6. Límites del guardado

- Un tag conserva el código y otros archivos rastreados del commit.
- No captura bases de datos, secretos, archivos ignorados ni procesos en ejecución.
- Visitar código antiguo no revierte automáticamente migraciones o datos.
- `Cargo.lock` ayuda a reproducir dependencias, pero no guarda por sí mismo todos
  los recursos externos necesarios para ejecutar el sistema.
- Un commit y un tag locales no equivalen a un respaldo remoto.
- No se hizo push de esta integración ni del tag durante el proceso descrito.

La publicación de rama y tag debe pedirse expresamente, revisando el remoto y
las referencias a enviar. No hace falta publicar para consultar el tag local,
pero perder el repositorio local sin otro respaldo también puede perder esas
referencias.
