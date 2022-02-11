use anyhow::{anyhow, Context, Result};
use indicatif::ProgressBar;
use serialport::SerialPort;
use std::io::{self, Write};
use std::ops::{Deref, DerefMut};
use std::thread::{self, JoinHandle};
use std::{fs::File, io::Read};

pub struct Term {
    port: Box<dyn SerialPort>,
}

impl Term {
    /// Open a new serial port
    pub fn open(path: &str) -> Result<Self> {
        let tty = serialport::new(path, 115200)
            .open()
            .context("Could not open serial port")?;

        Ok(Term::new(tty))
    }

    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self { port }
    }

    /// Flash a file to the md407
    pub fn flash(&mut self, file: &mut File) -> Result<u64> {
        let len = file.metadata()?.len();
        let pb = ProgressBar::new(len as u64);
        self.load()?;

        let mut port = self.port.try_clone()?;
        let handle: JoinHandle<Result<()>> = thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                let n = match port.read(&mut buf) {
                    Ok(n) => {
                        if n == 0 {
                            Err(anyhow!("Port closed"))?;
                        }
                        n
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::Interrupted {
                            continue;
                        }
                        return Err(e.into());
                    }
                };
                let res = &buf[..n];

                if res.ends_with(b"e") {
                    break;
                }
            }
            Ok(())
        });

        let res = io::copy(&mut pb.wrap_read(file), &mut self.port)
            .context("Could not write to serial port");

        match handle.join().unwrap().and(res) {
            Ok(res) => {
                pb.finish_with_message("Flash complete");
                Ok(res)
            }
            Err(err) => {
                pb.abandon_with_message("Flash failed");
                Err(err)
            }
        }
    }

    pub fn tr(&mut self) -> Result<()> {
        self.port
            .write_all(b"tr\n")
            .context("Error sending tr command")
    }

    pub fn go(&mut self) -> Result<()> {
        self.port
            .write_all(b"go\n")
            .context("Error sending go command")
    }

    pub fn reg(&mut self) -> Result<()> {
        self.port
            .write_all(b"reg\n")
            .context("Error sending reg command")
    }

    pub fn dm(&mut self) -> Result<()> {
        self.port
            .write_all(b"dm\n")
            .context("Error sending dm command")
    }

    pub fn mm(&mut self) -> Result<()> {
        self.port
            .write_all(b"mm\n")
            .context("Error sending mm command")
    }

    pub fn dasm(&mut self) -> Result<()> {
        self.port
            .write_all(b"dasm\n")
            .context("Error sending dasm command")
    }

    pub fn bp(&mut self) -> Result<()> {
        self.port
            .write_all(b"bp\n")
            .context("Error sending bp command")
    }

    pub fn load(&mut self) -> Result<()> {
        self.port
            .write_all(b"load\n")
            .context("Error sending load command")
    }

    pub fn copy<R>(&mut self, reader: &mut R) -> Result<u64>
    where
        R: ?Sized,
        R: Read,
    {
        io::copy(reader, &mut self.port).context("Could not copy to terminal")
    }

    pub fn copy_to<W>(&mut self, writer: &mut W) -> Result<u64>
    where
        W: ?Sized,
        W: Write,
    {
        io::copy(&mut self.port, writer).context("Could not copy from terminal")
    }

    fn cmd(&mut self, cmd: &str) -> Result<()> {
        writeln!(self.port, "{}", cmd).context("Could not write to terminal")?;
        Ok(())
    }

    pub fn interactive(&mut self) -> Result<()> {
        let mut buf = [0u8; 80];

        loop {
            let n = self.port.read(&mut buf);

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
    type Target = dyn SerialPort;

    fn deref(&self) -> &Self::Target {
        self.port.deref()
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.port.deref_mut()
    }
}
