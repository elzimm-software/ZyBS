use crate::ZyBSResult;

#[derive(Copy, Clone, Debug)]
pub enum Wizard {
    Screen,
    ZydocEntry,
}

pub fn new<T: AsRef<str>>(build_system: Option<T>, wizard: Wizard) -> ZyBSResult {
    todo!()
}