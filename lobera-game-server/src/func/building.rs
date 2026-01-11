use crate::base::app::{HandlerResult, wrap};
use lobera_game_msg::{func_reg, schemas};
use std::collections::HashMap;

#[func_reg] // 获取建筑基本信息
async fn __get_builds_base_info(
    _uid: u32,
    _msg: schemas::BuildingProto_BuildsBaseInfo,
) -> anyhow::Result<HandlerResult> {
    let mut results = vec![wrap(schemas::AchievementProto_UpdateFinishInfoRet {
        finish_list: Some(vec![schemas::sStrNumInfo {
            sid: Some(80106),
            num: Some(147),
        }]),
        is_finish: Some(true),
    })];

    results.push(wrap(schemas::BuildingProto_BuildsBaseInfoRet {
        roleCnt: Some(0),
        buildCnts: Some("{}".to_string()),
        power: Some(r#"{combineOrderPer=100,tradeOrderPer=100,perBenefit=100,add=540,realCost=440,cost=-440,expeditionPer=100,remouldOrderPer=100}"#.to_string()),
        perRolePower: Some(100),
        perRoleAbilityBenefit: Some(100),
        warningLv: Some(1),
        runTypeCfgId: Some(3),
        extraPresetTeamNum: Some(2),
    }));

    Ok(results)
}

#[func_reg] // 获取建筑列表
async fn __get_builds_list(
    _uid: u32,
    _msg: schemas::BuildingProto_BuildsList,
) -> anyhow::Result<HandlerResult> {
    let mut builds = vec![];

    let b1 = schemas::sBuildInfo {
        id: Some(20010006),
        cfgId: Some(2001),
        lv: Some(1),
        running: Some(true),
        perHpPower: Some(100),
        perRolePower: Some(100),
        perBenefit: Some(100),
        perHpBenefit: Some(100),
        perRoleTiredBenefit: Some(100),
        perRoleAbilityBenefit: Some(100),
        presetRoles: Some(vec![schemas::sPresetRoleTeam {
            teamId: Some(1),
            ..Default::default()
        }]),
        curPresetId: Some(1),
        ..Default::default()
    };
    builds.push(b1);

    let b2 = schemas::sBuildInfo {
        id: Some(20020004),
        cfgId: Some(2002),
        lv: Some(1),
        running: Some(true),
        perHpPower: Some(100),
        perRolePower: Some(100),
        perBenefit: Some(100),
        perHpBenefit: Some(100),
        perRoleTiredBenefit: Some(100),
        perRoleAbilityBenefit: Some(100),
        presetRoles: Some(vec![schemas::sPresetRoleTeam {
            teamId: Some(1),
            ..Default::default()
        }]),
        curPresetId: Some(1),
        ..Default::default()
    };
    builds.push(b2);

    let b3 = schemas::sBuildInfo {
        id: Some(10060005),
        cfgId: Some(1006),
        hp: Some(1000),
        lv: Some(1),
        running: Some(true),
        perHpPower: Some(100),
        perRolePower: Some(100),
        perBenefit: Some(100),
        perHpBenefit: Some(100),
        perRoleTiredBenefit: Some(100),
        perRoleAbilityBenefit: Some(100),
        presetRoles: Some(vec![schemas::sPresetRoleTeam {
            teamId: Some(1),
            ..Default::default()
        }]),
        curPresetId: Some(1),
        ..Default::default()
    };
    builds.push(b3);

    let mut b4 = schemas::sBuildInfo {
        id: Some(10030007),
        cfgId: Some(1003),
        hp: Some(800),
        lv: Some(1),
        tPreGifs: Some(1767314246),
        tNexGifs: Some(1767316046),
        running: Some(true),
        perHpPower: Some(100),
        perRolePower: Some(100),
        perBenefit: Some(100),
        perHpBenefit: Some(100),
        perRoleTiredBenefit: Some(100),
        perRoleAbilityBenefit: Some(100),
        tFlush: Some(1767316046.0),
        presetRoles: Some(vec![schemas::sPresetRoleTeam {
            teamId: Some(1),
            ..Default::default()
        }]),
        curPresetId: Some(1),
        ..Default::default()
    };
    let mut product_add = HashMap::new();
    product_add.insert(
        60102,
        schemas::sProductAdd {
            id: Some(60102),
            num: Some(0),
            limit: Some(0),
        },
    );
    product_add.insert(
        60101,
        schemas::sProductAdd {
            id: Some(60101),
            num: Some(0),
            limit: Some(0),
        },
    );
    product_add.insert(
        60103,
        schemas::sProductAdd {
            id: Some(60103),
            num: Some(0),
            limit: Some(0),
        },
    );
    b4.productAdd = Some(product_add);
    builds.push(b4);

    let b5 = schemas::sBuildInfo {
        id: Some(10010003),
        cfgId: Some(1001),
        hp: Some(1000),
        lv: Some(1),
        running: Some(true),
        perHpPower: Some(100),
        perRolePower: Some(100),
        perBenefit: Some(100),
        perHpBenefit: Some(100),
        perRoleTiredBenefit: Some(100),
        perRoleAbilityBenefit: Some(100),
        presetRoles: Some(vec![schemas::sPresetRoleTeam {
            teamId: Some(1),
            ..Default::default()
        }]),
        curPresetId: Some(1),
        ..Default::default()
    };
    builds.push(b5);

    Ok(vec![wrap(schemas::BuildingProto_BuildsListRet {
        builds: Some(builds),
        is_finish: Some(true),
    })])
}

#[func_reg] // 获取突袭信息
async fn __get_assualt_info(
    _uid: u32,
    _msg: schemas::BuildingProto_AssualtInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::BuildingProto_AssualtInfoRet {
        info: Some(schemas::sAssualInfo {
            running: Some(false),
            wIds: Some("{}".to_string()),
            fIds: Some("{}".to_string()),
            index: Some(0),
            ..Default::default()
        }),
    })])
}
