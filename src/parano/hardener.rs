use super::*;

/// A hardener is a key derivation method
pub struct Hardener {
    tr:Transform,
    qs:Vec<Block>,
}

impl Hardener {
    /// Create a new hardener using the given transform and random value
    pub fn new(tr:Transform,r:Q)->Self {
        let qs = vec![[[0;4];2];N];
        let mut this = Self {
            tr,
            qs
        };
        this.init(r);
        this
    }

    /// Re-key and re-initialize this hardener
    pub fn rekey(&mut self,k:Key,r:Q) {
        self.tr.rekey(k);
        self.init(r);
    }

    /// (Re-)initialize the hardener with a given random value
    pub fn init(&mut self,r:Q) {
        let mut xy : Block = [[0;4];2];

        for i in 0..N {
            for xyj in xy.iter_mut() {
                for (xyjk,rk) in xyj.iter_mut().zip(r.iter()) {
                    *xyjk = (*xyjk).wrapping_add(rk ^ i as W);
                }
            }
            xy = self.tr.transform(xy);
            self.qs[i] = xy;
        }
    }

    /// Get the derived key
    pub fn get(&self)->Key {
        self.qs[0][1]
    }
    
    /// Step the hardener
    pub fn step(&mut self) {
        let Self { qs,.. } = self;

        let mut qs_tot = qs[0];
        
        let mut p = qs_tot[0][0] & (N - 1) as W;

        for _ in 0..H {
            let qsp = qs[p as usize];
            qs_tot = xor_block(qs_tot,qsp);
            let mut w = qs[p as usize][0][0];
            let p_next = qs[p as usize][0][1] & (N - 1) as W;

            if w & 1 == 0 {
                w >>= 1;
                let i = w & (N - 1) as W;
                qs[i as usize] = self.tr.transform(qs[i as usize]);
            } else {
                w >>= 1;
                let i1 = w & (N - 1) as W;
                w >>= LG2_N;
                let i2 = w & (N - 1) as W;
                qs.swap(i1 as usize,i2 as usize);
            }

            p = p_next;
        }

        qs.swap(0,p as usize);
        qs[0] = xor_block(qs[0],qs_tot);
    }
}
