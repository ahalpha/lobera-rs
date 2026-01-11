mod base {
    pub mod app;
    pub mod config;
    pub mod packet;
}
mod handles {
    pub mod game_server;
}
mod func {
    mod abattoir;
    mod ability;
    mod achievement;
    mod army;
    mod badged;
    mod building;
    mod client;
    mod dorm;
    mod equip;
    mod exploration;
    mod fight;
    mod friend;
    mod guild;
    mod love_plus;
    mod mail;
    mod operate_active;
    mod permit;
    mod player;
    mod questionnaire;
    mod regression;
    mod shop;
    mod summer;
    mod task;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(&base::config::CONFIG.log));

    let addr = format!("{}:{}", base::config::CONFIG.host, base::config::CONFIG.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    log::info!("GameServer open on {}", addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handles::game_server::handle_connection(socket, addr).await {
                log::error!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}
