pub const TARGET_CONFIG_NAME: &str = "i18n.config.json";

pub const TARGET_DIR: &str = "i18n";

/// when match str to I18ns Error
pub const ERR_FROM_STR_I18NS: &str = "Cannot Match This Language to I18Ns";

pub const DEV_TEMPLATE: &str = r#"
use strum::EnumString;

#[derive(Debug, EnumString, PartialEq, Eq)]
enum I18nDict {
    ${items},
}
"#;

pub const DEV_TEMPLATE_KV: &str = r#"#[strum(serialize = "${value}", to_string = "${value}")]
${key}"#;
