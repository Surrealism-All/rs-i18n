use std::{collections::HashMap, fs::File, io::Write, path::Path};

pub struct Dev;

impl Dev {
    pub fn build(data_source: HashMap<String, String>, target_file: &Path) {
        let res = data_source
            .iter()
            .map(|(k, v)| {
                format!(
                    r#"
                    #[strum(serialize = "{v}", to_string = "{v}")]
                    {k}"#
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        let res = format!(
            "use strum::EnumString;\n#[derive(Debug, EnumString, PartialEq, Eq)]\npub enum I18nDict{}{}\n{}",
            "{",res,"}"
        );
        //write into file
        let _ = match File::create(target_file) {
            Ok(mut f) => f.write(res.as_bytes()),
            Err(_) => panic!("{}", "Cannot create file when dev"),
        };
    }
}

#[cfg(test)]
mod test_dev {
    use std::{collections::HashMap, path::PathBuf};

    use super::Dev;

    #[test]
    fn test_build() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("HELLO"), String::from("hello"));
        map.insert(String::from("YES"), String::from("yes"));
        let _ = Dev::build(
            map,
            PathBuf::from("E:\\Rust\\try\\rs-i18n-all\\i18n-test\\src\\i18n_dict.rs").as_path(),
        );
    }
}
