use actix_web::{HttpResponse, Responder, get};
use serde::{Serialize, Deserialize};
use crate::response::ApiResponse;
use std::time::Duration;
use crate::{START_TIME, APP_NAME, APP_VERSION};

// Struktur untuk data asli
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub manifest: Manifest,
    pub up_time: String,
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub app: String,
    pub version: String,
}



impl AppInfo {
    pub fn new() -> Self {
        let elapsed = START_TIME.elapsed();
        
        Self {
            manifest: Manifest {
                app: APP_NAME.to_string(),
                version: APP_VERSION.to_string(),
            },
            up_time: format_duration(elapsed),
        }
    }
}

#[get("/")]
pub async fn manifest_app() -> impl Responder {
    let info = AppInfo::new();

    // Menggunakan wrapper untuk response
    let response = ApiResponse::ok(info);
    
    HttpResponse::Ok().json(response)
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else if seconds < 86400 {
        format!("{}h {}m {}s", 
            seconds / 3600,
            (seconds % 3600) / 60,
            seconds % 60
        )
    } else {
        format!("{}d {}h {}m {}s",
            seconds / 86400,
            (seconds % 86400) / 3600,
            (seconds % 3600) / 60,
            seconds % 60
        )
    }
}
