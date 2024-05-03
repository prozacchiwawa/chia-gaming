use lazy_static::lazy_static;
use crate::common::types::{Sha256Input, Hash};

lazy_static! {
    pub static ref Q_KW_TREEHASH: Hash = Sha256Input::Array(vec![
        Sha256Input::Bytes(&[1]),
        Sha256Input::Bytes(&[1]),
    ]).hash();
}

pub const CREATE_COIN: u32 = 51;
pub const REM: u32 = 1;

pub const AGG_SIG_UNSAFE_ATOM: [u8; 1] = [49];
pub const AGG_SIG_ME_ATOM: [u8; 1] = [50];
pub const CREATE_COIN_ATOM: [u8; 1] = [51];
pub const REM_ATOM: [u8; 1] = [1];

pub const GROUP_ORDER: [u8; 32] = [
    0x73,
    0xED,
    0xA7,
    0x53,
    0x29,
    0x9D,
    0x7D,
    0x48,
    0x33,
    0x39,
    0xD8,
    0x08,
    0x09,
    0xA1,
    0xD8,
    0x05,
    0x53,
    0xBD,
    0xA4,
    0x02,
    0xFF,
    0xFE,
    0x5B,
    0xFE,
    0xFF,
    0xFF,
    0xFF,
    0xFF,
    0x00,
    0x00,
    0x00,
    0x01
];

pub const DEFAULT_HIDDEN_PUZZLE_HASH: [u8; 32] = [
    0x71,
    0x1d,
    0x6c,
    0x4e,
    0x32,
    0xc9,
    0x2e,
    0x53,
    0x17,
    0x9b,
    0x19,
    0x94,
    0x84,
    0xcf,
    0x8c,
    0x89,
    0x75,
    0x42,
    0xbc,
    0x57,
    0xf2,
    0xb2,
    0x25,
    0x82,
    0x79,
    0x9f,
    0x9d,
    0x65,
    0x7e,
    0xec,
    0x46,
    0x99,
];

pub const DEFAULT_PUZZLE_HASH: [u8; 32] = [
    0xe9,
    0xaa,
    0xa4,
    0x9f,
    0x45,
    0xba,
    0xd5,
    0xc8,
    0x89,
    0xb8,
    0x6e,
    0xe3,
    0x34,
    0x15,
    0x50,
    0xc1,
    0x55,
    0xcf,
    0xdd,
    0x10,
    0xc3,
    0xa6,
    0x75,
    0x7d,
    0xe6,
    0x18,
    0xd2,
    0x06,
    0x12,
    0xff,
    0xfd,
    0x52,
];

pub const AGG_SIG_ME_ADDITIONAL_DATA: [u8; 32] = [
    0xcc,
    0xd5,
    0xbb,
    0x71,
    0x18,
    0x35,
    0x32,
    0xbf,
    0xf2,
    0x20,
    0xba,
    0x46,
    0xc2,
    0x68,
    0x99,
    0x1a,
    0x3f,
    0xf0,
    0x7e,
    0xb3,
    0x58,
    0xe8,
    0x25,
    0x5a,
    0x65,
    0xc3,
    0x0a,
    0x2d,
    0xce,
    0x0e,
    0x5f,
    0xbb
];

pub const ONE: [u8; 1] = [1];
pub const TWO: [u8; 1] = [2];

pub const Q_KW: [u8; 1] = [1];
pub const A_KW: [u8; 1] = [2];
pub const C_KW: [u8; 1] = [4];

