use crate::base::app::{HandlerResult, wrap};
use crate::base::config::CONFIG;
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 签到记录
async fn __get_sign_info(
    _uid: u32,
    _msg: schemas::ClientProto_GetSignInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![])
}

#[func_reg] // 副本星级奖励领取情况
async fn __dup_sum_star_reward_info(
    _uid: u32,
    _msg: schemas::ClientProto_DupSumStarRewardInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::ClientProto_DupSumStarRewardInfoRet {
        infos: Some(vec![
            schemas::sDupSumStarReward {
                id: Some(101),
                indexs: Some(vec![1, 1, 1]),
            },
            schemas::sDupSumStarReward {
                id: Some(102),
                indexs: Some(vec![1, 1]),
            },
        ]),
    })])
}

#[func_reg] // 心跳
async fn __heartbeat(
    _uid: u32,
    _msg: schemas::ClientProto_Heartbeat,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::LoginProto_Heartbeat::default())])
}

#[func_reg] // 查询账号
async fn __query_account(
    _uid: u32,
    _msg: schemas::ClientProto_QueryAccount,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![
        // wrap(schemas::SystemProto_ServerWarning {
        //     msg: Some("客户端和服务器配置版本不一致, client:15749 svr:16857".to_string()),
        // }),
        wrap(schemas::LoginProto_QueryAccount {
            uid: Some(100000000),
            svr_version: Some("16857".to_string()),
            anti_addiction: Some(
                r#"{account=10000,subType=1,number="0",pi="0",accType=1,name="0"}"#.to_string(),
            ),
            is_anti_addiction: Some(0),
        }),
    ])
}

#[func_reg] // 预登陆逻辑服
async fn __pre_login_game(
    _uid: u32,
    _msg: schemas::ClientProto_PreLoginGame,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::LoginProto_PreLoginGame {
        key: Some("8526a1c9-06e8-4685-b40f-d3e68c2a9f00".to_string()),
        ip: Some(CONFIG.host.clone()),
        port: Some(CONFIG.port as i16),
        is_ok: Some(true),
        svr_group: Some("release".to_string()),
        center_web_uid: Some("1000000".to_string()),
        third_part_id: Some("P9374918225".to_string()),
        ..Default::default()
    })])
}

#[func_reg] // 登陆逻辑服
async fn __login_game(
    _uid: u32,
    _msg: schemas::ClientProto_LoginGame,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![
        wrap(schemas::LoginProto_LoginGame {
            infos: Some(schemas::sPlrData {
                uid: Some(10000),
                name: Some("AhAlpha".to_string()),
                hot: Some(999),
                level: Some(100),
                gold: Some(999999999),
                diamond: Some(999999999),
                currtime: Some(1767314861),
                create_time: Some(1704890315),
                army_coin: Some(16000),
                tp: Some(3),
                tpBeginTime: Some(1705158521),
                ..Default::default()
            }),
            can_modify_name: Some(1),
            icon_id: Some(7101001),
            panel_id: Some(7101001),
            ability_num: Some(430),
            serverID: Some(256),
            sel_card_ix: Some(1),
            use_vid: Some(1),
            icon_frame: Some(1),
            role_panel_id: Some(7101001),
            background_id: Some(1),
            icon_title: Some(1),
            icon_emotes: Some(vec![1033, 1031, 1032]),
            ..Default::default()
        }),
        wrap(schemas::FightProto_GlobalBossInfoRet {
            bossId: Some(1015),
            beginTime: Some(1767207600),
            endTime: Some(1768935600),
            hp: Some(1988725058568.0),
        }),
        wrap(schemas::DownloadProto_CheckDownloadRewardRet { isGet: Some(true) }),
    ])
}

#[func_reg] // 客户端初始化完成
async fn __init_finish(
    _uid: u32,
    _msg: schemas::ClientProto_InitFinish,
) -> anyhow::Result<HandlerResult> {
    let item = vec![
        schemas::ItemData {
            id: Some(10036),
            num: Some(99999999),
            ..Default::default()
        },
        schemas::ItemData {
            id: Some(11002),
            num: Some(99999999),
            ..Default::default()
        },
        schemas::ItemData {
            id: Some(10040),
            num: Some(99999999),
            ..Default::default()
        },
    ];

    Ok(vec![
        wrap(schemas::PlayerProto_ItemBag {
            item: Some(item),
            ix: Some(1),
            is_finish: Some(true),
        }),
        wrap(schemas::PlayerProto_DuplicateData {
            mainLine: Some(vec![]),
            is_finish: Some(true),
            ..Default::default()
        }),
        wrap(schemas::ClientProto_InitFinishRet::default()),
    ])
}

#[func_reg] // 签到
async fn __add_sign(
    _uid: u32,
    _msg: schemas::ClientProto_AddSign,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![])
}
