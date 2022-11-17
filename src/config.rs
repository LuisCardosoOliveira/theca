use std::ffi::CString;
use std::path::PathBuf;

use anyhow::Ok;

use crate::errors::Errcode;

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
}

impl ContainerOpts {
    pub fn new(command: String, uid: u32, mount_dir: PathBuf) -> Self {
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg"))
            .collect();

        let path = argv.get(0).expect("Failed while indexing arg").clone();

        Self {
            path,
            argv,
            uid,
            mount_dir,
        }
    }
}
