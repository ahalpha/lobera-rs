mod manager;
use manager::DataTable;

#[cfg(test)]
mod tests;

use anyhow::Result;
use lazy_static::lazy_static;
use serde_json::Value;

lazy_static! {
    pub static ref TABLES: Tables = Tables::new().expect("Failed to load tables");
}

pub struct Tables {
    pub card_data: DataTable<Value>,
    pub skill: DataTable<Value>,
}

impl Tables {
    fn new() -> Result<Self> {
        let mut card_data = DataTable::<Value>::load("CardData")?;
        card_data.create_index(vec!["id".to_string()], |v| {
            vec![v.get("id").cloned().unwrap_or(Value::Null)]
        });

        let mut skill = DataTable::<Value>::load("Skill")?;
        skill.create_index(vec!["id".to_string()], |v| {
            vec![v.get("id").cloned().unwrap_or(Value::Null)]
        });

        Ok(Tables { card_data, skill })
    }
}

