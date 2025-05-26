use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaterEntry {
    pub id: String,
    pub user_id: String,
    pub amount_ml: i32,
    pub timestamp: i64,
}

// БД в памяти
use std::sync::{Arc, Mutex};

pub type Db = Arc<Mutex<Vec<WaterEntry>>>;