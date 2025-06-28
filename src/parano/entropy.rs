use super::*;

pub trait Entropy {
    type Error;
    
    fn obtain_u64(&mut self)->Result<u64,Self::Error>;

    fn obtain_quad(&mut self)->Result<Q,Self::Error> {
        let k1 = self.obtain_u64()?;
        let k2 = self.obtain_u64()?;
        let k3 = self.obtain_u64()?;
        let k4 = self.obtain_u64()?;
        Ok([k1,k2,k3,k4])
    }
}
