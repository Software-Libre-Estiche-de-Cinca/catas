mod db;
mod handlers;
mod logger;
mod routes;

use crate::db::DbPool;

use tracing::{error, info};

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}

#[tokio::main]
async fn main() {
    // Inicializar el sistema de logging 
    logger::init_logger();

    info!("Iniciando aplicación de Catas");

    // Inicializar la base de datos
    info!("Inicializando base de datos...");
    let pool = match db::initialize_database_pool() {
        Ok(p) => p,
        Err(e) => {
            error!("Error al inicializar la base de datos: {}", e);
            std::process::exit(1);
        }
    };
    let state = AppState { db: pool };
    info!("Base de datos inicializada correctamente");

    // Si no hay errores, continuar con el servidor
    let app = routes::create_routes(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    info!("Servidor escuchando en http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
