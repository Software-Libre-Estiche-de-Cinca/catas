use crate::{handlers, AppState};
use axum::routing::post;
use axum::{routing::get, Router};
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Define el doc: incluye tus handlers documentados con #[utoipa::path]
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::hello_world,
        handlers::health,
        handlers::login
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
        .route("/{name}", get(handlers::hello_world))
        .route("/health", get(handlers::health))
        .route("/login", post(handlers::login))
        // Añadir SwaggerUI en /swagger-ui
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .with_state(app_state)
}
