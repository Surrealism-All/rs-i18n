use self::names::I18ns;
use crate::Loader;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

pub mod names;

pub struct UseI18n {
    lang: I18ns,
    data_source: HashMap<String, String>,
}

impl UseI18n {
    pub fn new(loader: &Loader) -> Self {
        let paths = loader.sources();
        let lang = loader.target();
        let target = paths
            .iter()
            .find(|path| {
                let name = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".json", "");
                name.eq(&lang.to_string())
            })
            .unwrap();
        let target_str = fs::read_to_string(target.as_path()).unwrap();
        let value: Value = serde_json::from_str(&target_str).unwrap();
        if let Some(map) = value.as_object() {
            let data_source = map
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect::<Vec<(String, String)>>()
                .into_iter()
                .collect::<HashMap<String, String>>();
            UseI18n {
                lang: lang.clone(),
                data_source,
            }
        } else {
            panic!("Cannot parse target i18n json data");
        }
    }
    pub fn t(&self, from: &str) -> String {
        match self.data_source.get(from) {
            Some(res) => res.to_string(),
            None => String::from(from),
        }
    }
}
