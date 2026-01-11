use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取能力
async fn __get_ability(
    _uid: u32,
    _msg: schemas::AbilityProto_GetAbility,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::AbilityProto_GetAbilityRet {
        num: Some(430),
        abilitys: Some(vec![]),
        lastResetTime: Some(0),
    })])
}
