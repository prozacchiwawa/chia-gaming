use lazy_static::lazy_static;
use crate::common::types::PuzzleHash;
use crate::common::standard_coin::shatree_atom_cant_fail;

lazy_static! {
    pub static ref AGG_SIG_UNSAFE: [u8; 1] = [49];
    pub static ref AGG_SIG_ME: [u8; 1] = [50];
    pub static ref CREATE_COIN: [u8; 1] = [51];

    pub static ref GROUP_ORDER: Vec<u8> = {
        vec![
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
        ]
    };
    pub static ref DEFAULT_HIDDEN_PUZZLE_HASH: PuzzleHash = {
        let ph: [u8; 32] = [
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
        PuzzleHash::from_bytes(ph)
    };
    pub static ref DEFAULT_PUZZLE_HASH: PuzzleHash = {
        let ph: [u8; 32] = [
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
        PuzzleHash::from_bytes(ph)
    };

    pub static ref ONE_TREEHASH: PuzzleHash = {
        shatree_atom_cant_fail(&[1])
    };

    pub static ref Q_KW_TREEHASH: PuzzleHash = {
        shatree_atom_cant_fail(&[1])
    };

    pub static ref A_KW_TREEHASH: PuzzleHash = {
        shatree_atom_cant_fail(&[2])
    };

    pub static ref C_KW_TREEHASH: PuzzleHash = {
        shatree_atom_cant_fail(&[4])
    };

    pub static ref NULL_TREEHASH: PuzzleHash = {
        shatree_atom_cant_fail(&[])
    };

    pub static ref AGG_SIG_ME_ADDITIONAL_DATA: [u8; 32] = [
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
}
