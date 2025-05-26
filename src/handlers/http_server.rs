use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tracing::info;

use crate::models::{Db, WaterEntry};



#[derive(Deserialize)]
pub struct AddWaterRequest {
    user_id: String,
    amount_ml: i32,
}

async fn add_water(
    db: web::Data<Db>,
    body: web::Json<AddWaterRequest>,
) -> impl Responder {
    let entry = WaterEntry {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: body.user_id.clone(),
        amount_ml: body.amount_ml,
        timestamp: chrono::Utc::now().timestamp(),
    };

    db.lock().unwrap().push(entry.clone());
    info!("/post");
    HttpResponse::Created().json(entry)
}

async fn get_water(db: web::Data<Db>) -> impl Responder {
    let entries = db.lock().unwrap().clone();
    info!("/get");
    HttpResponse::Ok().json(entries)
}

pub async fn start_http_server(db: Db) -> std::io::Result<()> {
    info!("HTTP сервер запущен на http://127.0.0.1:9081");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/water", web::post().to(add_water))
            .route("/water", web::get().to(get_water))
    })
    .bind("127.0.0.1:9081")?
    .run()
    .await
}