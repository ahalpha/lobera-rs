pub use lobera_game_msg_derive::{MsgNo, func_reg};
pub mod schemas {
    #![allow(warnings)]
    include!(concat!("../generated/game_msg.rs"));
}

pub trait GameMessage: lobera_ivproto::IvMessage + AnyGameMessage {
    const MSG_NO: u16;
}

pub trait AnyGameMessage: Send + Sync + std::fmt::Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn get_msg_no(&self) -> u16;
    fn decode_into(&mut self, reader: &mut lobera_ivproto::IvReader) -> std::io::Result<()>;
    fn encode_to(&self, writer: &mut lobera_ivproto::IvWriter) -> std::io::Result<()>;
}

impl<T: GameMessage + std::fmt::Debug + 'static> AnyGameMessage for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn get_msg_no(&self) -> u16 {
        Self::MSG_NO
    }
    fn decode_into(&mut self, reader: &mut lobera_ivproto::IvReader) -> std::io::Result<()> {
        *self = <Self as lobera_ivproto::IvMessage>::decode(reader)?;
        Ok(())
    }
    fn encode_to(&self, writer: &mut lobera_ivproto::IvWriter) -> std::io::Result<()> {
        lobera_ivproto::IvMessage::encode(self, writer)
    }
}

pub struct GameMessageRegistry {
    pub id: u16,
    pub factory: fn() -> Box<dyn AnyGameMessage>,
}

inventory::collect!(GameMessageRegistry);

pub fn create_message_by_id(id: u16) -> Option<Box<dyn AnyGameMessage>> {
    for msg in inventory::iter::<GameMessageRegistry> {
        if msg.id == id {
            return Some((msg.factory)());
        }
    }
    None
}

pub fn get_msg_no<T: GameMessage>() -> u16 {
    T::MSG_NO
}

#[cfg(test)]
mod tests;
