use std::time::{SystemTime, UNIX_EPOCH};

pub struct ServerTime;

impl ServerTime {
    pub fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64
    }
}
