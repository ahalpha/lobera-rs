mod base {
    pub mod config;
}
mod routes {
    pub mod sdk_cn;
    pub mod static_cn;
}

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(&base::config::CONFIG.log));

    let app = Router::new()
        .route(
            "/event-sdk.megagamelog.com/data/event/g/1001",
            post(routes::sdk_cn::data_event),
        )
        .route(
            "/account-sdk.megagamelog.com/sdk/init/g/1001",
            post(routes::sdk_cn::sdk_init),
        )
        .route(
            "/account-sdk.megagamelog.com/sdk/sku/list/zip/g/1001",
            post(routes::sdk_cn::sdk_sku_list),
        )
        .route(
            "/account-sdk.megagamelog.com/sdk/user/login/g/1001",
            post(routes::sdk_cn::sdk_user_login),
        )
        .route(
            "/sdk.megagamelog.com/php/sdk/centerWeb/login.php",
            post(routes::sdk_cn::sdk_centerweb_login),
        )
        .route(
            "/ext-sdk.megagamelog.com/sdk/role/sync/g/1001",
            post(routes::sdk_cn::sdk_role_sync),
        )
        .route(
            "/ext-sdk.megagamelog.com/sdk/heartbeat/g/1001",
            post(routes::sdk_cn::sdk_heartbeat),
        )
        .route(
            "/cdn.megagamelog.com/cross/release/sl.json",
            get(routes::static_cn::release_sl),
        )
        .route(
            "/cdn2.megagamelog.com/cross/release/sl.json",
            get(routes::static_cn::release_sl),
        )
        .route(
            "/notice.megagamelog.com/php/res/release/boardNotice/android/official/boardNotice.json",
            get(routes::static_cn::board_notice),
        )
        .route(
            "/notice.megagamelog.com/php/res/release/activitySkip.json",
            get(routes::static_cn::activity_skip),
        )
        .route(
            "/cdn.megagamelog.com/cross/*subpath",
            get(routes::static_cn::release_handler),
        )
        .route(
            "/cdn2.megagamelog.com/cross/*subpath",
            get(routes::static_cn::release_handler),
        )
        .layer(TraceLayer::new_for_http());

    let addr = format!(
        "{}:{}",
        base::config::CONFIG.host,
        base::config::CONFIG.port
    );
    log::info!("SDKServer listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
