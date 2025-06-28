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

// Quad-word (8x4 = 32 bytes)
pub type Q = [W;4];

// Block (64 bytes)
pub type Block = [Q;2];

pub type Key = Q;

// Number of rounds
const R : usize = 16;

// Bytes per block
const B : usize = std::mem::size_of::<Block>();

// Size of hardener memory table, in blocks
const LG2_N : u32 = 16;
const N : usize = 1 << LG2_N;

// Number of hardening rounds per step
const H : usize = 1 << 6;

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

pub use utils::{
    load_hex_key,
    show_quad
};
