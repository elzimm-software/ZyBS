use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use crate::zyl::screen::ScreenKind;

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
    build_system: String,
    build_cmd: Vec<Command>,
    // description: String, // is this more of a .zydoc thing?
    global_sigil: char,
    arg_sigil: char,
    default_sep: String,
    disallow_interpolated_paths: bool,
    outputs: Vec<Output>,
    default_output: usize,
    directories: Vec<Directory>,
    vars: HashMap<String, Variable>,
    screens: Vec<Screen>,
}