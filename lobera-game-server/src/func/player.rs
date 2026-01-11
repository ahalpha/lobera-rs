use crate::base::app::{wrap, HandlerResult};
use lobera_data::TABLES;
use lobera_game_msg::{func_reg, schemas};
use serde_json::Value;
use std::collections::HashMap;

#[func_reg] // 卡牌数据
async fn __player_cards_data(
    _uid: u32,
    _msg: schemas::PlayerProto_CardsData,
) -> anyhow::Result<HandlerResult> {
    let mut results = vec![wrap(schemas::PlayerProto_CardsDataRet {
        store_exp: Some(1637964),
        rename_records: Some(HashMap::new()),
    })];

    let mut cards = Vec::new();
    for card_excel in TABLES.card_data.all() {
        let id = card_excel.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let name = card_excel
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let model = card_excel
            .get("model")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        let mut skills = HashMap::new();
        if let Some(excel_skills) = card_excel.get("skills").and_then(|v| v.as_object()) {
            for (_, skill_id_val) in excel_skills {
                if let Some(skill_id) = skill_id_val.as_u64() {
                    let skill_id = skill_id as u32;
                    let skill_excel = TABLES.skill.find_one(
                        &["id".to_string()],
                        &[serde_json::to_value(skill_id).unwrap_or(Value::Null)],
                    );

                    let (id, _type) = if let Some(se) = skill_excel {
                        let sid = se.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                        let stype = se.get("main_type").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                        (sid, stype)
                    } else {
                        (skill_id, if skill_id > 99999999 { 1 } else { 2 })
                    };

                    skills.insert(
                        skill_id,
                        schemas::sSkillData {
                            id: Some(id),
                            exp: Some(0),
                            _type: Some(_type),
                        },
                    );
                }
            }
        }

        cards.push(schemas::sCardsData {
            cfgid: Some(id),
            cid: Some(id),
            name: Some(name),
            skills: Some(skills),
            level: Some(1),
            break_level: Some(1),
            intensify_level: Some(1),
            intensify_exp: Some(0),
            hp: Some(0),
            exp: Some(0),
            cur_hot: Some(100),
            equip_ids: Some("{}".to_string()),
            skin: Some(model),
            skinIsl2d: Some(1),
            skin_a: Some(0),
            skinIsl2d_a: Some(1),
            sub_talent: Some(r#"{use={[1]=0,[2]=0,[3]=0}}"#.to_string()),
            tag: Some(0),
            mix_data: Some(schemas::sCardMixData {
                cl: Some(1),
                ..Default::default()
            }),
            ctime: Some(0),
            open_cards: Some(vec![]),
            open_mechas: Some(vec![]),
            ..Default::default()
        });
    }

    for chunk in cards.chunks(20) {
        results.push(wrap(schemas::PlayerProto_CardAdd {
            cards: Some(chunk.to_vec()),
            cur_size: Some(cards.len() as i16),
            finish: Some(chunk.as_ptr() == cards.chunks(20).last().unwrap().as_ptr()),
        }));
    }

    Ok(results)
}

#[func_reg] // 副本翻倍信息
async fn __player_section_multi_info(
    _uid: u32,
    _msg: schemas::PlayerProto_SectionMultiInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_SectionMultiInfoRet {
        infos: Some(vec![]),
        cntInfos: Some(vec![]),
    })])
}

#[func_reg] // 卡牌技能升级列表
async fn __player_card_skill_upgradelist(
    _uid: u32,
    _msg: schemas::PlayerProto_CardSkillUpgradelist,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_CardSkillUpgradelistRet {
        infos: Some("{}".to_string()),
        is_finish: Some(true),
    })])
}

#[func_reg] // 卡牌厂区信息
async fn __player_card_factory_info(
    _uid: u32,
    _msg: schemas::PlayerProto_CardFactoryInfo,
) -> anyhow::Result<HandlerResult> {
    let mut sel_infos = HashMap::new();
    let ids = vec![
        1, 2, 1012, 1020, 1015, 1007, 1004, 1061, 1008, 1010, 1006, 1034, 1032, 1031, 1035, 1030,
        1028, 1037, 1036, 1046, 1013, 1029, 1060, 1011, 1009, 1014,
    ];
    for id in ids {
        sel_infos.insert(
            id as u32,
            schemas::sNumInfo {
                id: Some(id),
                num: Some(0),
                _type: Some(0),
                ..Default::default()
            },
        );
    }

    Ok(vec![wrap(schemas::PlayerProto_CardFactoryInfoRet {
        firt_create_infos: Some(HashMap::new()),
        sel_infos: Some(sel_infos),
        daily_use_cnt: Some(0),
        sum_pool_cnts: Some(vec![]),
        create_cnts: Some("{}".to_string()),
        dy_open_pool: Some("{}".to_string()),
        free_cnt: Some(1),
        buildings: Some(vec![]),
        waitings: Some(vec![]),
        finishs: Some(vec![]),
        choice_infos: Some(HashMap::new()),
    })])
}

