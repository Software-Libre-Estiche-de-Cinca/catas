use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::{error, info};

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
