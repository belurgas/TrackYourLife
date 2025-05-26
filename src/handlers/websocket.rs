use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use tracing::info;

use crate::models::Db;



pub async fn start_websocket_server(db: Db) {
    let listener = match tokio::net::TcpListener::bind("127.0.0.1:8087").await {
        Ok(listener) => {
            info!("WebSocket сервер запущен на ws://127.0.0.1:8087");
            listener
        }
        Err(e) => panic!("Не удалось запустить WebSocket сервер: {}", e),
    };

    while let Ok((stream, _)) = listener.accept().await {
        let db = db.clone();
        tokio::spawn(async move {
            let mut ws_stream = accept_async(stream)
                .await
                .expect("Ошибка установки WS-соединения");

            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        info!("Получено сообщение: {}", text);
                        ws_stream.send(Message::Text(format!("Эхо: {}", text).into())).await.unwrap();
                    }
                    Ok(Message::Close(_)) => break,
                    Err(e) => {
                        eprintln!("Ошибка: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });
    }
}