use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取商店的售卖商品
async fn __get_shop_commodity(
    _uid: u32,
    _msg: schemas::ShopProto_GetShopCommodity,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::ShopProto_GetShopCommodityRet {
        infos: Some(vec![]),
        m_cnt: Some(0),
        is_finish: Some(true),
    })])
}

#[func_reg] // 获取商店页的开启时间
async fn __get_shop_open_time(
    _uid: u32,
    _msg: schemas::ShopProto_GetShopOpenTime,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::ShopProto_GetShopOpenTimeRet {
        infos: Some(vec![]),
    })])
}

#[func_reg] // 获取皮肤返利领取记录
async fn __get_skin_rebate_record(
    _uid: u32,
    _msg: schemas::ShopProto_GetSkinRebateRecord,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![
        wrap(schemas::ShopProto_GetSkinRebateRecordRet {
            skinRebateRecordList: Some(vec![]),
        }),
        wrap(schemas::ShopProto_GetSkinRebateCanTakeRewardRet {
            skinRebateRecordList: Some(vec![]),
        }),
    ])
}
