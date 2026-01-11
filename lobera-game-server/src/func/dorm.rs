use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};
use std::collections::HashMap;

#[func_reg] // 获取开放列表
async fn __get_open_dorm(
    _uid: u32,
    _msg: schemas::DormProto_GetOpenDorm,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::DormProto_GetOpenDormRet {
        fid: _msg.fid,
        infos: Some(vec![
            schemas::sDormA {
                id: Some(101),
                num: Some(0),
                roleIds: Some(vec![]),
                lv: Some(1),
                comfort: Some(0),
                ..Default::default()
            },
            schemas::sDormA {
                id: Some(201),
                num: Some(0),
                roleIds: Some(vec![]),
                lv: Some(1),
                comfort: Some(0),
                ..Default::default()
            },
        ]),
    })])
}

#[func_reg] // 获取自己的主题
async fn __get_self_theme(
    _uid: u32,
    _msg: schemas::DormProto_GetSelfTheme,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![
        wrap(schemas::DormProto_GetSelfThemeRet {
            themeType: Some(4),
            themes: Some(HashMap::new()),
            isFinish: Some(false),
        }),
        wrap(schemas::DormProto_GetSelfThemeRet {
            themeType: Some(1),
            themes: Some(HashMap::new()),
            isFinish: Some(true),
        }),
    ])
}

#[func_reg] // 家具购买记录
async fn __buy_record(
    _uid: u32,
    _msg: schemas::DormProto_BuyRecord,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::DormProto_BuyRecordRet {
        infos: Some(r#"{[4005]=1,[4006]=1,[4007]=1,[4008]=1,[4009]=1,[4010]=1,[4011]=1,[4001]=1,[4002]=2,[4004]=3}"#.to_string()),
    })])
}

#[func_reg] // 宿舍宠物信息
async fn __dorm_pet_info(
    _uid: u32,
    _msg: schemas::DormProto_DormPetInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::DormProto_DormPetInfoRet {
        info: Some(vec![]),
    })])
}
