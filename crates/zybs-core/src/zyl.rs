use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;
use crate::zyl::screen::ScreenKind;

pub(crate) enum LazyOr<T> {
    Lazy((String, Box<dyn Fn(String) -> T>)),
    Value(T),
}

impl<T> LazyOr<T> {

    pub fn new(raw: String, parser: impl Fn(String) -> T + 'static) -> Self {
        Self::Lazy((raw, Box::new(parser)))
    }

    pub fn from(val: T) -> Self {
        Self::Value(val)
    }

    pub fn get(self) -> T {
        match self {
            LazyOr::Lazy((s, f)) => {
                f(s)
            }
            LazyOr::Value(v) => {v}
        }
    }
}

pub(crate) struct Output {
    path: PathBuf,
    buckets: Vec<String>,
}

pub(crate) struct FilePat {
    var: String,
    pattern: String, // TODO: create real pattern type to avoid reparsing strings
}

pub(crate) struct Directory {
    path: PathBuf,
    file_pats: Vec<FilePat>
}

pub(crate) enum Variable {
    Path(PathBuf),
    String(String),
    List(Vec<Variable>),
}

pub(crate) struct Screen {
    params: Vec<String>,
    bucket: String,
    screen_kind: ScreenKind,
}

mod screen {
    use std::path::PathBuf;
    use std::process::Command;

    pub(crate) enum ScreenKind {
        Leaf(Leaf),
        Compose(Compose),
    }

    pub(crate) struct Leaf {
        commands: Vec<Command>,
        output: PathBuf,
        uses_directories: Vec<PathBuf>,
    }

    pub(crate) struct Compose {
        subscreens: Vec<String>
    }
}

pub(crate) struct Zyl {
    // skipping format version for MVP, still 1.0 goal
    build_system: LazyOr<String>,
    build_cmd: LazyOr<Vec<Command>>,
    // description: String, // is this more of a .zydoc thing?
    global_sigil: LazyOr<char>,
    arg_sigil: LazyOr<char>,
    default_sep: LazyOr<String>,
    disallow_interpolated_paths: LazyOr<bool>,
    outputs: LazyOr<Vec<Output>>,
    default_output: LazyOr<usize>,
    directories: LazyOr<Vec<Directory>>,
    vars: LazyOr<HashMap<String, Variable>>,
    screens: LazyOr<Vec<Screen>>,
}