use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};

#[func_reg] // 任务列表
async fn __get_tasks_data(
    _uid: u32,
    _msg: schemas::TaskProto_GetTasksData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![
        wrap(schemas::TaskProto_TaskAdd {
            tasks: Some(vec![]),
            is_finish: Some(true),
        }),
        wrap(schemas::TaskProto_GetTasksDataRet::default()),
    ])
}

#[func_reg] // 获取重置任务信息
async fn __get_reset_task_info(
    _uid: u32,
    _msg: schemas::TaskProto_GetResetTaskInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::TaskProto_GetResetTaskInfoRet {
        dailyResetTime: Some(1767380400),
        weeklyResetTime: Some(1767553200),
        dailyStar: Some(0),
        weeklyStar: Some(0),
        anvsStarInfo: Some(vec![]),
    })])
}

#[func_reg] // 获取七日任务的当前天
async fn __get_seven_tasks_day(
    _uid: u32,
    _msg: schemas::TaskProto_GetSevenTasksDay,
) -> anyhow::Result<HandlerResult> {
    let mut res = schemas::TaskProto_GetSevenTasksDayRet {
        _type: _msg._type,
        ..Default::default()
    };

    if let Some(17) = _msg._type {
        res.c_day = Some(1);
    }

    Ok(vec![wrap(res)])
}
