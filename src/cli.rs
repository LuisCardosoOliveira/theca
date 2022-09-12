use clap::Parser;
use std::path::PathBuf;

use crate::errors::Errcode;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Activate debug mode
    #[clap(short = 'l', long = "log", default_value = "debug")]
    pub log_level: String,

    /// Command to execute inside the container
    #[clap(short, long)]
    pub command: String,

    /// User ID to create inside the container
    #[clap(short, long)]
    pub uid: u32,

    /// Directory to mount as root of the container
    #[clap(short = 'm', long = "mount")]
    pub mount_dir: PathBuf,
}

pub fn parse_args() -> Result<Args, Errcode> {
    let args = Args::parse();
    if !args.mount_dir.exists() || !args.mount_dir.is_dir() {
        return Err(Errcode::InvalidDir(args.mount_dir));
    }
    Ok(args)
}
