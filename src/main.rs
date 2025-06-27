mod cipher;
mod urandom;

use anyhow::{
    anyhow,
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
        Read,
	Write
    },
    os::fd::{
        AsRawFd,
        FromRawFd
    },
    path::Path,
};

use cipher::*;
use urandom::Urandom;

fn load_key<P:AsRef<Path>>(path:P)->Result<Q> {
    let mut fd = File::open(path)?;
    let mut u = String::new();
    let _ = fd.read_to_string(&mut u)?;
    let v = u.trim();
    let mut key = [0;4];
    let mut vs = v.split('-');
    for j in 0..4 {
        let w = vs.next()
            .ok_or_else(|| anyhow!("Short key"))?;
        let k = W::from_str_radix(w,16)?;
        key[j] = k;
    }
    Ok(key)
}

fn write_u64s<W:Write>(w:&mut W,xs:&[u64])->Result<()> {
    for &x in xs {
        let y = x.to_le_bytes();
        w.write_all(&y[..])?;
    }
    Ok(())
}

fn read_u64s<R:Read>(r:&mut R,xs:&mut [u64])->Result<()> {
    let mut y = [0;8];
    for x in xs.iter_mut() {
        r.read_exact(&mut y[..])?;
        let y = u64::from_le_bytes(y);
        *x = y;
    }
    Ok(())
}

const T : usize = 256;
const B : usize = 64;

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

    let k0 = load_key(&key_file)?;

    let mut urnd = Urandom::new()?;

    let mut r1 : Q = [0;4];
    let mut r2 : Q = [0;4];
    let mut p0 : Q = [0;4];
    let mut p1 : Q = [0;4];

    if decrypt {
        read_u64s(&mut input,&mut r1[..])?;
        read_u64s(&mut input,&mut r2[..])?;
        read_u64s(&mut input,&mut p0[..])?;
        read_u64s(&mut input,&mut p1[..])?;
    } else {
        r1 = urnd.random_quad()?;
        r2 = urnd.random_quad()?;
        p0 = urnd.random_quad()?;
        p1 = urnd.random_quad()?;

        write_u64s(&mut output,&r1[..])?;
        write_u64s(&mut output,&r2[..])?;
        write_u64s(&mut output,&p0[..])?;
        write_u64s(&mut output,&p1[..])?;
    }
    
    let mut pos = [p0,p1];

    let mut c0 = Cipher::new(k0);
    c0.init(r1);
    let k1 = c0.get();

    let mut c1 = Cipher::new(k1);
    c1.init(r2);

    let mut buf = vec![0;16384];

    let mut stream = [0;B];
    let mut i = B;
    let mut t = T;
    let mut k2 = [0;4];

    loop {
        let m = input.read(&mut buf[..])?;
        if m == 0 {
            break;
        }

        for b in &mut buf[0..m] {
            if i == B {
                t += 1;
                if t == T {
                    c1.step();
                    k2 = c1.get();
                    t = 0;
                }
                
                let y = encrypt(k2,pos);
                pos[0][0] += 1;
                let mut n = 0;
                for j in 0..2 {
                    for k in 0..4 {
                        let z = y[j][k].to_le_bytes();
                        for l in 0..8 {
                            stream[n] = z[l];
                            n += 1;
                        }
                    }
                }
                i = 0;
            }

            *b ^= stream[i];
            i += 1;
        }

        output.write_all(&buf[0..m])?;
    }

    Ok(())
}
