mod parano;
mod urandom;

use anyhow::{
    bail,
    Result,
};
use pico_args::Arguments;
use std::{
    fs::File,
    ffi::OsString,
    io::{
        BufWriter,
        IsTerminal,
    },
    os::fd::{
        AsRawFd,
        FromRawFd
    },
};

use urandom::Urandom;

fn main()->Result<()> {
    let mut args = Arguments::from_env();

    let key_file : OsString = args.value_from_str("--key")?;
    let decrypt = args.contains("--decrypt");
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut input = stdin.lock();
    let stdout = stdout.lock();
    if stdout.is_terminal() {
        bail!("Will not write to a TTY");
    }
    let stdout = stdout.as_raw_fd();
    let stdout = unsafe { File::from_raw_fd(stdout) };
    let mut output = BufWriter::new(stdout);

    let k0 = parano::load_hex_key(&key_file)?;
    let mut buf = vec![0;16384];

    let state =
        if decrypt {
            parano::CipherState::read(&mut input)?
        } else {
            let mut urnd = Urandom::new()?;
            let state = parano::CipherState::read(urnd.file_mut())?;
            state.write(&mut output)?;
            state
        };

    let mut cipher = parano::Cipher::new(k0,state);
    cipher.process_stream(&mut input,&mut output,&mut buf[..])?;

    Ok(())
}
