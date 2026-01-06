use crate::db::User;
use crate::models::users::login::{LoginRequest, LoginResponse};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use tracing::info;

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