#[func_reg] // 卡牌建造
async fn __player_card_create(
    _uid: u32,
    _msg: schemas::PlayerProto_CardCreate,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_CardCreateFinishRet {
        infos: Some(vec![
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![schemas::sNumInfo {
                    id: Some(10033),
                    num: Some(1),
                    _type: Some(2),
                    ..Default::default()
                }]),
            },
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![
                    schemas::sNumInfo {
                        id: Some(107801),
                        num: Some(1),
                        _type: Some(2),
                        ..Default::default()
                    },
                    schemas::sNumInfo {
                        id: Some(10033),
                        num: Some(10),
                        _type: Some(2),
                        ..Default::default()
                    },
                ]),
            },
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![
                    schemas::sNumInfo {
                        id: Some(107801),
                        num: Some(1),
                        _type: Some(2),
                        ..Default::default()
                    },
                    schemas::sNumInfo {
                        id: Some(10033),
                        num: Some(10),
                        _type: Some(2),
                        ..Default::default()
                    },
                ]),
            },
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![
                    schemas::sNumInfo {
                        id: Some(107801),
                        num: Some(1),
                        _type: Some(2),
                        ..Default::default()
                    },
                    schemas::sNumInfo {
                        id: Some(10033),
                        num: Some(10),
                        _type: Some(2),
                        ..Default::default()
                    },
                ]),
            },
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![
                    schemas::sNumInfo {
                        id: Some(107801),
                        num: Some(1),
                        _type: Some(2),
                        ..Default::default()
                    },
                    schemas::sNumInfo {
                        id: Some(10033),
                        num: Some(10),
                        _type: Some(2),
                        ..Default::default()
                    },
                ]),
            },
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![
                    schemas::sNumInfo {
                        id: Some(107801),
                        num: Some(1),
                        _type: Some(2),
                        ..Default::default()
                    },
                    schemas::sNumInfo {
                        id: Some(10033),
                        num: Some(10),
                        _type: Some(2),
                        ..Default::default()
                    },
                ]),
            },
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![
                    schemas::sNumInfo {
                        id: Some(107801),
                        num: Some(1),
                        _type: Some(2),
                        ..Default::default()
                    },
                    schemas::sNumInfo {
                        id: Some(10033),
                        num: Some(10),
                        _type: Some(2),
                        ..Default::default()
                    },
                ]),
            },
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![
                    schemas::sNumInfo {
                        id: Some(107801),
                        num: Some(1),
                        _type: Some(2),
                        ..Default::default()
                    },
                    schemas::sNumInfo {
                        id: Some(10033),
                        num: Some(10),
                        _type: Some(2),
                        ..Default::default()
                    },
                ]),
            },
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![
                    schemas::sNumInfo {
                        id: Some(107801),
                        num: Some(1),
                        _type: Some(2),
                        ..Default::default()
                    },
                    schemas::sNumInfo {
                        id: Some(10033),
                        num: Some(10),
                        _type: Some(2),
                        ..Default::default()
                    },
                ]),
            },
            schemas::sCardPoolRewardInfo {
                id: Some(78010),
                num: Some(1),
                items: Some(vec![
                    schemas::sNumInfo {
                        id: Some(107801),
                        num: Some(1),
                        _type: Some(2),
                        ..Default::default()
                    },
                    schemas::sNumInfo {
                        id: Some(10033),
                        num: Some(15),
                        _type: Some(2),
                        ..Default::default()
                    },
                ]),
            },
        ]),
        cnt: Some(_msg.cnt.unwrap_or(0)),
        create_cnt: Some(_msg.cnt.unwrap_or(0) as u32),
        quality_up: Some(vec![6, 6]),
        card_pool_id: Some(_msg.card_pool_id.unwrap_or(0) as u16),
        daily_use_cnt: Some(0),
        costs: Some(vec![schemas::sNumInfo {
            id: Some(11002),
            num: Some(10),
            _type: Some(2),
            ..Default::default()
        }]),
    })])
}

