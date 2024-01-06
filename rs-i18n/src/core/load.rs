use super::common::names::I18ns;
use super::constants::{TARGET_CONFIG_NAME, TARGET_DIR};
use serde_json::{self, Value};
use std::fs;
use std::{
    env::{self, current_dir},
    fs::{read_dir, DirEntry, File},
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug)]
pub struct Loader {
    sources: Vec<DirEntry>,
    target: I18ns,
    sys_lang: I18ns,
}

impl Default for Loader {
    fn default() -> Self {
        Self {
            sources: Default::default(),
            target: Default::default(),
            sys_lang: Default::default(),
        }
    }
}

impl Loader {
    /// Build a new Loader with configurations
    pub fn new() -> Self {
        Loader::load(Loader::get_configuration().as_path())
    }
    /// Build a new Loader with specified target source path
    pub fn load(source: &Path) -> Self {
        let sources = read_dir(source)
            .unwrap()
            .into_iter()
            .map(|dir| dir.unwrap())
            .collect::<Vec<DirEntry>>();
        let sys_lang = Loader::get_sys_lang();
        Loader {
            sources,
            target: sys_lang.clone(),
            sys_lang,
        }
    }
    /// get system language from current system
    pub fn get_sys_lang() -> I18ns {
        // such as zh_CN.UTF-8
        let lang = env::var("LANG").unwrap();
        //get 5 char
        let lang_spl = lang.split_at(5).0;
        I18ns::from_str(lang_spl).unwrap()
    }
    /// get configuration and return source dir path
    pub fn get_configuration() -> PathBuf {
        let mut config_path = current_dir().unwrap();
        config_path.push(TARGET_CONFIG_NAME);
        match File::open(config_path.as_path()) {
            Ok(mut file) => {
                let mut buffer = String::new();
                let _ = file.read_to_string(&mut buffer);
                //parse config source path
                let json_value: Value = serde_json::from_str(&buffer).unwrap();
                let mut i18n_path = current_dir().unwrap();
                match json_value.get("source") {
                    Some(f_path) => {
                        let f_path = fs::canonicalize(f_path.as_str().unwrap());
                        i18n_path.push(f_path.unwrap().as_path())
                    }
                    None => i18n_path.push(TARGET_DIR),
                };
                i18n_path
            }
            Err(e) => panic!("{}", e),
        }
    }
}

#[cfg(test)]
mod test_loader {
    use super::*;
    #[test]
    fn test_get_sys_lang() {
        let lang = Loader::get_sys_lang();
        dbg!(lang.to_string());
    }
    #[test]
    fn test_new_loader() {
        let loader = Loader::new();
        dbg!(loader);
    }
    #[test]
    fn test_load_loader() {
        let loader = Loader::load(Path::new("E:\\Rust\\try\\rs-i18n-all\\i18n-test\\i18n"));
        dbg!(loader);
    }
    #[test]
    fn test_get_configuration() {
        let loader = Loader::get_configuration();
        dbg!(loader);
    }
}
