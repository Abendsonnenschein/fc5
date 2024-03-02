use serde_json::Value;
use std::collections::HashMap;

pub(crate) struct Config {
    pub sources: HashMap<String, Vec<String>>,
    pub overrides: Vec<(String, bool)>,
}

impl Config {
    pub(crate) fn new(json: &str) -> Self {
        let v: Value = serde_json::from_str(json).unwrap();

        let sources = v["sources"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_str().unwrap().to_string())
                        .collect(),
                )
            })
            .collect();

        let overrides = v["overrides"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                (
                    v["name"].as_str().unwrap().to_string(),
                    v["ok"].as_bool().unwrap(),
                )
            })
            .collect();

        Config { sources, overrides }
    }
}
