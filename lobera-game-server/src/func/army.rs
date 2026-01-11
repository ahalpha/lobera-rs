use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取军演信息
async fn __get_practice_info(
    _uid: u32,
    _msg: schemas::ArmyProto_GetPracticeInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::ArmyProto_GetPracticeInfoRet {
        info: Some(schemas::sPracticeInfo {
            start_time: Some(1765307400),
            end_time: Some(1767985200),
            can_join_cnt: Some(10),
            t_join_cnt: Some(1767380400),
            flush_cnt: Some(0),
            rank_level: Some(1),
            max_rank_level: Some(0),
            rank: Some(0),
            max_rank: Some(0),
            score: Some(0),
            can_join_buy_cnt: Some(0),
            role_panel_id: Some(7101001),
            live2d: Some(1),
            ..Default::default()
        }),
        selfInfo: Some(true),
        listInfo: Some(false),
        army_ix: Some(24),
        ..Default::default()
    })])
}

#[func_reg] // 获取自由匹配军演信息
async fn __free_match_info(
    _uid: u32,
    _msg: schemas::ArmyProto_FreeMatchInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::ArmyProto_FreeMatchInfoRet {
        cfg_id: Some(0),
        score: Some(0),
        rank: Some(1),
        max_rank: Some(1),
        can_join_cnt: Some(999),
        reward_info: Some(schemas::sFreeMatchRewarInfo {
            join_cnt: Some(0),
            get_join_cnt_id: Some(0),
            get_rank_lv_id: Some(0),
            win_cnt: Some(0),
            get_win_cnt_ix: Some(0),
        }),
        role_panel_id: Some(7101001),
        live2d: Some(1),
        ..Default::default()
    })])
}