#[func_reg] // 获取首次抽卡logs
async fn __player_first_card_create_logs(
    _uid: u32,
    _msg: schemas::PlayerProto_FirstCardCreateLogs,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_FirstCardCreateLogsRet {
        logs: Some(HashMap::new()),
    })])
}

#[func_reg] // 获取生活buf
async fn __player_get_life_buff(
    _uid: u32,
    _msg: schemas::PlayerProto_GetLifeBuff,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_GetLifeBuffRet {
        buffs: Some(HashMap::new()),
    })])
}

#[func_reg] // 获取可用头像id
async fn __player_get_skins(
    _uid: u32,
    _msg: schemas::PlayerProto_GetSkins,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_GetSkinsRet {
        info: Some(vec![
            schemas::sSkin {
                cfgid: Some(60110),
                info: Some(vec![6011003]),
                is_add: Some(false),
                ltSkins: Some(vec![]),
            },
            schemas::sSkin {
                cfgid: Some(40010),
                info: Some(vec![4001003]),
                is_add: Some(false),
                ltSkins: Some(vec![]),
            },
        ]),
    })])
}

#[func_reg] // 获取角色
async fn __player_get_card_role(
    _uid: u32,
    _msg: schemas::PlayerProto_GetCardRole,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_UpdateCardRole {
        roles: Some(vec![]),
        is_finish: Some(true),
    })])
}

#[func_reg] // 获取扫荡数据
async fn __player_duplicate_mod_up_data(
    _uid: u32,
    _msg: schemas::PlayerProto_DuplicateModUpData,
) -> anyhow::Result<HandlerResult> {
    let ids = vec![
        1103, 1105, 1106, 1004, 1007, 1002, 1101, 1104, 1003, 1001, 1006, 1005, 1102, 1008, 1107,
    ];
    let mod_up_data = ids
        .into_iter()
        .map(|id| schemas::ModUpDataItemData {
            id: Some(id),
            isOpenModUp: Some(true),
            modUpCount: Some(if id == 1001 { 569 } else { 0 }),
            ..Default::default()
        })
        .collect();

    Ok(vec![wrap(schemas::PlayerProto_DuplicateModUpDataRet {
        modUpData: Some(mod_up_data),
    })])
}

#[func_reg] // 新皮肤获记录
async fn __player_pay_reward(
    _uid: u32,
    _msg: schemas::PlayerProto_PayReward,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_PayRewardRet {
        rewards: Some(vec![]),
        is_finish: Some(true),
        is_notice: Some(false),
    })])
}

#[func_reg] // 获取今天的剩余重置次数
async fn __player_get_new_tower_reset_cnt(
    _uid: u32,
    _msg: schemas::PlayerProto_GetNewTowerResetCnt,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_GetNewTowerResetCntRet {
        reset_cnt: Some(vec![
            schemas::sStrNumInfo {
                sid: Some(7001),
                num: Some(1),
            },
            schemas::sStrNumInfo {
                sid: Some(7002),
                num: Some(1),
            },
        ]),
    })])
}

#[func_reg] // 获取特殊掉落掉落数量以及上限
async fn __player_get_special_drops_info(
    _uid: u32,
    _msg: schemas::PlayerProto_GetSpecialDropsInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_GetSpecialDropsInfoRet {
        dropInfos: Some(vec![]),
    })])
}

#[func_reg] // 十二宫获取挑战开启信息
async fn __player_get_star_palace_info(
    _uid: u32,
    _msg: schemas::PlayerProto_GetStarPalaceInfo,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_GetStarPalaceInfoRet {
        infos: Some(vec![
            schemas::sStarPalaceInfo {
                groupId: Some(9007),
                ranks: Some(0),
                starDupInfos: Some(vec![]),
                next_refresh_time: Some(1767315600),
            },
            schemas::sStarPalaceInfo {
                groupId: Some(9008),
                ranks: Some(0),
                starDupInfos: Some(vec![]),
                next_refresh_time: Some(1767315600),
            },
            schemas::sStarPalaceInfo {
                groupId: Some(9006),
                ranks: Some(0),
                starDupInfos: Some(vec![]),
                next_refresh_time: Some(1767315600),
            },
        ]),
        isReset: Some(true),
        stopTime: Some(0),
        boss_hp: Some(0),
        dupId: Some(0),
    })])
}

#[func_reg] // 获取累充的领取状态
async fn __player_get_collet_data(
    _uid: u32,
    _msg: schemas::PlayerProto_GetColletData,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_GetColletDataRet {
        score: Some(0),
        data: Some(vec![]),
    })])
}

