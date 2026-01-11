use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取问卷调查列表
async fn __get_questionnaire_info(
    _uid: u32,
    _msg: schemas::QuestionnaireProto_GetInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::QuestionnaireProto_GetInfoRet {
        infos: Some(vec![]),
    })])
}
