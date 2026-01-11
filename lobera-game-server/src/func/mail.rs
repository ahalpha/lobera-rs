use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 邮件列表
async fn __get_mails_data(
    _uid: u32,
    _msg: schemas::MailProto_GetMailsData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::MailProto_MailAddNotice {
        adds: Some(vec![]),
    })])
}
