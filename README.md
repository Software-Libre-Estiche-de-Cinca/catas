# No se hacer Readmes
Se lo pides a una IA que para eso están.
Y ya está.

Lo basico para ejecutar:

```bash
cargo run
```

Luego lo tipico de Rust para saber si has hecho algo o todo mal:

```bash
cargo check --all
cargo clippy --all
cargo build --all --release
```

Y si, tienes que instalar Rust y Cargo, obvio.

## Estructura del Proyecto

```
src/
├── handlers/             # Controladores HTTP
│   ├── health.rs         # Health check del servidor
│   └── users/            # Funcionalidades relacionadas con usuarios
│       └── login.rs      # Login de usuarios
├── models/               # Modelos de datos
│   └── users/            # Modelos relacionados con usuarios
│       └── login.rs      # Estructuras LoginRequest, LoginResponse
├── db.rs                 # Capa de acceso a base de datos
├── routes.rs             # Configuración de rutas y OpenAPI
├── logger.rs             # Sistema de logging
└── main.rs               # Punto de entrada de la aplicación
```

## Tutorial: Añadir una Nueva Ruta a la API (Paso a Paso)


### Paso 1: Crea el Modelo

Crea un archivo `src/models/...` si es que te hace falta algun modelo. Un ejemplo:

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
}
```

### Paso 2: Actualiza el módulo de modelos

Edita `src/main.rs` y agrega el nuevo modelo:

```rust
mod models {
    pub mod users {
        pub mod login;
        pub mod profile;  // ← AÑADE ESTA LÍNEA
    }
}
```

### Paso 3: Crea el Handler

Crea un archivo `src/handlers/...`. Este si es obligatorio, es donde van las funciones asociadas a las rutas. Un ejemplo que no hace un carajo:

```rust
use crate::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use tracing::info;
use crate::models::users::profile::UserProfile;

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Perfil del usuario", body = UserProfile),
        (status = 404, description = "Usuario no encontrado")
    ),
    params(
        ("id" = i32, Path, description = "ID del usuario")
    )
)]
pub async fn get_profile(
    State(_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserProfile>, StatusCode> {
    info!("Obteniendo perfil del usuario: {}", id);
    
    Ok(Json(UserProfile {
        id,
        username: "usuario_ejemplo".to_string(),
        email: "usuario@example.com".to_string(),
    }))
}
```

### Paso 4: Actualiza el módulo de handlers

Edita `src/main.rs` y agrega el nuevo handler, igual que hiciste con los modelos:

```rust
mod handlers {
    pub mod health;
    pub mod users {
        pub mod login;
        pub mod profile;  // ← AÑADE ESTA LÍNEA
    }

    pub use health::health;
    pub use users::login::login;
    pub use users::profile::get_profile;  // ← AÑADE ESTA LÍNEA
}
```

### Paso 5: Registra la ruta en routes.rs

Edita `src/routes.rs` y:

1. Importa el nuevo handler:
```rust
use crate::handlers::users::profile;
```

2. Añade el path al macro OpenAPI:
```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        health::health,
        user_login::login,
        profile::get_profile,  // ← AÑADE ESTA LÍNEA
    ),
    tags(
        (name = "api", description = "API endpoints")
    )
)]
```

3. Registra la ruta en el router:
```rust
Router::new()
    .route("/health", get(health::health))
    .route("/login", post(user_login::login))
    .route("/users/:id", get(profile::get_profile))  // ← AÑADE ESTA LÍNEA
    // ...resto del código
```



## Mas cosas

La aplicación incluye SwaggerUI para documentar los endpoints de la API.

Una vez iniciada la aplicación con `cargo run`, puedes acceder a:

- **SwaggerUI**: http://127.0.0.1:3000/swagger-ui/
- **OpenAPI Spec**: http://127.0.0.1:3000/api-docs/openapi.json

