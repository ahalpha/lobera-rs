use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取成就的数据
async fn __get_finish_info(
    _uid: u32,
    _msg: schemas::AchievementProto_GetFinishInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::AchievementProto_GetFinishInfoRet {
        finish_list: Some(vec![
            schemas::sStrNumInfo {
                sid: Some(80002),
                num: Some(1),
            },
            schemas::sStrNumInfo {
                sid: Some(20003),
                num: Some(1),
            },
            schemas::sStrNumInfo {
                sid: Some(80101),
                num: Some(236),
            },
            schemas::sStrNumInfo {
                sid: Some(80105),
                num: Some(540),
            },
            schemas::sStrNumInfo {
                sid: Some(30001),
                num: Some(569),
            },
            schemas::sStrNumInfo {
                sid: Some(10101),
                num: Some(20),
            },
            schemas::sStrNumInfo {
                sid: Some(10005),
                num: Some(1),
            },
            schemas::sStrNumInfo {
                sid: Some(80001),
                num: Some(1),
            },
            schemas::sStrNumInfo {
                sid: Some(10002),
                num: Some(1),
            },
            schemas::sStrNumInfo {
                sid: Some(80106),
                num: Some(142),
            },
            schemas::sStrNumInfo {
                sid: Some(40001),
                num: Some(4),
            },
            schemas::sStrNumInfo {
                sid: Some(30105),
                num: Some(569),
            },
            schemas::sStrNumInfo {
                sid: Some(50001),
                num: Some(1),
            },
            schemas::sStrNumInfo {
                sid: Some(40004),
                num: Some(3414),
            },
            schemas::sStrNumInfo {
                sid: Some(10001),
                num: Some(1),
            },
            schemas::sStrNumInfo {
                sid: Some(50005),
                num: Some(45),
            },
            schemas::sStrNumInfo {
                sid: Some(80004),
                num: Some(1),
            },
            schemas::sStrNumInfo {
                sid: Some(40003),
                num: Some(1740818),
            },
            schemas::sStrNumInfo {
                sid: Some(20001),
                num: Some(1),
            },
        ]),
        is_finish: Some(true),
    })])
}

#[func_reg] // 成就的领取详情
async fn __get_reward_info(
    _uid: u32,
    _msg: schemas::AchievementProto_GetRewardInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::AchievementProto_GetRewardInfoRet {
        infos: Some(vec![
            schemas::sAchievementRewardDetail {
                id: Some(10010111),
                finish_time: Some(1735414876),
                ..Default::default()
            },
            schemas::sAchievementRewardDetail {
                id: Some(10010241),
                time: Some(1728156582),
                finish_time: Some(1728156509),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10040031),
                time: Some(1732936880),
                finish_time: Some(1732878114),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10080122),
                time: Some(1734079904),
                finish_time: Some(1734079886),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10080021),
                time: Some(1728156582),
                finish_time: Some(1728156509),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10040041),
                time: Some(1731129779),
                finish_time: Some(1731129750),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10050011),
                time: Some(1728156582),
                finish_time: Some(1728156509),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10080121),
                time: Some(1729298128),
                finish_time: Some(1729298114),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10030042),
                time: Some(1728696935),
                finish_time: Some(1728696907),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10030043),
                time: Some(1731776228),
                finish_time: Some(1731776210),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10030093),
                time: Some(1731776228),
                finish_time: Some(1731776210),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10020011),
                time: Some(1728156582),
                finish_time: Some(1728156509),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10030041),
                time: Some(1728366780),
                finish_time: Some(1728366752),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10020021),
                time: Some(1728156582),
                finish_time: Some(1728156509),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10030091),
                time: Some(1728366780),
                finish_time: Some(1728366752),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10030092),
                time: Some(1728696935),
                finish_time: Some(1728696907),
            },
            schemas::sAchievementRewardDetail {
                id: Some(10010101),
                finish_time: Some(1735967507),
                ..Default::default()
            },
            schemas::sAchievementRewardDetail {
                id: Some(10040042),
                finish_time: Some(1734322871),
                ..Default::default()
            },
        ]),
        is_finish: Some(true),
    })])
}
