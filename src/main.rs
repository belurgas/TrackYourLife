mod config;
mod error;
mod models;
mod handlers;

use std::sync::{Arc, Mutex};

use config::Config;
use handlers::{http_server, websocket::{self, start_websocket_server}};
use tokio::join;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Логирование
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Загружаем конфиг
    let config = Config::from_env();
    tracing::info!("Сервер запущен на {}", config.addr);

    // Общая БД в памяти
    let db: Arc<Mutex<Vec<models::WaterEntry>>> = Arc::new(Mutex::new(vec![]));

    // Запуск HTTP сервера
    let db_http = db.clone();
    let http_server = http_server::start_http_server(db_http);

    // Запуск WebSocket сервера
    let db_ws = db.clone();
    let ws_server = websocket::start_websocket_server(db_ws);

    let (_first, _second) = join!(http_server, ws_server);

    loop {
        tokio::signal::ctrl_c().await?;
    }

    Ok(())
}