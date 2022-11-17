use crate::cli::Args;
use crate::config::ContainerOpts;
use crate::errors::Errcode;

use nix::sys::utsname::uname;

pub struct Container {
    config: ContainerOpts,
}

impl Container {
    pub fn new(args: Args) -> Container {
        let config = ContainerOpts::new(args.command, args.uid, args.mount_dir);
        Self { config }
    }

    pub fn create(&mut self) -> Result<(), Errcode> {
        tracing::info!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Errcode> {
        tracing::info!("Cleaning container");
        Ok(())
    }
}

pub fn start(args: Args) -> Result<(), Errcode> {
    check_linux_version()?;
    let mut container = Container::new(args);
    if let Err(e) = container.create() {
        container.clean_exit()?;
        tracing::error!("Error while creating container: {:?}", e);
        return Err(e);
    }
    tracing::info!("Finished, cleaning & exit");
    container.clean_exit()
}

pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;

pub fn check_linux_version() -> Result<(), Errcode> {
    let host = uname().expect("An error occured while trying to fetch system information.");
    tracing::info!("Linux release: {:?}", host.release());

    if let Ok(version) = scan_fmt!(host.release().to_str().unwrap(), "{f}.{}", f32) {
        if version < MINIMAL_KERNEL_VERSION {
            return Err(Errcode::NotSupported(0));
        }
    } else {
        return Err(Errcode::ContainerError(0));
    }

    if host.machine() != "aarch64" {
        return Err(Errcode::NotSupported(1));
    }

    Ok(())
}
