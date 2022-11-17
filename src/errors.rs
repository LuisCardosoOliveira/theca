use std::{path::PathBuf, process::exit};

#[derive(thiserror::Error, Debug)]
pub enum Errcode {
    #[error("The mount directory `{0}` passed does not exist or is not a dir. Please check the given path. ")]
    InvalidDir(PathBuf),
    #[error("Unknown error. please check the logs.")]
    _Unknown,
    #[error("An error in the container has occurred.")]
    ContainerError(u8),
    #[error("This is not supported.")]
    NotSupported(u8),
    #[error("The argument pass is invalid.")]
    ArgumentInvalid(&'static str),
}

impl Errcode {
    /// Translate an `Errcode::X` into a number to return (the Unix way)
    pub fn get_retcode(&self) -> i32 {
        1_i32
    }
}

/// Get the result from a function, and exit the process with the correct error code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            tracing::info!("exit without any error, returning 0");
            exit(0);
        }
        Err(e) => {
            let retcode = e.get_retcode();
            tracing::error!("error on exit:\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
