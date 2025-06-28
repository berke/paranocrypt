use super::*;

pub trait Random {
    type Error;
    
    fn random_u64(&mut self)->Result<u64,Self::Error>;

    fn random_quad(&mut self)->Result<Q,Self::Error> {
        let k1 = self.random_u64()?;
        let k2 = self.random_u64()?;
        let k3 = self.random_u64()?;
        let k4 = self.random_u64()?;
        Ok([k1,k2,k3,k4])
    }
}
