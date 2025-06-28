use super::*;

#[allow(dead_code)]
pub fn show_block(x:Block) {
    for xi in x.iter() {
        for (j,xij) in xi.iter().enumerate() {
            if j != 0 {
                print!(" ");
            }
            print!("{:016X}",xij);
            if j == 3 {
                println!();
            }
        }
    }
}

#[allow(dead_code)]
pub fn show_quad<W:Write>(w:&mut W,x:Q)->Result<(),std::io::Error> {
    for (j,xj) in x.iter().enumerate() {
        if j != 0 {
            write!(w,"-")?;
        }
        write!(w,"{:016X}",xj)?;
    }
    Ok(())
}

pub(crate) fn write_u64<W:Write>(w:&mut W,x:u64)->Result<(),std::io::Error> {
    let y = x.to_le_bytes();
    w.write_all(&y[..])?;
    Ok(())
}

pub(crate) fn write_quad<W:Write>(w:&mut W,[a,b,c,d]:Q)->Result<(),std::io::Error> {
    write_u64(w,a)?;
    write_u64(w,b)?;
    write_u64(w,c)?;
    write_u64(w,d)?;
    Ok(())
}

pub(crate) fn read_u64<R:Read>(r:&mut R)->Result<u64,std::io::Error> {
    let mut y = [0;8];
    r.read_exact(&mut y[..])?;
    let x = u64::from_le_bytes(y);
    Ok(x)
}

pub(crate) fn read_quad<R:Read>(r:&mut R)->Result<Q,std::io::Error> {
    let a = read_u64(r)?;
    let b = read_u64(r)?;
    let c = read_u64(r)?;
    let d = read_u64(r)?;
    Ok([a,b,c,d])
}

pub fn load_hex_key<P:AsRef<Path>>(path:P)->Result<Q,std::io::Error> {
    let mut fd = File::open(path)?;
    let mut u = String::new();
    let _ = fd.read_to_string(&mut u)?;
    let v = u.trim();
    let mut key = [0;4];
    let mut vs = v.split('-');
    for keyj in key.iter_mut() {
        let w = vs.next()
            .ok_or_else(|| std::io::Error::other("Short key"))?;
        let k = W::from_str_radix(w,16)
            .map_err(std::io::Error::other)?;
        *keyj = k;
    }
    if vs.next().is_some() {
        Err(std::io::Error::other("Junk after key"))
    } else {
        Ok(key)
    }
}

pub fn bytes_to_block(mut s:&[u8])->Block {
    std::array::from_fn(
        |_| std::array::from_fn(
            |_| {
                let y : [u8;8] = s[0..8].try_into().unwrap();
                let x = u64::from_le_bytes(y);
                s = &s[8..];
                x
            })
    )
}

pub fn block_to_bytes(y:Block,mut d:&mut [u8]) {
    for yj in y.iter() {
        for yjk in yj.iter() {
            let z = yjk.to_le_bytes();
            for zl in z {
                d[0] = zl;
                d = &mut d[1..];
            }
        }
    }
}

#[test]
fn test_block_vs_bytes() {
    let mut b0 = [0;64];
    let mut b1 = [0;64];
    for k in 0 .. 64 {
        b0[k] = k as u8;
    }
    let y0 = bytes_to_block(&b0[..]);
    block_to_bytes(y0,&mut b1[..]);
    let y1 = bytes_to_block(&b1[..]);
    println!("y0 = {:?}",y0);
    println!("y1 = {:?}",y1);
    println!("b0 = {:?}",b0);
    println!("b1 = {:?}",b1);
    assert!(y0 == y1);
    assert!(b0 == b1);
}
