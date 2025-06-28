use super::*;

pub struct CipherState {
    r0:Q,
    r1:Q,
    p0:Q,
    p1:Q
}

impl CipherState {
    pub fn read<R:Read>(input:&mut R)->Result<Self,std::io::Error> {
        let r0 = read_quad(input)?;
        let r1 = read_quad(input)?;
        let p0 = read_quad(input)?;
        let p1 = read_quad(input)?;

        Ok(Self { r0,r1,p0,p1 })
    }

    pub fn write<W:Write>(&self,output:&mut W)->Result<(),std::io::Error> {
        let Self { r0,r1,p0,p1 } = *self;
        write_quad(output,r0)?;
        write_quad(output,r1)?;
        write_quad(output,p0)?;
        write_quad(output,p1)?;
        Ok(())
    }
}

pub struct Cipher {
    /// The hardener
    h:Hardener,

    /// Position in keystream
    pos:Block,

    /// Current portion of keystream
    stream:[u8;B],

    /// Position in portion
    i:usize,

    /// Number of blocks until we have to rekey
    t:usize,

    /// The transform for the traffic
    tr:Transform,
}

impl Cipher
{
    pub fn new(k0:Key,state:CipherState)->Self
    {
        let CipherState { r0,r1,p0,p1 } = state;
        
        let pos = [p0,p1];

        // Create a transform with the master key
        let tr = Transform::new(k0);

        // Create a hardener
        let mut h = Hardener::new(tr,r0);

        // Obtain the first derived (session) key.  We do not want to
        // use the master key
        let k1 = h.get();

        // Re-key the hardener
        h.rekey(k1,r1);

        // Get the first traffic key
        let k2 = h.get();

        // Create the traffic transformer
        let tr = Transform::new(k2);
        
        Self {
            h,

            pos,

            tr,

            // Set the block position to maximum to force a
            // re-calculation on the first run
            i:B,

            stream:[0;B],

            // The current key has been freshly generated
            t:0,
        }
    }

    /// Apply (that is, xor) the keystream to the buffer
    /// As this is a stream cipher, this operation is used for encryption
    /// as well as decryption
    fn transform(&mut self,buf:&mut [u8]) {
        let Self { i,t,h,pos,tr,stream,.. } = self;

        for b in buf {
            if *i == B {
                *t += 1;
                if *t == T {
                    // This traffic key is spent

                    // Compute a new traffic key using the hardener
                    h.step();
                    let k = h.get();

                    // Re-key the traffic transform
                    tr.rekey(k);

                    // Re-se the usage counter
                    *t = 0;
                }
                
                // Get the next stream block
                let y = tr.transform(*pos);

                // Increment the position
                pos[0][0] += 1;

                // Fill the stream buffer
                let mut n = 0;
                for yj in y.iter() {
                    for yjk in yj.iter() {
                        let z = yjk.to_le_bytes();
                        for zl in z {
                            stream[n] = zl;
                            n += 1;
                        }
                    }
                }

                // Reset the stream position
                *i = 0;
            }

            // Encrypt the next byte
            *b ^= stream[*i];
            *i += 1;
        }
    }

    pub fn process_stream<R,W>(&mut self,
                               input:&mut R,
                               output:&mut W,
                               buf:&mut [u8])->
        Result<usize,std::io::Error>
    where
        R:Read,
        W:Write
    {
        let mut total = 0;
        loop {
            let m = input.read(buf)?;
            if m == 0 {
                break;
            }
            self.transform(&mut buf[0..m]);
            output.write_all(&buf[0..m])?;
            total += m;
        }
        Ok(total)
    }
}
