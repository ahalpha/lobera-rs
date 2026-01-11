use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 获取章节缩略信息
async fn __get_chapter_simple_info(
    _uid: u32,
    _msg: schemas::LovePlusProto_GetChapterSimpleInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::LovePlusProto_GetChapterSimpleInfoRet {
        chapterInfo: Some(vec![]),
        imgIds: Some(vec![]),
    })])
}
