use super::*;
use serde_json::json;

#[test]
fn test_data() {
    let _data = TABLES
        .card_data
        .find_one(&["id".to_string()], &[json!(10230)]);
    println!("{:?}", _data)
}
