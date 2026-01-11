use super::*;

#[test]
fn test_time() {
    print!("{:?}", time::ServerTime::now_ms());
}