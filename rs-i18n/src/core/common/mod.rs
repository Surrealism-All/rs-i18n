use self::dev::Dev;
use self::names::I18ns;
use crate::Loader;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub mod dev;
pub mod names;

/// ## I18n core
/// ### example
/// ```rs
/// #[macro_use]
/// extern crate lazy_static;
///
/// use rs_i18n::{I18ns, Loader, UseI18n};
///
/// lazy_static! {
///    static ref LOADER: Loader = Loader::new();
/// }
/// fn main() {
///    let mut i18n = UseI18n::new(&LOADER);
///    i18n.set_lang(I18ns::ENUS);
///    println!("{}", i18n.t("HELLO"));
/// }
/// ````
pub struct UseI18n<'a> {
    lang: I18ns,
    data_source: HashMap<String, String>,
    loader: &'a Loader,
    dev_path: Option<&'a str>,
}

impl<'a> UseI18n<'a> {
    pub fn new(loader: &'a Loader, dev_path: Option<&'a str>) -> Self {
        let lang = loader.target().lock().unwrap().clone();
        let data_source = UseI18n::get_data_source(loader);
        match dev_path {
            Some(target_file) => Dev::build(data_source.clone(), Path::new(target_file)),
            None => (),
        };
        UseI18n {
            lang: lang.clone(),
            data_source,
            loader,
            dev_path,
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
        match self.dev_path {
            Some(target_file) => Dev::build(self.data_source.clone(), Path::new(target_file)),
            None => (),
        };
        self.lang = lang;
    }
    pub fn data_source(&self) -> &HashMap<String, String> {
        &self.data_source
    }
}
