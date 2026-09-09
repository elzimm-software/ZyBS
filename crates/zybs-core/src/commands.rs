#![allow(unused_imports)]

use crate::zybs_error::ZybsError;

pub(crate) mod build;
pub(crate) mod dirs;
pub(crate) mod fmt;
pub(crate) mod generate;
pub(crate) mod init;
pub(crate) mod modules;
pub(crate) mod new;
pub(crate) mod peel;
pub(crate) mod screens;
pub(crate) mod show;
pub(crate) mod tree;
pub(crate) mod validate;
pub(crate) mod zybs_return;

pub type ZyBSResult = Result<(), ZybsError>;
