pub type W = u64;

pub type D = [W;2];

pub type Q = [W;4];

const R : usize = 128;

pub const LG2_N : u32 = 12;

pub const N : usize = 1 << LG2_N;

const H : usize = 1 << 12;

const W : usize = 64;

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

fn xor(mut q:Q,r:Q)->Q {
    for i in 0..4 {
        q[i] ^= r[i];
    }
    q
}

fn f2(mut z:W,mut q0:Q)->Q {
    while z > 0 {
        q0 = f1(z,q0);
        z /= 11;
    }
    q0
}

fn f3([a,b,c,d]:Q)->D {
    [(a.wrapping_mul(b.wrapping_add(1537228672809128983)) % 4611686018427386911)
     .rotate_left(37)
     .wrapping_add(2305843009213693487),
     (c.wrapping_mul(d.wrapping_add(54255129628557493)) % 4611686018427385831)
     .rotate_right(37)
     .wrapping_add(922337203685477171)]
}

pub fn encrypt(mut k:Q,mut x:[Q;2])->[Q;2] {
    for _ in 0..R {
        k[0] = k[0].wrapping_add(0x3e304f694602defe);
        k[1] = k[1].wrapping_add(0x750032b57a2d17b1);
        k[2] = k[2].wrapping_add(0xff2afee4acb8d237);
        k[3] = k[3].wrapping_add(0x0e96955b0bb60320);
        for j in 0..4 {
            x[1][j] ^= k[j];
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

pub struct Cipher {
    k:Q,
    qs:Vec<[Q;2]>,
}

impl Cipher {
    pub fn new(k:Q)->Self {
        let qs = vec![[[0;4];2];N];
        Self {
            k,
            qs
        }
    }

    pub fn init(&mut self,r:Q) {
        let mut xy = [[0;4];2];

        for i in 0..N {
            for j in 0..2 {
                for k in 0..4 {
                    xy[j][k] = r[k] ^ i as W;
                }
            }
            self.qs[i] = encrypt(self.k,xy);
        }

        for _ in 0..W {
            self.step();
        }
    }

    pub fn get(&self)->Q {
        self.qs[0][1]
    }
    
    pub fn step(&mut self) {
        let Self { qs,.. } = self;
        let mut p = qs[0][0][0] & (N - 1) as W;

        for _ in 0..H {
            let mut w = qs[p as usize][0][0];
            let p_next = qs[p as usize][0][1] & (N - 1) as W;
            let kk = qs[p as usize][1];

            if w & 1 == 0 {
                w >>= 1;
                let i = w & (N - 1) as W;
                qs[i as usize] = encrypt(kk,qs[i as usize]);
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
    }
}

pub fn show(x:[Q;2]) {
    for i in 0..2 {
        for j in 0..4 {
            if j != 0 {
                print!(" ");
            }
            print!("{:016X}",x[i][j]);
            if j == 3 {
                println!();
            }
        }
    }
}

pub fn show_q(x:Q) {
    for j in 0..4 {
        if j != 0 {
            print!(" ");
        }
        print!("{:016X}",x[j]);
    }
}
