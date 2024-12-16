//! An alternative to the [`probe-run`](https://github.com/knurling-rs/probe-run) printer,
//! used by [`defmt`](https://github.com/knurling-rs/defmt).
//! Parses data sent by QEMU over semihosting (ARM Cortex-M only).
//! *Printers* are *host* programs that receive log data, format it and display
//! it.
//!
//! Addapted from DEFMT's `qemu-run` crate.

use std::env;
use std::fs;
use std::io::Read as _;
use std::process::Command;
use std::process::Stdio;
use std::process::{self};

use anyhow::anyhow;
use anyhow::bail;
use defmt_decoder::DecodeError;
use defmt_decoder::StreamDecoder;
use defmt_decoder::Table;
use process::Child;

fn main() -> Result<(), anyhow::Error> {
    notmain().map(|opt_code| {
        if let Some(code) = opt_code {
            process::exit(code);
        }
    })
}

fn notmain() -> Result<Option<i32>, anyhow::Error> {
    let args = env::args().skip(1 /* program name */).collect::<Vec<_>>();

    if args.len() != 1 {
        bail!("expected exactly one argument. Syntax: `lemur <path-to-elf>`");
    }

    let path = &args[0];
    let bytes = fs::read(path)?;

    let table = if env::var_os("LEMUR_IGNORE_VERSION").is_some() {
        Table::parse_ignore_version(&bytes)
    } else {
        Table::parse(&bytes)
    };
    let table = table?.ok_or_else(|| anyhow!("`.defmt` section not found"))?;

    let mut child = KillOnDrop(
        Command::new("qemu-system-arm")
            .args([
                "-cpu",
                "cortex-m7",
                "-machine",
                "mps2-an500",
                "-nographic",
                "-monitor",
                "none",
                "-semihosting-config",
                "enable=on,target=native",
                "-kernel",
            ])
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Error running qemu-system-arm; perhaps you haven't installed it yet?"),
    );

    let mut stdout = child
        .0
        .stdout
        .take()
        .ok_or_else(|| anyhow!("failed to acquire child's stdout handle"))?;

    let mut decoder = table.new_stream_decoder();

    let mut readbuf = [0; 256];
    let exit_code;
    loop {
        let n = stdout.read(&mut readbuf)?;
        decoder.received(&readbuf[..n]);
        decode(&mut *decoder)?;

        if let Some(status) = child.0.try_wait()? {
            exit_code = status.code();

            let mut data = Vec::new();
            stdout.read_to_end(&mut data)?;
            decoder.received(&data);
            decode(&mut *decoder)?;

            break;
        }
    }

    Ok(exit_code)
}

fn decode(decoder: &mut dyn StreamDecoder) -> Result<(), DecodeError> {
    loop {
        match decoder.decode() {
            Ok(frame) => {
                println!("{}", frame.display(true))
            }
            Err(DecodeError::UnexpectedEof) => return Ok(()),
            Err(DecodeError::Malformed) => {
                eprintln!("failed to decode defmt data");
                return Err(DecodeError::Malformed);
            }
        }
    }
}

struct KillOnDrop(Child);

impl Drop for KillOnDrop {
    fn drop(&mut self) {
        self.0.kill().ok();
    }
}
