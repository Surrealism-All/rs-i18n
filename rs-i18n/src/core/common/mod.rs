use self::names::I18ns;
use crate::Loader;
use serde_json::Value;
use std::collections::HashMap;
use std::{fs, mem};

pub mod names;

pub struct UseI18n<'a> {
    lang: I18ns,
    data_source: HashMap<String, String>,
    loader: &'a Loader,
}

impl<'a> UseI18n<'a> {
    pub fn new(loader: &'a Loader) -> Self {
        let lang = loader.target().lock().unwrap().clone();
        let data_source = UseI18n::get_data_source(loader);
        UseI18n {
            lang: lang.clone(),
            data_source,
            loader,
        }
    }
    fn get_data_source(loader: &'a Loader) -> HashMap<String, String> {
        let paths = loader.sources();
        let lang = loader.target().lock().unwrap().clone();
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
                .map(|(k, v)| (k, v.as_str().unwrap().to_string()))
                .collect::<Vec<(String, String)>>()
                .into_iter()
                .collect::<HashMap<String, String>>();
            return data_source;
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
    pub fn set_lang(&mut self, lang: I18ns) {
        let _ = self.loader.set_target(lang.clone());
        //reload
        self.data_source = UseI18n::get_data_source(self.loader);
        self.lang = lang;
    }
}
