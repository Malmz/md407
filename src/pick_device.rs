use std::{fs::read_dir, path::PathBuf, sync::Arc, thread};

use anyhow::{ensure, Context, Result};
use serialport::SerialPortType;
use skim::{prelude::unbounded, Skim, SkimItem, SkimItemReceiver, SkimItemSender, SkimOptions};

struct Item {
    path: PathBuf,
}

impl SkimItem for Item {
    fn text(&self) -> std::borrow::Cow<str> {
        self.path.file_name().unwrap().to_string_lossy().into()
    }
}

pub fn pick_device(always_pick: bool) -> Result<String> {
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    let ports = serialport::available_ports().context("No serial ports found")?;

    thread::spawn(move || {
        for port in ports {
            if let SerialPortType::UsbPort(info) = port.port_type {
                dbg!(info);
            }
            let _ = tx.send(Arc::new(port.port_name));
        }
    });

    let options = SkimOptions {
        exit0: true,
        select1: !always_pick,
        ..Default::default()
    };

    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    Ok(selected_items
        .first()
        .context("No device selected")?
        .output()
        .to_string())
}

pub fn pick_device_old(always_pick: bool) -> Result<PathBuf> {
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for port in serialport::available_ports().context("No serial ports found")? {
        if let SerialPortType::UsbPort(info) = port.port_type {
            dbg!(info);
            let _ = tx.send(Arc::new(Item {
                path: port.port_name.into(),
            }));
        }
    }

    // let dir = read_dir("/dev")?;

    /* for p in dir {
        let p = p?;
        let path = p.path();

        let is_ttyusb = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("ttyUSB");

        if is_ttyusb {
            tx.send(Arc::new(Item { path }))
                .context("Failed to send item")?;
        }
    } */

    drop(tx);

    ensure!(!rx.is_empty(), "No ttyUSB found");

    if rx.len() > 1 || always_pick {
        let options = SkimOptions::default();

        let selected_items = Skim::run_with(&options, Some(rx))
            .map(|out| out.selected_items)
            .unwrap_or_else(|| Vec::new());

        let item = selected_items.first().context("No device selected")?;
        let item: &Item = item.as_any().downcast_ref().unwrap();
        Ok(item.path.clone())
    } else {
        let item = rx.recv().unwrap();
        let item: &Item = item.as_any().downcast_ref().unwrap();
        Ok(item.path.clone())
    }
}
