use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 活动列表时间推送
async fn __get_active_time_list(
    _uid: u32,
    _msg: schemas::OperateActiveProto_GetActiveTimeList,
) -> anyhow::Result<HandlerResult> {
    let list = vec![
        schemas::sOperateActive {
            id: Some(3001),
            openTime: Some(1761703200),
            closeTime: Some(1763492400),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1031),
            openTime: Some(1747203000),
            closeTime: Some(1758654000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1034),
            openTime: Some(1764727200),
            closeTime: Some(1767034800),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1036),
            openTime: Some(1761703200),
            closeTime: Some(1763492399),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1040),
            openTime: Some(1767060000),
            closeTime: Some(1770750000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(3005),
            openTime: Some(1750816800),
            closeTime: Some(1753815600),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(3003),
            openTime: Some(1756864800),
            closeTime: Some(1758654000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1012),
            openTime: Some(1767034800),
            closeTime: Some(1769972400),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1010),
            openTime: Some(1762282800),
            closeTime: Some(1764702000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1008),
            openTime: Some(1712541600),
            closeTime: Some(1713294000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1014),
            openTime: Some(1726200000),
            closeTime: Some(1727031600),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1013),
            openTime: Some(1721761200),
            closeTime: Some(1723575600),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1018),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1016),
            openTime: Some(1758999600),
            closeTime: Some(1759950000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1023),
            openTime: Some(1753840800),
            closeTime: Some(1756839600),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1020),
            openTime: Some(1707580800),
            closeTime: Some(1733882400),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1042),
            openTime: Some(1767060000),
            closeTime: Some(1770750000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1041),
            openTime: Some(1767060000),
            closeTime: Some(1770750000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1038),
            openTime: Some(1767060000),
            closeTime: Some(1770750000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1037),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1003),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1001),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1007),
            openTime: Some(1750816800),
            closeTime: Some(1751655600),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1004),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1035),
            openTime: Some(1761703200),
            closeTime: Some(1763492399),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1015),
            openTime: Some(1763517600),
            closeTime: Some(1764702000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1009),
            openTime: Some(1718737200),
            closeTime: Some(1737486000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1011),
            openTime: Some(1763517600),
            closeTime: Some(1764702000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(3002),
            openTime: Some(1764727200),
            closeTime: Some(1767034800),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1002),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1006),
            openTime: Some(1707271200),
            closeTime: Some(1708455600),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1005),
            openTime: Some(1707271200),
            closeTime: Some(1739905200),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1028),
            openTime: Some(1737918000),
            closeTime: Some(1738857599),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1024),
            openTime: Some(1706155200),
            closeTime: Some(1735930800),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1019),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1021),
            openTime: Some(1767232800),
            closeTime: Some(1767801599),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1017),
            openTime: Some(1724551200),
            closeTime: Some(1729018800),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1025),
            openTime: Some(1706155200),
            closeTime: Some(1736622000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1026),
            openTime: Some(1706155200),
            closeTime: Some(1736276400),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1027),
            openTime: Some(1706155200),
            closeTime: Some(1737486000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1029),
            openTime: Some(1748372400),
            closeTime: Some(1749236400),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1030),
            openTime: Some(1753840800),
            closeTime: Some(1755630000),
            ..Default::default()
        },
        schemas::sOperateActive {
            id: Some(1033),
            openTime: Some(1758679200),
            closeTime: Some(1760468400),
            ..Default::default()
        },
    ];

    Ok(vec![wrap(
        schemas::OperateActiveProto_GetActiveTimeListRet {
            operateActiveList: Some(list),
            isFinish: Some(true),
        },
    )])
}

#[func_reg] // 获取皮肤返利特权卡数据
async fn __get_skin_rebate_info(
    _uid: u32,
    _msg: schemas::OperateActiveProto_GetSkinRebateInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(
        schemas::OperateActiveProto_GetSkinRebateInfoRet {
            skinIdList: Some(vec![]),
        },
    )])
}

#[func_reg] // 端午签到数据
async fn __get_dragon_boat_festival_info(
    _uid: u32,
    _msg: schemas::OperateActiveProto_GetDragonBoatFestivalInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![])
}

#[func_reg] // 请求女仆咖啡玩法数据
async fn __get_maid_coffee_data(
    _uid: u32,
    _msg: schemas::OperateActiveProto_GetMaidCoffeeData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![])
}

#[func_reg] // 请求万圣节游戏数据
async fn __get_halloween_game_data(
    _uid: u32,
    _msg: schemas::OperateActiveProto_GetHalloweenGameData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![])
}

#[func_reg] // 请求圣诞活动数据
async fn __get_christmas_gift_data(
    _uid: u32,
    _msg: schemas::OperateActiveProto_GetChristmasGiftData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![])
}

#[func_reg] // 请求大富翁活动数据
async fn __get_rich_man_data(
    _uid: u32,
    _msg: schemas::OperateActiveProto_GetRichManData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::OperateActiveProto_GetRichManDataRet {
        cfgId: Some(1001),
        sort: Some(1),
        mapId: Some(1001),
        throwCnt: Some(0),
        ..Default::default()
    })])
}
