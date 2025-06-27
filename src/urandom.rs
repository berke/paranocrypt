use super::*;

pub struct Urandom {
    file:File
}

impl Urandom {
    pub fn new()->Result<Self> {
        let file = File::open("/dev/urandom")?;
        Ok(Self { file })
    }

    pub fn random_u64(&mut self)->Result<u64> {
        let mut x = [0;8];
        self.file.read_exact(&mut x[..])?;
        Ok(u64::from_le_bytes(x))
    }

    pub fn random_quad(&mut self)->Result<Q> {
        let k1 = self.random_u64()?;
        let k2 = self.random_u64()?;
        let k3 = self.random_u64()?;
        let k4 = self.random_u64()?;
        Ok([k1,k2,k3,k4])
    }
}
