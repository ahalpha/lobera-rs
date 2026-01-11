use bytes::{Buf, BufMut, BytesMut};

pub struct PacketParser {
    is_server: bool,
    buf: BytesMut,
}

impl PacketParser {
    pub fn new(is_server: bool) -> Self {
        Self {
            is_server,
            buf: BytesMut::with_capacity(4096),
        }
    }

    pub fn add_raw(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    pub fn parse_packet(&mut self) -> Option<(u16, Vec<u8>)> {
        let min_header_size = if self.is_server { 11 } else { 3 };
        if self.buf.len() < min_header_size {
            return None;
        }

        let mut reader = &self.buf[..];
        let packet_size = reader.get_u16();

        if self.buf.len() < (packet_size + 2) as usize {
            return None;
        }

        let _packet_flag = reader.get_u8();
        let packet_buffer = if self.is_server {
            let _server_time = reader.get_u64();
            &self.buf[11..(packet_size + 2) as usize]
        } else {
            &self.buf[3..(packet_size + 2) as usize]
        };

        let (msg_no, msg_body) = self.parse_packet_buffer(packet_buffer);

        self.buf.advance((packet_size + 2) as usize);

        Some((msg_no, msg_body))
    }

    fn parse_packet_buffer(&self, packet_buffer: &[u8]) -> (u16, Vec<u8>) {
        let mut reader = packet_buffer;
        let _msg_size = reader.get_u16_le();
        let msg_no = reader.get_u16_le();
        let msg_body = reader.to_vec();
        (msg_no, msg_body)
    }
}

pub struct PacketBuilder;

impl PacketBuilder {
    pub fn build_packet(
        msg_no: u16,
        msg_body: &[u8],
        flag: u8,
        server_time: Option<u64>,
    ) -> Vec<u8> {
        let msg_size = (msg_body.len() + 2) as u16;

        let mut inner_buf = Vec::new();
        inner_buf.put_u16_le(msg_size);
        inner_buf.put_u16_le(msg_no);
        inner_buf.extend_from_slice(msg_body);

        let mut buffer = Vec::new();
        buffer.put_u8(flag);
        if let Some(time) = server_time {
            buffer.put_u64(time);
        }
        buffer.extend_from_slice(&inner_buf);

        let packet_size = buffer.len() as u16;
        let mut final_buf = Vec::with_capacity(buffer.len() + 2);
        final_buf.put_u16(packet_size);
        final_buf.extend_from_slice(&buffer);

        final_buf
    }
}
