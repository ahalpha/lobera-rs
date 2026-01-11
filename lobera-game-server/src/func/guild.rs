use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取工会信息
async fn __get_guild_info(
    _uid: u32,
    _msg: schemas::GuildProto_GuildInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::GuildProto_GuildInfoRet::default())])
}
