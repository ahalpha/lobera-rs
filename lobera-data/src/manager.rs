use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct DataTable<T> {
    data: Vec<T>,
    indexes: HashMap<Vec<String>, HashMap<String, Vec<usize>>>,
}

impl<T: DeserializeOwned + Clone> DataTable<T> {
    pub fn load(table_name: &str) -> Result<Self> {
        let mut path = PathBuf::from("data/tables").join(format!("{}.json", table_name));
        if !path.exists() {
            path = PathBuf::from("../data/tables").join(format!("{}.json", table_name));
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read table file: {:?}", path))?;
        let data: Vec<T> = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse table JSON: {}", table_name))?;
        Ok(Self {
            data,
            indexes: HashMap::new(),
        })
    }

    pub fn create_index<F>(&mut self, keys: Vec<String>, extractor: F)
    where
        F: Fn(&T) -> Vec<Value>,
    {
        let mut index = HashMap::new();
        for (idx, item) in self.data.iter().enumerate() {
            let values = extractor(item);
            let key = serde_json::to_string(&values).unwrap_or_default();
            index.entry(key).or_insert_with(Vec::new).push(idx);
        }
        self.indexes.insert(keys, index);
    }

    pub fn find_one(&self, keys: &[String], values: &[Value]) -> Option<&T> {
        let key = serde_json::to_string(values).unwrap_or_default();
        self.indexes.get(keys).and_then(|index| {
            index
                .get(&key)
                .and_then(|indices| indices.first().map(|&idx| &self.data[idx]))
        })
    }

    pub fn find(&self, keys: &[String], values: &[Value]) -> Vec<&T> {
        let key = serde_json::to_string(values).unwrap_or_default();
        self.indexes
            .get(keys)
            .and_then(|index| {
                index
                    .get(&key)
                    .map(|indices| indices.iter().map(|&idx| &self.data[idx]).collect())
            })
            .unwrap_or_default()
    }

    pub fn all(&self) -> &[T] {
        &self.data
    }
}
