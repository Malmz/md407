mod pick_device;
mod term;
mod util;

use std::{
    fs::{File, OpenOptions},
    io::{self, prelude::*},
    os::unix::prelude::{CommandExt, IntoRawFd},
    path::{self, Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Result};
use pico_args::Arguments;
use rustyline::Editor;
use term::Term;
use termios::{
    cfsetspeed, os::linux::B115200, tcsetattr, Termios, ECHO, ICANON, ICRNL, IGNPAR, ISIG, OPOST,
    TCSANOW,
};

use crate::pick_device::pick_device;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    let subcommand = args.subcommand()?.unwrap_or(String::new());
    let path = get_tty_path(&mut args).context("Could not connect to a tty")?;
    let mut tty = Term::open(&path)?;
    let file = file_from_args(&mut args);

    match &*subcommand {
        "load" => load(&mut tty, &mut file?).context("Load command failed")?,
        "go" => go(&mut tty).context("Go command failed")?,
        "run" => run(&mut tty, &mut file?).context("Run command failed")?,
        "picocom" => picocom(&path)?,
        "read" => read(&mut tty)?,
        "" => interactive(&mut tty)?,
        _ => bail!("invalid subcommand"),
    }

    println!("done");

    Ok(())
}

fn interactive(tty: &mut Term) -> Result<()> {
    tty.interactive()?;
    Ok(())
    /* let mut rl = Editor::<()>::new();
    let _ = rl.load_history("history.txt");

    loop {
        let readline = rl.readline("dbg: ");
        match readline.as_deref() {
            Ok(line) => {
                rl.add_history_entry(line);
                println!("Line: {}", line);
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }

    rl.save_history("history.txt")?;

    Ok(()) */
}

fn read(tty: &mut Term) -> Result<()> {
    let mut out = File::create("log")?;
    tty.copy_to(&mut out)?;
    Ok(())
}

fn load(tty: &mut Term, source: &mut File) -> Result<()> {
    tty.load()?;
    tty.copy(source)?;
    Ok(())
}

fn go(tty: &mut Term) -> Result<()> {
    tty.go()?;
    Ok(())
}

fn run(tty: &mut Term, source: &mut File) -> Result<()> {
    tty.load()?;
    tty.copy(source)?;
    tty.go()?;
    Ok(())
}

fn picocom(path: &Path) -> Result<()> {
    exec_interactive(path)?;
    Ok(())
}

fn _setup_terminal(args: &mut Arguments) -> Result<()> {
    let dev_path: PathBuf = args.free_from_str()?;
    let dev = File::open(dev_path)?;
    let fd = dev.into_raw_fd();
    let mut termios = Termios::from_fd(fd)?;

    termios.c_iflag |= IGNPAR;
    termios.c_iflag &= !(ICRNL);
    termios.c_oflag &= !(OPOST);
    termios.c_lflag &= !(ECHO | ICANON | ISIG);

    cfsetspeed(&mut termios, B115200)?;
    tcsetattr(fd, TCSANOW, &termios)?;

    Ok(())
}

fn file_from_args(args: &mut Arguments) -> Result<File> {
    let file_path: PathBuf = args.free_from_str()?;
    let file = File::open(file_path).context("Could not open file")?;
    Ok(file)
}

fn get_tty_path(args: &mut Arguments) -> Result<PathBuf> {
    let pick = args.contains("-p");
    let dev_path: PathBuf = match args.opt_value_from_str("--tty")? {
        Some(path) => path,
        None => pick_device(pick)?,
    };
    Ok(dev_path)
}

fn exec_interactive(path: &Path) -> Result<()> {
    Err(Command::new("picocom")
        .arg(path)
        .arg("-b")
        .arg("115200")
        .arg("--imap")
        .arg("lfcrlf")
        .exec())?
}
