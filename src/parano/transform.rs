use super::*;

/// A keyed transform
pub struct Transform {
    k:Key
}

fn f1(z:W,[a,b,c,d]:Q)->Q {
    match z % 11 {
        0 => [a,b,c,d],
        1 => [b,c,d,a],
        2 => [c,d,a,b],
        3 => [d,a,b,c],
        4 => [b,a,c,d],
        5 => [a^b,b^c,c^d,d^a],
        6 => [a.wrapping_add(b),
              b.wrapping_add(c),
              c.wrapping_add(d),
              d.wrapping_add(a)],
        7 => [a.wrapping_sub(b),
              b.wrapping_sub(c),
              c.wrapping_sub(d),
              d.wrapping_sub(a)],
        8 => [a.wrapping_mul(b) ^ c,
              b.wrapping_mul(c) ^ d,
              c.wrapping_mul(d) ^ a,
              d.wrapping_mul(a) ^ b],
        9 => [a.rotate_left((b & 63) as u32),
              b.rotate_left((c & 63) as u32),
              c.rotate_left((d & 63) as u32),
              d.rotate_left((a & 63) as u32)],
        _ => [a.rotate_left(7),
              b.rotate_left(19),
              c.rotate_left(41),
              d.rotate_left(53)],
    }
}

pub(crate) fn xor(mut q:Q,r:Q)->Q {
    for i in 0..4 {
        q[i] ^= r[i];
    }
    q
}

pub(crate) fn xor_block(q:Block,r:Block)->Block {
    [xor(q[0],r[0]),xor(q[1],r[1])]
}

fn f2(mut z:W,mut q0:Q)->Q {
    while z > 0 {
        q0 = f1(z,q0);
        z /= 11;
    }
    q0
}

fn f3([a,b,c,d]:Q)->[W;2] {
    [(a.wrapping_mul(b.wrapping_add(1537228672809128983)) % 4611686018427386911)
     .rotate_left(37)
     .wrapping_add(2305843009213693487),
     (c.wrapping_mul(d.wrapping_add(54255129628557493)) % 4611686018427385831)
     .rotate_right(37)
     .wrapping_add(922337203685477171)]
}

// Core function, transform a block using a Feistel construction
fn f(mut k:Key,mut x:Block)->Block {
    for _ in 0..R {
        k[0] = k[0].wrapping_add(0x3e304f694602defe);
        k[1] = k[1].wrapping_add(0x750032b57a2d17b1);
        k[2] = k[2].wrapping_add(0xff2afee4acb8d237);
        k[3] = k[3].wrapping_add(0x0e96955b0bb60320);
        for (x1j,kj) in x[1].iter_mut().zip(k.iter()) {
            *x1j ^= kj;
        }
        let c = f2(k[0],x[1]);
        x[0] = xor(x[0],c);
        x = [x[1],x[0]];
        let [s0,s1] = f3(k);
        k = [s0 ^ k[1],
             s1.wrapping_add(k[2]),
             s1 ^ k[3],
             s0.wrapping_add(k[0])];
    }
    x
}

impl Transform {
    pub fn new(k:Key)->Self {
        Self { k }
    }

    pub fn transform(&self,x:Block)->Block {
        f(self.k,x)
    }

    pub fn rekey(&mut self,k:Key) {
        self.k = k;
    }
}
