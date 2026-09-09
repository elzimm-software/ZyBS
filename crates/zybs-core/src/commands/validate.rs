use crate::ZyBSResult;

#[derive(Copy, Clone, Debug, Default)]
pub enum Scope {
    #[default]
    All,
    ZylOnly,
    ZydocOnly,
}

pub fn validate<T: AsRef<str>>(
    build_system: Option<T>,
    fix: bool,
    strict: bool,
    scope: Scope,
) -> ZyBSResult {
    todo!()
}
