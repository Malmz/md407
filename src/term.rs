use std::fs::OpenOptions;

use std::io::{self, Write};
use std::ops::{Deref, DerefMut};
use std::os::unix::prelude::OpenOptionsExt;
use std::path::Path;
use std::{fs::File, io::Read};

use anyhow::{Context, Result};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use libc::O_NONBLOCK;

pub struct Term {
    dev: File,
}

impl Term {
    pub fn open(path: &Path) -> Result<Self> {
        let tty = OpenOptions::new()
            .write(true)
            .read(true)
            .custom_flags(O_NONBLOCK)
            .open(path)
            .context("Could not open serial terminal")?;
        Ok(Term::new(tty))
    }

    pub fn new(dev: File) -> Self {
        Self { dev }
    }

    pub fn tr(&mut self) -> Result<()> {
        self.cmd("tr").context("tr command")
    }

    pub fn go(&mut self) -> Result<()> {
        self.cmd("go").context("go command")
    }

    pub fn reg(&mut self) -> Result<()> {
        self.cmd("reg").context("reg command")
    }

    pub fn dm(&mut self) -> Result<()> {
        self.cmd("dm").context("dm command")
    }

    pub fn mm(&mut self) -> Result<()> {
        self.cmd("mm").context("mm command")
    }

    pub fn dasm(&mut self) -> Result<()> {
        self.cmd("dasm").context("dasm command")
    }

    pub fn bp(&mut self) -> Result<()> {
        self.cmd("bp").context("bp command")
    }

    pub fn load(&mut self) -> Result<()> {
        self.cmd("load").context("load command")
    }

    pub fn copy<R>(&mut self, reader: &mut R) -> Result<u64>
    where
        R: ?Sized,
        R: Read,
    {
        io::copy(reader, &mut self.dev).context("Could not copy to terminal")
    }

    pub fn copy_to<W>(&mut self, writer: &mut W) -> Result<u64>
    where
        W: ?Sized,
        W: Write,
    {
        io::copy(&mut self.dev, writer).context("Could not copy from terminal")
    }

    fn cmd(&mut self, cmd: &str) -> Result<()> {
        writeln!(self.dev, "{}", cmd).context("Could not write to terminal")?;
        Ok(())
    }

    pub fn interactive(&mut self) -> Result<()> {
        let mut buf = [0u8; 80];

        loop {
            let n = self.dev.read(&mut buf);

            if let Err(ref e) = n {
                dbg!(e);
            }
            let n = n?;
            if n > 0 {
                println!("{:?}", &buf[..n]);
            }
        }

        //enable_raw_mode()?;

        //disable_raw_mode()?;

        //Ok(())
    }
}

impl Deref for Term {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.dev
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dev
    }
}

/* impl Read for Term {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.dev.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.dev.read_vectored(bufs)
    }
}

impl Write for Term {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.dev.write(buf)
    }
} */
