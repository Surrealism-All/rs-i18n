#[macro_use]
extern crate lazy_static;

use rs_i18n::{I18ns, Loader, UseI18n};

lazy_static! {
    static ref LOADER: Loader = Loader::new();
}
fn main() {
    let mut i18n = UseI18n::new(&LOADER);
    i18n.set_lang(I18ns::ENUS);
    println!("{}", i18n.t("HELLO"));
}
