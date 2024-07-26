
use lazy_static::lazy_static;
use rand::Rng;
use super::super::*;
use once_cell::sync::Lazy;

pub const CASTLING_RIGHTS_INDEX: usize = 13*64;
pub const EP_INDEX: usize = 13 * 64 + 4;
pub const PLAYER_INDEX: usize = 13 * 64 + 4 + 8;

pub const WHITE_QUEENSIDE_CASTLING_MASK: u64 = 0b11111 << 56;
pub const WHITE_KINGSIDE_CASTLING_MASK: u64 = 0b1111 << 60;
pub const BLACK_QUEENSIDE_CASTLING_MASK: u64 = 0b11111;
pub const BLACK_KINGSIDE_CASTLING_MASK: u64 = 0b1111 << 4;

pub const WHITE_QUEENSIDE_FREE_CASTLING_MASK: u64 = 0b11111 << 56;
pub const WHITE_KINGSIDE_FREE_CASTLING_MASK: u64 = 0b1111 << 60;
pub const BLACK_QUEENSIDE_FREE_CASTLING_MASK: u64 = 0b01110;
pub const BLACK_KINGSIDE_FREE_CASTLING_MASK: u64 = 0b0110 << 4;

const fn p_rng(state: u128) -> (u128, u64) {
    let state = state * 0xaadec8c3186345282b4e141f3a1232d5;
    let mask = (1u128 << 64) - 1;
    return (state, (state & mask) as u64);
}

/// Bit-filled columns, used for masking columns.
pub const COLUMNS: [u64; 8] = {
    let mut masks = [0u64; 8];
    let mut i = 0;
    while i < 8 {
        let mut mask = 1u64 << i;
        let mut offset = 8;
        while offset < 64 {
            mask |= mask << offset;
            offset <<= 1;
        }
        masks[i] = mask;
        i += 1;
    }
    masks
};

/// Bit-filled rows, used for masking rows.
pub const ROWS: [u64; 8] = {
    let mut masks = [0u64; 8];
    let mut i = 0;
    while i < 8 {
        let mut mask = 1u64 << (i * 8);
        let mut offset = 1;
        while offset < 8 {
            mask |= mask << offset;
            offset <<= 1;
        }
        masks[i] = mask;
        i += 1;
    }
    masks
};

static ZOOBRIST_KEYS2: Lazy<[u64;13*64 + 4 + 8 + 1]> = Lazy::new(|| {
    let mut keys = [0u64; 13*64 + 4 + 8 + 1];
    let mut rng = rand::thread_rng();
    for i in 0..keys.len() {
        keys[i] = rng.gen::<u64>();
    }
    for i in 0..64 {
        keys[(Piece::Empty.to_u8() as usize) * 64 + i] = 0;
    }
    return keys;
});

// Lazy initialize some state
lazy_static! {
    /// Zoobrist keys
    pub static ref ZOOBRIST_KEYS: [u64;13*64 + 4 + 8 + 1] = {
        let mut keys = [0u64; 13*64 + 4 + 8 + 1];
        let mut rng = rand::thread_rng();
        for i in 0..keys.len() {
            keys[i] = rng.gen::<u64>();
        }
        for i in 0..64 {
            keys[(Piece::Empty.to_u8() as usize) * 64 + i] = 0;
        }
        return keys;
    };
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BitboardRuntimeConstants {
    bishop_magic_table: Vec<u64>
}

impl BitboardRuntimeConstants{
    pub fn create() -> BitboardRuntimeConstants {
        BitboardRuntimeConstants {
            bishop_magic_table: Self::create_magic_table()
        }
    }

    fn create_magic_table() -> Vec<u64> {
        return vec![];
    }
}