use thiserror::Error;
use tokio_tungstenite::tungstenite;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Ошибка WebSocket: {0}")]
    WsError(#[from] tungstenite::Error),

    #[error("Ошибка ввода/вывода: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Неизвестная ошибка")]
    Unknown,
}