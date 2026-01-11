use super::*;
use lobera_ivproto::{IvMessage, IvReader, IvWriter};

#[test]
fn test1() {
    let mut data = schemas::sShopCommodityCfg::default();
    data.nDiscountStart = Option::Some(3);
    data.nDiscountEnd = Option::Some(11);
    println!("data: {:?}", data);

    let mut writer = IvWriter::new();
    data.encode(&mut writer).unwrap();
    let buf = writer.buf;
    println!("buf: {:?}", buf);

    let mut reader = IvReader::new(&buf);
    let decoded = schemas::sShopCommodityCfg::decode(&mut reader).unwrap();
    println!("decoded: {:?}", decoded);
}

#[test]
fn test_game_msg_derive() {
    assert_eq!(schemas::PlayerProto_Sign::MSG_NO, 1001);
}

#[test]
fn test_game_msg_dispatch() {
    let msg = create_message_by_id(1001).expect("1");
    assert_eq!(msg.get_msg_no(), 1001);

    let sign_msg = msg
        .as_any()
        .downcast_ref::<schemas::PlayerProto_Sign>()
        .unwrap();
    assert!(sign_msg.sign.is_none());
}

#[test]
fn test_dynamic_decode() {
    let mut data = schemas::PlayerProto_Sign::default();
    data.sign = Some("hello".to_string());

    let mut writer = IvWriter::new();
    data.encode(&mut writer).unwrap();
    let buf = writer.buf;

    let mut msg = create_message_by_id(1001).unwrap();
    let mut reader = IvReader::new(&buf);
    msg.decode_into(&mut reader).unwrap();

    let sign_msg = msg
        .as_any()
        .downcast_ref::<schemas::PlayerProto_Sign>()
        .unwrap();
    assert_eq!(sign_msg.sign.as_deref(), Some("hello"));
}
