use crate::handlers::health;
use crate::handlers::users::login as user_login;
use crate::AppState;
use axum::routing::post;
use axum::{routing::get, Router};
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Define el doc: incluye tus handlers documentados con #[utoipa::path]
#[derive(OpenApi)]
#[openapi(
    paths(
        health::health,
        user_login::login
    ),
    tags(
        (name = "api", description = "API endpoints")
    )
)]
struct ApiDoc;

pub fn create_routes(app_state: AppState) -> Router {
    debug!("Configurando rutas de la aplicación");

    // Generar el OpenAPI spec
    let openapi = ApiDoc::openapi();

    Router::new()
        .route("/health", get(health::health))
        .route("/login", post(user_login::login))
        // Añadir SwaggerUI en /swagger-ui
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .with_state(app_state)
}
