use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 请求赛季数据
async fn __get_season_data(
    _uid: u32,
    _msg: schemas::AbattoirProto_GetSeasonData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::AbattoirProto_GetSeasonDataRet {
        id: Some(7),
        startTime: Some(1764554400),
        endTime: Some(1769885999),
        randRefreshTime: Some(1769885999),
        selectRefreshTime: Some(1769885999),
        scoreData: Some(vec![
            schemas::sScoreData {
                modType: Some(1),
                starNum: Some(0),
                maxLv: Some(0),
                maxHardLv: Some(0),
            },
            schemas::sScoreData {
                modType: Some(2),
                starNum: Some(0),
                maxLv: Some(0),
                maxHardLv: Some(0),
            },
        ]),
        randModData: Some(schemas::sRandModData {
            randLvs: Some(vec![]),
            selectCardData: Some(schemas::sSelectCard {
                waitSelectCards: Some(vec![]),
                selectCards: Some(vec![]),
            }),
            isGet: Some(false),
            isOver: Some(false),
        }),
        isRandPay: Some(false),
        isSelectPay: Some(false),
        freeCnt: Some(3),
        ..Default::default()
    })])
}
