use anyhow::Result;
use lobera_game_msg::create_message_by_id;
use lobera_ivproto::{IvReader, IvWriter};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::base::app::APP;
use crate::base::packet::{PacketBuilder, PacketParser};
use lobera_commons::time::ServerTime;

pub async fn handle_connection(mut socket: TcpStream, addr: SocketAddr) -> Result<()> {
    log::info!("Connecting to the Client ({})", addr);
    let mut parser = PacketParser::new(false);
    let mut buf = [0u8; 1024];
    let mut uid = 0u32;

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                log::error!("Error reading from socket: {}", e);
                break;
            }
        };

        parser.add_raw(&buf[..n]);

        while let Some((msg_no, msg_body)) = parser.parse_packet() {
            let msg_box = if let Some(mut msg) = create_message_by_id(msg_no) {
                let mut reader = IvReader::new(&msg_body);
                if let Err(e) = msg.decode_into(&mut reader) {
                    log::error!("ProtoParseError: {} {:?}", msg_no, e);
                    continue;
                }
                log::debug!("[IN] -> ({}) {:?}", msg_no, msg);
                msg
            } else {
                log::debug!("[IN] -> ({}) Unknown msg_no", msg_no);
                continue;
            };

            let result_msgs = match APP.handle_message(msg_no, uid, msg_box).await {
                Ok(res) => {
                    if msg_no == 2001 {
                        uid = 10000;
                    }
                    res
                }
                Err(e) => {
                    log::error!("Error handling message {}: {}", msg_no, e);
                    vec![None]
                }
            };

            for res_data in result_msgs {
                let (res_id, res_body) = if let Some(data) = res_data {
                    let res_id = data.get_msg_no();
                    log::debug!("[OUT] <- ({}) {:?}", res_id, data);
                    let mut writer = IvWriter::new();
                    if let Err(e) = data.encode_to(&mut writer) {
                        log::error!("ProtoBuildError: {} {:?}", res_id, e);
                        (res_id, vec![0])
                    } else {
                        (res_id, writer.into_bytes())
                    }
                } else {
                    (msg_no + 1, vec![0])
                };

                let packet =
                    PacketBuilder::build_packet(res_id, &res_body, 3, Some(ServerTime::now_ms()));
                socket.write_all(&packet).await?;
                socket.flush().await?;
            }
        }
    }

    log::info!("Connection closed ({})", addr);
    Ok(())
}