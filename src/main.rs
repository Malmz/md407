mod pick_device;

use std::{
    fs::{File, OpenOptions},
    io::{self, prelude::*},
    os::unix::prelude::CommandExt,
    path::PathBuf,
    process::Command,
};

use anyhow::{bail, Context, Result};
use pico_args::Arguments;

use crate::pick_device::pick_device;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    let subcommand = args.subcommand()?.context("No subcommand")?;

    match &*subcommand {
        "load" => load(&mut args).context("Load command failed")?,
        "go" => go(&mut args).context("Go command failed")?,
        "run" => run(&mut args).context("Run command failed")?,
        "interactive" => interactive(&mut args)?,
        _ => bail!("Unknown subcommand"),
    }

    Ok(())
}

fn load(args: &mut Arguments) -> Result<()> {
    let source_path: PathBuf = args.free_from_str()?;
    let mut source = File::open(source_path).context("Could not open source file")?;
    let mut tty = get_device(args)?;

    writeln!(tty, "load").context("Could not write to terminal")?;
    io::copy(&mut source, &mut tty).context("Could not write to terminal")?;

    Ok(())
}

fn go(args: &mut Arguments) -> Result<()> {
    let mut tty = get_device(args)?;
    writeln!(tty, "go").context("Could not write to terminal")?;
    Ok(())
}

fn run(args: &mut Arguments) -> Result<()> {
    let source_path: PathBuf = args.free_from_str()?;
    let mut source = File::open(source_path).context("Could not open source file")?;
    let mut tty = get_device(args)?;

    writeln!(tty, "load").context("Could not write to terminal")?;
    io::copy(&mut source, &mut tty).context("Could not write to terminal")?;
    writeln!(tty, "go").context("Could not write to terminal")?;

    Ok(())
}

fn interactive(args: &mut Arguments) -> Result<()> {
    let dev_path: PathBuf = match args.opt_free_from_str()? {
        Some(path) => path,
        None => pick_device()?,
    };

    Err(Command::new("picocom")
        .arg(dev_path)
        .arg("-b")
        .arg("115200")
        .arg("--imap")
        .arg("lfcrlf")
        .exec())?
}

fn get_device_path(args: &mut Arguments) -> Result<PathBuf> {
    let dev_path: PathBuf = match args.opt_free_from_str()? {
        Some(path) => path,
        None => pick_device()?,
    };
    Ok(dev_path)
}

fn get_device(args: &mut Arguments) -> Result<File> {
    let dev_path = get_device_path(args)?;
    let tty = OpenOptions::new()
        .write(true)
        .open(dev_path)
        .context("Could not open serial terminal")?;
    Ok(tty)
}
