# Contexto Actual: DDD Hexagonal Rust

Este archivo sirve para que un nuevo chat/agente entienda rapidamente que estamos haciendo en este workspace.

El usuario esta aprendiendo arquitectura hexagonal/DDD en Rust paso a paso. La regla de trabajo es: explicar con claridad, avanzar de forma incremental y no editar codigo salvo que el usuario lo pida explicitamente.

---

## 1) Proyecto de referencia usado

El proyecto de ejemplo esta en:

```text
/Users/benji/Documents/RUST/aprendisajeRS/rust-api-ddd
```

Ese DDD se esta usando como laboratorio para probar patrones antes de copiarlos al backend real:

```text
/Users/benji/Documents/RUST/backend
```

---

## 2) Capas actuales importantes

En el DDD trabajamos con estas capas:

```text
src/domain
src/ports
src/common
src/application
src/infrastructure
src/presentation
```

La regla mental aplicada es:

```text
presentation -> ports/in -> application -> ports/out -> infrastructure
```

---

## 3) Cambios recientes en `application`

Se reorganizo `src/application/user` para separar responsabilidades:

```text
src/application/user/
  mod.rs
  user_service.rs
  dto/
    mod.rs
    input/
      mod.rs
      create_user_input.rs
      update_user_input.rs
      login_input.rs
    output/
      mod.rs
      user_output.rs
      session_output.rs
  policies/
    mod.rs
    password_policy.rs
  use_cases/
    mod.rs
    create_user.rs
    list_users.rs
    get_user.rs
```

Responsabilidades:

```text
user_service.rs
  Fachada que implementa `IUserUseCase`.

use_cases/
  Logica concreta de cada accion.

dto/input/
  Datos que entran a casos de uso.

dto/output/
  Datos que salen de casos de uso.

policies/
  Reglas auxiliares de application, por ejemplo hash de password.
```

---

## 4) DTOs de application

`CreateUserInput` vive en:

```text
src/application/user/dto/input/create_user_input.rs
```

Representa los datos que necesita el caso de uso `create_user` para empezar.

`UserOutput` vive en:

```text
src/application/user/dto/output/user_output.rs
```

Representa la salida segura de application. No expone password.

---

## 5) Cambios en `ports/in`

El puerto de entrada `IUserUseCase` ahora usa DTOs de application:

```rust
async fn create_user(&self, input: CreateUserInput) -> Result<UserOutput>;
async fn list_users(&self) -> Result<Vec<UserOutput>>;
async fn get_user(&self, id: &Uuid) -> Result<UserOutput>;
```

Esto evita que presentation tenga que construir o recibir `User` directamente.

---

## 6) Cambios en `ports/out`

Se hizo una version mas estricta del repositorio usando records de frontera.

Estructura actual:

```text
src/ports/out/
  mod.rs
  OUTBOUND_PORT_RECORDS_GUIDE.md
  user/
    mod.rs
    user_repository.rs
    records/
      mod.rs
      create_user_record.rs
      stored_user_record.rs
```

`CreateUserRecord` vive en:

```text
src/ports/out/user/records/create_user_record.rs
```

Representa lo que application entrega al repositorio para crear un usuario.

`StoredUserRecord` vive en:

```text
src/ports/out/user/records/stored_user_record.rs
```

Representa lo que el repositorio devuelve despues de crear o leer un usuario.

El trait `IUserRepository` ahora usa records:

```rust
async fn create_user(&self, record: &CreateUserRecord) -> Result<StoredUserRecord>;
async fn list_users(&self) -> Result<Vec<StoredUserRecord>>;
async fn get_user(&self, id: &Uuid) -> Result<StoredUserRecord>;
```

---

## 7) Diferencia clave: DTOs vs Records

```text
application/user/dto/input
  Datos que entran a casos de uso.

application/user/dto/output
  Datos que salen de casos de uso.

ports/out/user/records
  Datos que cruzan la frontera hacia/desde el repositorio.
```

Ejemplo:

```text
CreateUserInput
  Entrada al caso de uso. Tiene password cruda.

CreateUserRecord
  Salida hacia repository. Tiene password_hash.

StoredUserRecord
  Vuelve desde repository. Representa datos persistidos, pero no es SQLx entity.

UserOutput
  Sale de application hacia presentation. No tiene password.
```

---

## 8) Flujo final implementado en el DDD

```text
presentation/http CreateUserDto
  -> application CreateUserInput
  -> use_cases/create_user.rs
  -> domain User temporal
  -> policies/password_policy.rs
  -> ports/out CreateUserRecord
  -> IUserRepository
  -> infrastructure UserEntity
  -> ports/out StoredUserRecord
  -> application UserOutput
  -> presentation/http UserResponseDto
```

---

## 9) Archivos guía importantes

Leer estos archivos antes de continuar:

```text
/Users/benji/Documents/RUST/aprendisajeRS/rust-api-ddd/src/application/APPLICATION_USER_DTO_USE_CASES_GUIDE.md
/Users/benji/Documents/RUST/aprendisajeRS/rust-api-ddd/src/ports/out/OUTBOUND_PORT_RECORDS_GUIDE.md
/Users/benji/Documents/RUST/aprendisajeRS/rust-api-ddd/POSIBLES_AGREGADOS_DDD_PROYECTO_GRANDE.md
```

---

## 10) Estado de compilacion

Despues de los cambios en el DDD se ejecuto:

```bash
cargo check
```

Resultado: compila correctamente. Quedan warnings, pero no errores.

Warnings esperados:

```text
unused imports
unused variable e
password_hash never read
created_at never read
```

No son bloqueantes.

---

## 11) Como continuar en el backend real

El backend real esta en:

```text
/Users/benji/Documents/RUST/backend
```

Antes de copiar cambios, revisar el DDD y seguir este orden:

```text
1. Application DTOs input/output.
2. Policies.
3. Use cases.
4. Convertir user_service.rs en fachada.
5. Cambiar ports/in para usar CreateUserInput y UserOutput.
6. Crear ports/out/user/records.
7. Cambiar IUserRepository para usar CreateUserRecord y StoredUserRecord.
8. Adaptar infrastructure repository/data_store/entity.
9. Adaptar presentation HTTP DTO conversions.
10. Ejecutar cargo check.
```

---

## 12) Regla de personalidad/trabajo para el nuevo chat

El usuario quiere aprender, no solo que se le haga todo.

Reglas:

```text
Explicar claro y por pasos.
No editar codigo si el usuario no lo pide explicitamente.
Si se edita, decir que se va a editar antes.
Preferir cambios pequenos y verificables.
Usar el DDD como referencia antes de tocar backend.
No mezclar responsabilidades entre capas.
```

Tono:

```text
Calido, paciente, directo, como instructor tecnico.
No sobrecomplicar, pero tampoco esconder la arquitectura.
```
