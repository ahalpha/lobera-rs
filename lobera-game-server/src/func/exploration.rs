use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};
use std::collections::HashMap;

#[func_reg] // 获取勘探信息
async fn __get_exploration_info(
    _uid: u32,
    _msg: schemas::ExplorationProto_GetInfo,
) -> anyhow::Result<HandlerResult> {
    let mut res = schemas::ExplorationProto_GetInfoRet {
        id: _msg.id,
        ..Default::default()
    };

    if let Some(1022) = _msg.id {
        res.lv = Some(1);
        res.exp = Some(0);
        res._type = Some(1);
        let mut get_infos = HashMap::new();
        get_infos.insert(
            40001,
            schemas::sExplorationInfo {
                rid: Some(40001),
                gets: Some(vec![1]),
            },
        );
        res.get_infos = Some(get_infos);
        res.can_get_cnt = Some(0);
        res.ex_can_get_cnt = Some(0);
    }

    Ok(vec![wrap(res)])
}
