use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取通行证信息
async fn __get_permit_info(
    _uid: u32,
    _msg: schemas::PermitProto_GetInfo,
) -> anyhow::Result<HandlerResult> {
    let mut res = schemas::PermitProto_GetInfoRet { list: Some(vec![]) };

    if let Some(-1) = _msg.id {
        res.list = Some(vec![
            schemas::sPermitGroup {
                id: Some(2002),
                gets: Some(vec![]),
                total: Some(0),
            },
            schemas::sPermitGroup {
                id: Some(2005),
                gets: Some(vec![]),
                total: Some(0),
            },
        ]);
    }

    Ok(vec![wrap(res)])
}
