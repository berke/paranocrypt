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

    /// Total bytes
    len:u64,

    /// The HMAC value
    hmac:Block,

    /// Direction
    decrypt:bool
}

#[derive(Clone,Debug)]
pub struct CipherOutcome {
    /// Total number of plaintext bytes
    pub len:u64,

    /// Computed HMAC
    pub hmac:Q
}

impl Cipher
{
    pub fn new(k0:Key,state:CipherState,decrypt:bool)->Self
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

            len:0,

            hmac:[[0;4];2],

            decrypt
        }
    }

    fn flush_hmac(&mut self) {
        // We've finished a block, update HMAC
        let mut hm = bytes_to_block(&self.stream[..]);
        // let mut hm = [[0;4];2];
        hm = xor_block(hm,self.hmac);
        self.hmac = self.tr.transform(hm);
    }

    /// Apply (that is, xor) the keystream to the buffer
    /// As this is a stream cipher, this operation is used for encryption
    /// as well as decryption
    fn transform(&mut self,buf:&mut [u8],decrypt:bool) {
        let m = buf.len();
        for b in buf {
            if self.i == B {
                // We've finished a block, update HMAC
                self.flush_hmac();
                
                let Self { i,t,h,pos,tr,stream,.. } = self;
                
                // Increment key use counter
                *t += 1;

                // Is this traffic key spent?
                if *t == T {
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
                block_to_bytes(y,stream);

                // Reset the stream position
                *i = 0;
            }

            let Self { i,stream,.. } = self;

            // Transform the next byte
            let nb = *b ^ stream[*i];

            // Store ciphertext byte for HMAC
            if decrypt {
                stream[*i] = *b;
            } else {
                stream[*i] = nb;
            }

            // Write modified byte
            *b = nb;
            
            // Increment
            *i += 1;
        }
        self.len += m as u64;
    }

    pub fn hmac(&mut self)->CipherOutcome {
        let len = self.len;
        if self.i < B || true {
            let mut zeroes = [0;B];
            self.transform(&mut zeroes[self.i ..],false);
        }
        for _ in 0..8 {
            let mut len = len.to_le_bytes();
            self.transform(&mut len[..],false);
        }
        self.flush_hmac();
        let hmac = self.hmac[0];
        CipherOutcome {
            len,
            hmac
        }
    }

    pub fn process_stream<R,W>(mut self,
                               input:&mut R,
                               output:&mut W,
                               buf:&mut [u8])->
        Result<CipherOutcome,std::io::Error>
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
            self.transform(&mut buf[0..m],self.decrypt);
            output.write_all(&buf[0..m])?;
            total += m;
        }
        Ok(self.hmac())
    }
}
