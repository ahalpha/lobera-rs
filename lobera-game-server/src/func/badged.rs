use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取徽章的数据
async fn __get_badged_info(
    _uid: u32,
    _msg: schemas::BadgedProto_GetBadgedInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::BadgedProto_GetBadgedInfoRet {
        infos: Some(vec![
            schemas::sBadged {
                id: Some(3),
                finish_time: Some(1728156509),
                is_new: Some(true),
            },
            schemas::sBadged {
                id: Some(1301),
                finish_time: Some(1736063351),
                is_new: Some(true),
            },
            schemas::sBadged {
                id: Some(1302),
                finish_time: Some(1735967489),
                is_new: Some(true),
            },
            schemas::sBadged {
                id: Some(1303),
                finish_time: Some(1735848549),
                is_new: Some(true),
            },
            schemas::sBadged {
                id: Some(1304),
                finish_time: Some(1735414729),
                is_new: Some(true),
            },
        ]),
        is_finish: Some(true),
    })])
}

#[func_reg] // 获取徽章的排序
async fn __get_sort_badged_info(
    _uid: u32,
    _msg: schemas::BadgedProto_GetSortBadgedInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::BadgedProto_GetSortBadgedInfoRet {
        pos: Some(vec![]),
    })])
}
