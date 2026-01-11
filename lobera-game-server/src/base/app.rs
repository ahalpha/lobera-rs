use lobera_game_msg::AnyGameMessage;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type HandlerResult = Vec<Option<Box<dyn AnyGameMessage>>>;
pub type MsgHandler = Box<
    dyn Fn(
            u32, // uid
            Box<dyn AnyGameMessage>,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<HandlerResult>> + Send>>
        + Send
        + Sync,
>;

pub struct GameServerApp {
    handlers: HashMap<u16, MsgHandler>,
}

pub struct HandlerRegistration {
    pub register_fn: fn(&mut GameServerApp),
}

inventory::collect!(HandlerRegistration);

impl GameServerApp {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<F, Fut, M>(&mut self, handler: F)
    where
        M: lobera_game_msg::GameMessage + Clone + 'static,
        F: Fn(u32, M) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = anyhow::Result<HandlerResult>> + Send + 'static,
    {
        let msg_id = M::MSG_NO;
        let wrapped_handler = Box::new(move |uid: u32, msg: Box<dyn AnyGameMessage>| {
            let msg = msg.as_any().downcast_ref::<M>().unwrap().clone();
            let fut = handler(uid, msg);
            Box::pin(fut) as Pin<Box<dyn Future<Output = anyhow::Result<HandlerResult>> + Send>>
        });
        self.handlers.insert(msg_id, wrapped_handler);
    }

    pub async fn handle_message(
        &self,
        msg_id: u16,
        uid: u32,
        msg: Box<dyn AnyGameMessage>,
    ) -> anyhow::Result<HandlerResult> {
        if let Some(handler) = self.handlers.get(&msg_id) {
            handler(uid, msg).await
        } else {
            log::debug!("Unknown protocol: {}", msg_id);
            Ok(vec![None])
        }
    }
}

lazy_static::lazy_static! {
    pub static ref APP: Arc<GameServerApp> = {
        let mut app = GameServerApp::new();
        register_handlers(&mut app);
        Arc::new(app)
    };
}

fn register_handlers(app: &mut GameServerApp) {
    for registration in inventory::iter::<HandlerRegistration> {
        (registration.register_fn)(app);
    }
}


pub fn wrap<T: AnyGameMessage + 'static>(msg: T) -> Option<Box<dyn AnyGameMessage>> {
    Some(Box::new(msg))
}