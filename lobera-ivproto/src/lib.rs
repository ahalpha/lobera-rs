use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Cursor, Read, Write};

#[cfg(feature = "derive")]
pub use lobera_ivproto_derive::Message;

pub trait IvMessage: Sized + Default {
    fn decode(reader: &mut IvReader) -> io::Result<Self>;
    fn encode(&self, writer: &mut IvWriter) -> io::Result<()>;
}

pub struct IvReader<'a> {
    pub cursor: Cursor<&'a [u8]>,
}

impl<'a> IvReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        self.cursor.read_u8()
    }
    pub fn read_i16(&mut self) -> io::Result<i16> {
        self.cursor.read_i16::<LittleEndian>()
    }
    pub fn read_u16(&mut self) -> io::Result<u16> {
        self.cursor.read_u16::<LittleEndian>()
    }
    pub fn read_i32(&mut self) -> io::Result<i32> {
        self.cursor.read_i32::<LittleEndian>()
    }
    pub fn read_u32(&mut self) -> io::Result<u32> {
        self.cursor.read_u32::<LittleEndian>()
    }
    pub fn read_i64(&mut self) -> io::Result<i64> {
        self.cursor.read_i64::<LittleEndian>()
    }
    pub fn read_f32(&mut self) -> io::Result<f32> {
        self.cursor.read_f32::<LittleEndian>()
    }
    pub fn read_f64(&mut self) -> io::Result<f64> {
        self.cursor.read_f64::<LittleEndian>()
    }
    pub fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.cursor.read_u8()? != 0)
    }

    pub fn read_string(&mut self) -> io::Result<String> {
        let len = self.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        self.cursor.read_exact(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }

    pub fn read_json(&mut self) -> io::Result<String> {
        Ok(self.read_string()?)
    }
}

pub struct IvWriter {
    pub buf: Vec<u8>,
}

impl IvWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn write_u8(&mut self, v: u8) -> io::Result<()> {
        self.buf.write_u8(v)
    }
    pub fn write_i16(&mut self, v: i16) -> io::Result<()> {
        self.buf.write_i16::<LittleEndian>(v)
    }
    pub fn write_u16(&mut self, v: u16) -> io::Result<()> {
        self.buf.write_u16::<LittleEndian>(v)
    }
    pub fn write_i32(&mut self, v: i32) -> io::Result<()> {
        self.buf.write_i32::<LittleEndian>(v)
    }
    pub fn write_u32(&mut self, v: u32) -> io::Result<()> {
        self.buf.write_u32::<LittleEndian>(v)
    }
    pub fn write_i64(&mut self, v: i64) -> io::Result<()> {
        self.buf.write_i64::<LittleEndian>(v)
    }
    pub fn write_f32(&mut self, v: f32) -> io::Result<()> {
        self.buf.write_f32::<LittleEndian>(v)
    }
    pub fn write_f64(&mut self, v: f64) -> io::Result<()> {
        self.buf.write_f64::<LittleEndian>(v)
    }
    pub fn write_bool(&mut self, v: bool) -> io::Result<()> {
        self.buf.write_u8(if v { 1 } else { 0 })
    }

    pub fn write_string(&mut self, v: &str) -> io::Result<()> {
        let bytes = v.as_bytes();
        self.write_u16(bytes.len() as u16)?;
        self.buf.write_all(bytes)
    }

    pub fn write_json(&mut self, v: &str) -> io::Result<()> {
        self.write_string(v)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}
