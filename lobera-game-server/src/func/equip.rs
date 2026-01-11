use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取装备列表
async fn __get_equips(
    _uid: u32,
    _msg: schemas::EquipProto_GetEquips,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![
        wrap(schemas::EquipProto_EquipAdd {
            equips: Some(vec![]),
            cur_size: Some(0),
            max_size: Some(500),
            is_finish: Some(true),
        }),
        wrap(schemas::EquipProto_GetEquipsRet {
            equips: Some(vec![]),
            cur_size: Some(104),
            max_size: Some(500),
        }),
    ])
}

#[func_reg] // 装备洗练请求上一次的洗练数据
async fn __equip_refresh_get_last_data(
    _uid: u32,
    _msg: schemas::EquipProto_EquipRefreshGetLastData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![])
}
