use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 好友列表
async fn __get_friends_data(
    _uid: u32,
    _msg: schemas::FriendProto_GetFriendsData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::FriendProto_FriendAdd {
        friends: Some(vec![]),
        had_del_cnt: Some(0),
        had_apply_cnt: Some(0),
    })])
}
