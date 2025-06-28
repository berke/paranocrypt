use super::*;

pub struct Urandom {
    file:File
}

impl Urandom {
    pub fn new()->Result<Self> {
        let file = File::open("/dev/urandom")?;
        Ok(Self { file })
    }

    pub fn file_mut(&mut self)->&mut File {
        &mut self.file
    }
}

// impl Entropy for Urandom {
//     type Error = Error;

//     fn obtain_u64(&mut self)->Result<u64> {
//         let mut x = [0;8];
//         self.file.read_exact(&mut x[..])?;
//         Ok(u64::from_le_bytes(x))
//     }
// }
