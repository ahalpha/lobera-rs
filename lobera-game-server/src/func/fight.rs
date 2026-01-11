use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取世界boss活动信息
async fn __get_boss_activity_info(
    _uid: u32,
    _msg: schemas::FightProtocol_GetBossActivityInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![])
}

#[func_reg] // 请求乱序演习数据
async fn __get_rogue_info(
    _uid: u32,
    _msg: schemas::FightProtocol_GetRogueInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::FightProto_GetRogueInfoRet {
        datas: Some(vec![]),
        is_finish: Some(true),
        maxGroup: Some(0),
        isFighting: Some(false),
    })])
}

#[func_reg] // 请求战力调遣（RogueSecond）数据
async fn __get_rogue_s_info(
    _uid: u32,
    _msg: schemas::FightProtocol_GetRogueSInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::FightProto_GetRogueSInfoRet {
        datas: Some(vec![]),
        is_finish: Some(true),
        maxGroup: Some(0),
        gained: Some(vec![]),
        ..Default::default()
    })])
}

#[func_reg] // 请求限制肉鸽爬塔
async fn __get_rogue_t_info(
    _uid: u32,
    _msg: schemas::FightProtocol_GetRogueTInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::FightProto_GetRogueTInfoRet {
        data: Some(vec![]),
        is_finish: Some(true),
        score: Some(0),
        stageIdx: Some(0),
        monthScore: Some(0),
        periodIdx: Some(0),
        maxGroup: Some(0),
        maxBoss: Some(0),
        win1: Some(1767254400),
        win2: Some(0),
    })])
}

#[func_reg] // 请求深塔计划数据
async fn __get_tower_deep_info(
    _uid: u32,
    _msg: schemas::FightProtocol_GetTowerDeepInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::FightProto_GetTowerDeepInfoRet {
        id: _msg.id,
        datas: Some(vec![]),
        maxGroup: Some(0),
    })])
}
