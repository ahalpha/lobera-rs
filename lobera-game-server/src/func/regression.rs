use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 回归活动信息
async fn __get_regression_info(
    _uid: u32,
    _msg: schemas::RegressionProto_GetInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::RegressionProto_GetInfoRet {
        resourcesIsGain: Some(0),
    })])
}

#[func_reg] // 获取道具池信息
async fn __get_item_pool_info(
    _uid: u32,
    _msg: schemas::RegressionProto_ItemPoolInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::RegressionProto_ItemPoolInfoRet {
        info: Some(vec![schemas::sItemPool {
            id: Some(1007),
            round: Some(1),
            startTime: Some(1767139228),
            drawArr: Some("{}".to_string()),
            drawTimes: Some(0),
        }]),
    })])
}

#[func_reg] // 获取绑定信息
async fn __get_plr_bind_info(
    _uid: u32,
    _msg: schemas::RegressionProto_PlrBindInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::RegressionProto_PlrBindInfoRet {
        activeId: Some(3),
        code: Some("60028_100000000".to_string()),
        plrs: Some("{}".to_string()),
        taskGets: Some(
            r#"{[1]={resetTime=1767380400},[2]={resetTime=1767553200}}"#.to_string(),
        ),
        applyCnt: Some(0),
        applyResetTime: Some(1767315600),
        stage: Some(1),
        doneTaskNum: Some(3),
    })])
}

#[func_reg] // 获取申请我的列表
async fn __get_plr_bind_invite_list(
    _uid: u32,
    _msg: schemas::RegressionProto_PlrBindInviteList,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::RegressionProto_PlrBindInviteListRet {
        page: _msg.page,
        friends: Some(vec![]),
        isEnd: Some(true),
    })])
}

#[func_reg] // 获取燃料补给信息
async fn __get_resupply_info(
    _uid: u32,
    _msg: schemas::RegressionProto_ResupplyInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::RegressionProto_ResupplyInfoRet {
        info: Some(vec![schemas::sResupply {
            id: Some(1),
            gainArr: Some(vec![0, 0]),
        }]),
    })])
}

#[func_reg] // 回归活跃返利信息
async fn __get_active_rewards_info(
    _uid: u32,
    _msg: schemas::RegressionProto_ActiveRewardsInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::RegressionProto_ActiveRewardsInfoRet {
        idx: Some(0),
        loginDay: Some(0),
        gainArr: Some(vec![]),
    })])
}
