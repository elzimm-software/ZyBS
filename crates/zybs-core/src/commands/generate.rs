use crate::commands::ZyBSResult;
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub enum Debug {
    True(Vec<String>),
    #[default]
    False,
}

pub fn generate<T: AsRef<str>, P: AsRef<Path>>(
    build_system: Option<T>,
    output: Option<P>,
    debug: Debug,
) -> ZyBSResult {
    todo!()
}