#[func_reg] // 看板信息
async fn __player_get_new_panel(
    _uid: u32,
    _msg: schemas::PlayerProto_GetNewPanel,
) -> anyhow::Result<HandlerResult> {
    let mut panels = HashMap::new();
    panels.insert(
        1,
        schemas::sNewPanel {
            idx: Some(1),
            ids: Some(vec![7101001]),
            detail1: Some(r#"{live2d=false,top=true,x=0,scale=1,y=0}"#.to_string()),
            bg: Some(1),
            ty: Some(1),
            ..Default::default()
        },
    );

    Ok(vec![
        wrap(schemas::PlayerProto_GetRandomPanelRet {
            random_panels: Some(HashMap::new()),
            random_idx: Some(1000),
            finish: Some(true),
        }),
        wrap(schemas::PlayerProto_GetNewPanelRet {
            panels: Some(panels),
            setting: Some(0),
            random: Some(0),
            using: Some(1),
            update_time: Some(1767314861),
            random_type: Some(1),
            ..Default::default()
        }),
    ])
}

#[func_reg] // 看板信息设置
async fn __player_set_new_panel(
    _uid: u32,
    _msg: schemas::PlayerProto_SetNewPanel,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_GetNewPanelRet {
        panels: _msg.panels,
        setting: _msg.setting,
        random: _msg.random,
        using: _msg.using,
        update_time: Some(1728156510),
        random_type: Some(1),
        ..Default::default()
    })])
}

#[func_reg] // 设置随机看板
async fn __player_set_random_panel(
    _uid: u32,
    _msg: schemas::PlayerProto_SetRandomPanel,
) -> anyhow::Result<HandlerResult> {
    let random_idx = if let Some(ref p) = _msg.random_panel {
        p.idx.unwrap_or(0)
    } else {
        0
    };
    Ok(vec![wrap(schemas::PlayerProto_SetRandomPanelRet {
        random_panel: _msg.random_panel,
        random_idx: Some(random_idx),
    })])
}

#[func_reg] // 获取已拥有的音乐
async fn __player_get_all_music(
    _uid: u32,
    _msg: schemas::PlayerProto_GetAllMusic,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_GetAllMusicRet {
        data: Some(vec![]),
        is_finish: Some(true),
    })])
}

#[func_reg] // 设置
async fn __player_setting(
    _uid: u32,
    _msg: schemas::PlayerProto_Setting,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![wrap(schemas::PlayerProto_SettingRet {
        res: Some(true),
    })])
}

#[func_reg] // 获取累充的领取状态 (按类型)
async fn __player_get_collet_data_by_type(
    _uid: u32,
    _msg: schemas::PlayerProto_GetColletDataByType,
) -> anyhow::Result<HandlerResult> {
    Ok(vec![
        wrap(schemas::PlayerProto_GetColletDataRet {
            score: Some(0),
            data: Some(vec![]),
        }),
        wrap(schemas::PlayerProto_GetColletDataByTypeRet {
            _type: Some(2),
            openTime: Some(0),
            closeTime: Some(0),
            score: Some(0),
            data: Some(vec![]),
        }),
        wrap(schemas::PlayerProto_GetColletDataByTypeRet {
            _type: Some(3),
            openTime: Some(1753840800),
            closeTime: Some(1756839600),
            score: Some(0),
            data: Some(vec![]),
        }),
    ])
}

#[func_reg] // 客户端获取自定义数据
async fn __player_get_client_data(
    _uid: u32,
    _msg: schemas::PlayerProto_GetClientData,
) -> anyhow::Result<HandlerResult> {
    let mut res = schemas::PlayerProto_GetClientDataRet {
        key: _msg.key.clone(),
        ..Default::default()
    };

    if let Some(ref key) = _msg.key {
        if key == "plot_data" {
            res.data = Some(
                r#"{line_10052=10052,line_10051=10051,line_10054=10054,line_1=10115}"#.to_string(),
            );
            res._type = Some(3);
        } else if key == "passiveRed_isLook" {
            res._type = Some(4);
        } else if key == "guide_data_key" {
            res.data = Some(r#"{k1150=1,k1140=1,k10=1,k40=1,k30=1,k1080=1,k113=1,k1090=1,k1=1,k1130=1,k1120=1,k120=1,k105=1,k115=1,k110=1,k1110=1,k50=1,k60=1}"#.to_string());
            res._type = Some(3);
        }
    }

    Ok(vec![wrap(res)])
}
