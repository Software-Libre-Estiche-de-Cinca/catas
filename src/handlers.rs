use crate::db::User;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

#[utoipa::path(
    get,
    path = "/{name}",
    responses(
        (status = 200, description = "Retorna el nombre recibido", body = String)
    ),
    params(
        ("name" = String, Path, description = "El nombre a retornar")
    )
)]
pub async fn hello_world(Path(name): Path<String>) -> String {
    info!("Request recibido para: {}", name);
    name
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "La aplicación está saludable", body = String),
        (status = 500, description = "Error interno del servidor", body = String)
    )
)]
pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    info!("Request recibido para health check");
    let result = tokio::task::spawn_blocking(move || {
        let conn = state.db.get()?; // sacar conexión del pool

        let value: i32 = conn.query_row("SELECT 1", [], |row| row.get(0))?;

        Ok::<i32, anyhow::Error>(value)
    })
    .await;

    match result {
        Ok(Ok(1)) => (StatusCode::OK, "ok"),
        Ok(Ok(_)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error inesperado en la consulta de salud",
        ),
        Ok(Err(e)) => {
            error!("Error de base de datos en health check: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Error de base de datos")
        }
        Err(e) => {
            error!("Error al ejecutar la tarea de health check: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del servidor",
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login exitoso", body = LoginResponse),
        (status = 401, description = "Credenciales inválidas"),
        (status = 500, description = "Error al acceder a la base de datos")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    info!("Intento de login para el usuario: {}", payload.username);

    // El bloque spawn_blocking se ejecuta en otro hilo para no bloquear el reactor async
    // Necesita clonar los valores que va a usar, no se pueden prestar
    let username = payload.username.clone();
    let password = payload.password.clone();

    let user = tokio::task::spawn_blocking(move || {
        let conn = state.db.get()?;
        User::fetch_from_db(&conn, &username)
    })
    .await
    // Spawn_blocking retorna Result<Result<T, E>, JoinError>.
    // Necesitamos separar ambos niveles de error, no está mal
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = user.ok_or_else(|| {
        info!("Usuario no encontrado: {}", payload.username);
        StatusCode::UNAUTHORIZED
    })?;

    if !user
        .check_password(password.as_bytes())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        info!(
            "Credenciales inválidas para el usuario: {}",
            payload.username
        );
        return Err(StatusCode::UNAUTHORIZED);
    }

    info!("Login exitoso para el usuario: {}", payload.username);

    // Devolvemos un token dummy (pendiente de implementar generación de tokens)
    Ok(Json(LoginResponse {
        token: "dummy_token".to_string(),
    }))
}
