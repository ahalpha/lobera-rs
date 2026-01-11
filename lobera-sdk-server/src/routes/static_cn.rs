use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose};
use lobera_commons::crypto::abcustom::ABCustom;
use serde_json::json;

use crate::base::config::CONFIG;

// cdn.megagamelog.com/cross/*subpath
// cdn2.megagamelog.com/cross/*subpath
pub async fn release_handler(Path(subpath): Path<String>) -> Response {
    let file_path = format!("data/static/{}", subpath);
    match std::fs::read(&file_path) {
        Ok(content) => {
            let mut headers = HeaderMap::new();
            headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
            // Optionally set content type based on extension
            if file_path.ends_with(".json") {
                headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
            } else {
                headers.insert(
                    header::CONTENT_TYPE,
                    "application/octet-stream".parse().unwrap(),
                );
            }
            (StatusCode::OK, headers, content).into_response()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

// cdn.megagamelog.com/cross/release/sl.json
// cdn2.megagamelog.com/cross/release/sl.json
pub async fn release_sl() -> impl IntoResponse {
    let data = json!({
        "code": 0,
        "data": [
            {
                "index": 2,
                "id": 1,
                "description": "少女前线",
                "serverName": "阿尔法南梁服",
                "state": 1,
                "serverIp": [format!("{}:{}", CONFIG.gameserver_host, CONFIG.gameserver_port)],
                "port": CONFIG.gameserver_port,
                "resDir": "release",
                "channel": 0,
                "gmSvrIp": CONFIG.gameserver_host,
                "gmSvrPort": CONFIG.gameserver_port,
                "webIp": "http://notice.megagamelog.com",
                "webPort": 80,
                "processTag": "/home/cross/project/",
                "svrDir": "/home/cross/project/server",
                "SDK_URL": "http://sdk.megagamelog.com/php/sdk/",
                "zone": 0,
                "bgSelect": 0,
                "is_open_white": 0,
                "modTime": "2023-12-26 16:00:34",
                "mainntentFile": "",
            }
        ],
    });

    let body_str = serde_json::to_string(&data).unwrap();
    let b64 = general_purpose::STANDARD.encode(body_str.as_bytes());
    let encrypted = ABCustom::ddoo_eennccyypptt_ssttrr(&b64);

    let mut headers = HeaderMap::new();
    headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
    (StatusCode::OK, headers, encrypted)
}

// notice.megagamelog.com/php/res/release/boardNotice/android/official/boardNotice.json
pub async fn board_notice() -> impl IntoResponse {
    let path = "data/static/boardNotice.json";
    let content = std::fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());

    let mut headers = HeaderMap::new();
    headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    (StatusCode::OK, headers, content)
}

// notice.megagamelog.com/php/res/release/activitySkip.json
pub async fn activity_skip() -> impl IntoResponse {
    let path = "data/static/activitySkip.json";
    let content = std::fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());

    let mut headers = HeaderMap::new();
    headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    (StatusCode::OK, headers, content)
}
