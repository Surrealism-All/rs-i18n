use super::common::I18ns;
use std::path::{Path, PathBuf};
struct Loader {
    sources: Vec<PathBuf>,
    target: I18ns,
}
