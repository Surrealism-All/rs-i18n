mod core;

pub use core::common::{names::I18ns, UseI18n};
pub use core::Loader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_trans() {
        let loader = Loader::new();
        let mut i18n = UseI18n::new(&loader);
        i18n.set_lang(I18ns::ENUS);
        let hello_str = i18n.t("HELLO");
        println!("{}", hello_str);
    }
}
