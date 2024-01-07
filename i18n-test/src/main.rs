#[macro_use]
extern crate lazy_static;

use rs_i18n::{I18ns, Loader, UseI18n};
mod i18n_dict;

use i18n_dict::I18nDict;

lazy_static! {
    static ref LOADER: Loader = Loader::new(Some("./i18n"));
}
fn main() {
    // print!("{:?}", I18nDict::HELLO);
    // let mut i18n = UseI18n::new(&LOADER, None);
    let mut i18n = UseI18n::new(
        &LOADER,
        Some("E:\\Rust\\try\\rs-i18n-all\\i18n-test\\src\\i18n_dict.rs"),
    );

    i18n.set_lang(I18ns::ENUS);
    // println!("{}", i18n.t("JOY"));
    println!("{}", i18n.t(&format!("{:?}", I18nDict::HAPPY)));
}
