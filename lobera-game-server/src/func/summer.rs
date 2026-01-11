use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 宠物信息
async fn __pet_info(
    _uid: u32,
    _msg: schemas::SummerProto_PetInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::SummerProto_PetInfoRet {
        info: Some(vec![schemas::sPet {
            id: Some(1),
            feed: Some(0),
            idx: Some(0),
            wash: Some(0),
            happy: Some(0),
            food: Some(0),
            last_time: Some(1767314840),
            sport: Some(0),
            scene: Some(0),
            keep_time: Some(0),
            start_time: Some(0),
        }]),
        cur_pet: Some(1),
        locked: Some(vec![107, 101, 102]),
        gained: Some(vec![]),
        tNextRandom: Some(0),
        haveReward: Some(false),
    })])
}
