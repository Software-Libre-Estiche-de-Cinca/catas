use std::io;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Inicializa el sistema de logging con salida a consola y archivo
/// Los logs se guardan en el directorio ./logs con rotación diaria
pub fn init_logger() {
    // Crear el directorio de logs si no existe
    std::fs::create_dir_all("logs").expect("No se pudo crear el directorio de logs");

    // Configurar el appender de archivos con rotación diaria
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "catas.log");

    // Configurar el filtro de niveles (por defecto INFO, puede ser sobreescrito con RUST_LOG)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Layer para la consola (con colores y formato legible)
    let console_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(true)
        .with_line_number(true);

    // Layer para el archivo (formato más detallado)
    let file_layer = fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false) // Sin colores en archivo
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    // Combinar ambos layers
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();
}
