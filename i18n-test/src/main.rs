#[macro_use]
extern crate lazy_static;

use rs_i18n::{Loader, UseI18n};

lazy_static! {
    static ref LOADER: Loader = Loader::new();
}
fn main() {
    // println!("{:#?}", LOADER.sources());
    let i18n = UseI18n::new(&LOADER);

    println!("{}", i18n.t("HELLO"));
}
