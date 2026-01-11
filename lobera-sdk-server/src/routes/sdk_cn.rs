use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use base64::{Engine as _, engine::general_purpose};
use lobera_commons::crypto::mjsdk::MJSdk;
use lobera_commons::time::ServerTime;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Write;

// event-sdk.megagamelog.com/data/event/g/1001
pub async fn data_event() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );
    (StatusCode::OK, headers, "200")
}

// account-sdk.megagamelog.com/sdk/init/g/1001
pub async fn sdk_init(body: Bytes) -> impl IntoResponse {
    let _params = MJSdk::parse_request_data(&body);
    // log::info!("sdk_init_req: {:?}", String::from_utf8_lossy(&_params));

    let mut data = BTreeMap::new();
    data.insert("timeSecs".to_string(), json!(ServerTime::now()));
    data.insert("res".to_string(), json!({"code": 200, "msg": "OK"}));
    data.insert("adName".to_string(), json!("TapTap"));
    data.insert("turboOpenIds".to_string(), json!(true));
    data.insert(
        "deletionUrl".to_string(),
        json!("https://account-sdk.megagamelog.com/webui/index.html#/deletionPotocol"),
    );
    data.insert("heartInterval".to_string(), json!(60));
    data.insert("eventInterval".to_string(), json!(20));
    data.insert("eventSize".to_string(), json!(100));
    data.insert(
        "csInfo1".to_string(),
        json!("https://ext-sdk.megagamelog.com/webui/index.html#/cs/home"),
    );
    data.insert(
        "csInfo2".to_string(),
        json!("<a href='https://sdk-cdn.megagamelog.com/page/cs.html?v=1.0.0'>客服中心</a>"),
    );
    data.insert("skVer".to_string(), json!(1));
    data.insert("uiVer".to_string(), json!(29));
    data.insert("thirdPartyLogins".to_string(), json!("13"));

    let sign = MJSdk::sign_data(&data);
    data.insert("sign".to_string(), json!(sign));

    let body_str = serde_json::to_string(&data).unwrap();
    let encrypted_body = MJSdk::build_response_data(body_str.as_bytes());

    let mut headers = HeaderMap::new();
    headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
    (StatusCode::OK, headers, encrypted_body)
}

// account-sdk.megagamelog.com/sdk/sku/list/zip/g/1001
pub async fn sdk_sku_list(body: Bytes) -> impl IntoResponse {
    let _params = MJSdk::parse_request_data(&body);

    let sku_path = "data/static/sku_data.json";
    let sku_data: Value = match std::fs::read_to_string(sku_path) {
        Ok(s) => serde_json::from_str(&s).unwrap_or(json!({})),
        Err(_) => json!({}),
    };

    let sku_json = serde_json::to_string(&sku_data).unwrap();
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(sku_json.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();

    let xored: Vec<u8> = compressed.iter().map(|&d| d ^ 0x80).collect();
    let sku_zip = general_purpose::STANDARD.encode(xored);

    let mut data = BTreeMap::new();
    data.insert("timeSecs".to_string(), json!(ServerTime::now()));
    data.insert("res".to_string(), json!({"code": 200, "msg": "OK"}));
    data.insert("skuZip".to_string(), json!(sku_zip));

    let sign = MJSdk::sign_data(&data);
    data.insert("sign".to_string(), json!(sign));

    let body_str = serde_json::to_string(&data).unwrap();
    let encrypted_body = MJSdk::build_response_data(body_str.as_bytes());

    let mut headers = HeaderMap::new();
    headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
    (StatusCode::OK, headers, encrypted_body)
}

// account-sdk.megagamelog.com/sdk/user/login/g/1001
pub async fn sdk_user_login(body: Bytes) -> impl IntoResponse {
    let _params = MJSdk::parse_request_data(&body);

    let mut data = BTreeMap::new();
    data.insert("timeSecs".to_string(), json!(ServerTime::now()));
    data.insert("res".to_string(), json!({"code": 200, "msg": "OK"}));
    data.insert("uid".to_string(), json!("1000000"));
    data.insert("phone".to_string(), json!(18800000000u64));
    data.insert("needUpgrade".to_string(), json!(false));
    data.insert("firstJoin".to_string(), json!(false));
    data.insert("firstJoinSecs".to_string(), json!(0));
    data.insert("lastLoginSes".to_string(), json!(ServerTime::now()));
    data.insert("openId".to_string(), json!("P1"));
    data.insert("authToken".to_string(), json!("1"));
    data.insert("secondToken".to_string(), json!("2"));
    data.insert("idCard".to_string(), json!("******************"));
    data.insert("acctType".to_string(), json!(1));

    let sign = MJSdk::sign_data(&data);
    data.insert("sign".to_string(), json!(sign));

    let body_str = serde_json::to_string(&data).unwrap();
    let encrypted_body = MJSdk::build_response_data(body_str.as_bytes());

    let mut headers = HeaderMap::new();
    headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
    (StatusCode::OK, headers, encrypted_body)
}

// sdk.megagamelog.com/php/sdk/centerWeb/login.php
pub async fn sdk_centerweb_login(_body: Bytes) -> impl IntoResponse {
    let data = json!({
        "opwd": "3",
        "oacc_name": "18800000000",
        "uid": "100000000",
        "open_id": 10000,
        "acc_name": "1000000",
        "bnew": 0,
        "code": 0,
        "channel": 0
    });

    let body_str = serde_json::to_string(&data).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
    (StatusCode::OK, headers, body_str)
}

// ext-sdk.megagamelog.com/sdk/role/sync/g/1001
pub async fn sdk_role_sync(_body: Bytes) -> impl IntoResponse {
    let mut data = BTreeMap::new();
    data.insert("timeSecs".to_string(), json!(ServerTime::now()));
    data.insert("res".to_string(), json!({"code": 200, "msg": "OK"}));

    let sign = MJSdk::sign_data(&data);
    data.insert("sign".to_string(), json!(sign));

    let body_str = serde_json::to_string(&data).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
    (StatusCode::OK, headers, body_str)
}

// ext-sdk.megagamelog.com/sdk/heartbeat/g/1001
pub async fn sdk_heartbeat(_body: Bytes) -> impl IntoResponse {
    let mut data = BTreeMap::new();
    data.insert("timeSecs".to_string(), json!(ServerTime::now()));
    data.insert("res".to_string(), json!({"code": 200, "msg": "OK"}));

    let sign = MJSdk::sign_data(&data);
    data.insert("sign".to_string(), json!(sign));

    let body_str = serde_json::to_string(&data).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Server", "nginx/1.18.0 (Ubuntu)".parse().unwrap());
    (StatusCode::OK, headers, body_str)
}
