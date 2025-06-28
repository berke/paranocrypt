mod cipher;
mod hardener;
pub mod utils;
mod transform;

use std::{
    fs::File,
    io::{
        Read,
        Write
    },
    path::Path,
};

// Word size
pub type W = u64;

// Quad-word
pub type Q = [W;4];

// Block
pub type Block = [Q;2];

pub type Key = Q;

// Number of rounds
const R : usize = 128;

// Bytes per block
const B : usize = std::mem::size_of::<Block>();

// Number of blocks we can generate from a given key before it is
// "spent"
const T : usize = 256;

use hardener::*;
use transform::*;
use utils::*;

pub use cipher::{
    CipherState,
    Cipher,
};

pub use utils::load_hex_key;
