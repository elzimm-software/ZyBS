mod zybs_error;

use crate::validate::Scope;
use std::error::Error;
use std::path::Path;

type ZyBSResult = Result<(), Box<dyn Error>>;

pub fn init<T: AsRef<str>>(build_system: Option<T>) -> ZyBSResult {
    todo!()
}

pub fn build<T: AsRef<str>>(build_system: Option<T>) -> ZyBSResult {
    todo!()
}

pub mod generate {
    #[derive(Clone, Debug, Default)]
    pub enum Debug {
        True(Vec<String>),
        #[default]
        False,
    }
}

pub fn generate<T: AsRef<str>, P: AsRef<Path>>(
    build_system: Option<T>,
    output: Option<P>,
    debug: generate::Debug,
) -> ZyBSResult {
    todo!()
}

pub fn peel<T: AsRef<str>, U: AsRef<str>>(build_system: Option<T>, screen: U) -> ZyBSResult {
    todo!()
}

pub fn zybs_return<T: AsRef<str>, U: AsRef<str>>(build_system: Option<T>, screen: U) -> ZyBSResult {
    todo!()
}

pub fn tree<T: AsRef<str>>(build_system: Option<T>) -> ZyBSResult {
    todo!()
}

pub fn screens<T: AsRef<str>>(build_system: Option<T>) -> ZyBSResult {
    todo!()
}

pub fn show<T: AsRef<str>, U: AsRef<str>>(
    build_system: Option<T>,
    screen: U,
    resolved: bool,
) -> ZyBSResult {
    todo!()
}

pub fn modules<T: AsRef<str>>(build_system: Option<T>) -> ZyBSResult {
    todo!()
}

pub fn dirs<T: AsRef<str>, U: AsRef<str>>(build_system: Option<T>, screen: U) -> ZyBSResult {
    todo!()
}

pub mod validate {
    #[derive(Copy, Clone, Debug, Default)]
    pub enum Scope {
        #[default]
        All,
        ZylOnly,
        ZydocOnly,
    }
}

pub fn validate<T: AsRef<str>>(
    build_system: Option<T>,
    fix: bool,
    strict: bool,
    scope: Scope,
) -> ZyBSResult {
    todo!()
}

pub mod new {
    #[derive(Copy, Clone, Debug)]
    pub enum Wizard {
        Screen,
        ZydocEntry,
    }
}

pub fn new<T: AsRef<str>>(build_system: Option<T>, wizard: new::Wizard) -> ZyBSResult {
    todo!()
}

pub fn fmt<T: AsRef<str>>(build_system: Option<T>) -> ZyBSResult {
    todo!()
}
