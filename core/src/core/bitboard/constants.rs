#![allow(long_running_const_eval)]

use std::{cmp::max, collections::btree_map::Keys, default, io::empty};

use lazy_static::lazy_static;
use num::integer::Roots;
use rand::{seq::index, Rng};
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
    let state = state.wrapping_mul(0xaadec8c3186345282b4e141f3a1232d5);
    let val = state >> 64;
    return (state, val as u64);
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

pub const ZOOBRIST_KEYS : [u64; 13*64 + 4 + 8 + 1] = {
    let mut keys = [0u64; 13*64 + 4 + 8 + 1];
    let mut state: u128 = 23598723948723894;
    let mut key: u64;
    let mut i = 0;
    while i < 13*64 + 4 + 8 + 1 {
        (state, key) = p_rng(state);
        keys[i] = key;
        i += 1;
    }
    i = 0;
    while i < 64 {
        keys[12 * 64 + i] = 0;
        i += 1;
    }
    keys
};

const fn directional_shift(lhs: u64, rhs: i32) -> u64 {
    if rhs > 0 {
        return  lhs << rhs;
    } else {
        return  lhs >> (-rhs);
    }
}

/// Generates sliding piece moves
/// 
/// ```step_size``` - direction to move in
/// 
/// ```position``` - u64 with bit at occupied position set
/// 
/// ```occupancy``` - occupied squares
pub const fn help_bit_step(step_size: i32, mut position: u64, occupancy: u64) -> u64 {
    let mut stop_mask: u64 = match step_size {
        -1 | -9 | 7 => COLUMNS[7],
        1 | -7 | 9 => COLUMNS[0],
        8 | -8 => 0,
        _ => panic!("help_bit_step incorrect step_size")
    };
    stop_mask = stop_mask | match step_size {
        -9 | -8 | -7 => ROWS[7],
        7 | 8 | 9 => ROWS[0],
        -1 | 1 => 0,
        _ => panic!("help_bit_step incorrect step_size")
    };
    stop_mask = !stop_mask;
    let noccupancy = !occupancy;
    let mut result: u64 = 0;

    while position > 0 {
        position = directional_shift(position, step_size) & stop_mask;
        result |= position;
        position &= noccupancy;
    }
    return result;
}

pub const fn generate_bishop_moves_slow(position: u64, occupancy: u64) -> u64 {
    let mut result = help_bit_step(-7, position, occupancy);
    result |= help_bit_step(-9, position, occupancy);
    result |= help_bit_step(7, position, occupancy);
    result |= help_bit_step(9, position, occupancy);
    return result;
}

pub const fn generate_rook_moves_slow(position: u64, occupancy: u64) -> u64 {
    let mut result = help_bit_step(-1, position, occupancy);
    result |= help_bit_step(1, position, occupancy);
    result |= help_bit_step(8, position, occupancy);
    result |= help_bit_step(-8, position, occupancy);
    return result;
}

#[cfg(target_feature = "bmi2")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};

pub const BISHOP_MASKS: [u64; 64] = {
    let mut masks = [0u64; 64];
    let mut i: usize = 0;
    while i < 64 {
        let pos = 1u64 << i;
        let and_mask = !(COLUMNS[0] | ROWS[0] | COLUMNS[7] | ROWS[7]);
    
        masks[i] = generate_bishop_moves_slow(pos, 0) & and_mask;
        i += 1;
    }
    masks
};

pub const ROOK_MASKS: [u64; 64] = {
    let mut masks = [0u64; 64];
    let mut i: usize = 0;
    while i < 64 {
        let pos = 1u64 << i;
        let mut side_mask = help_bit_step(-1, pos, 0);
        side_mask |= help_bit_step(1, pos, 0);
        side_mask &= !(COLUMNS[0] | COLUMNS[7]);
        let mut height_mask = help_bit_step(8, pos, 0);
        height_mask |= help_bit_step(-8, pos, 0);
        height_mask &= !(ROWS[0] | ROWS[7]);


        //masks[i] = help_bit_step(step_size, position, occupancy)
        masks[i] = side_mask | height_mask;
        i += 1;
    }
    masks
};

pub const fn pext_const(x: u64, mask: u64) -> u64 {
    let mut mask = mask;
    let mut result_bit = 0;
    let mut result: u64 = 0;
    while mask > 0 {
        let mask_bit = mask.trailing_zeros();
        result |= ((x >> mask_bit) & 1) << result_bit;
        mask &= mask - 1;
        result_bit += 1;
    }
    return result;
}


/*fn generate_pext_bishop_table<const N: usize>(position: u8, occupancy: u64, bits: &mut Vec<usize>, keys: &mut Vec<u64>) {

    if  bits.len() > 0 {
        let top_bit = bits.pop().unwrap();
        generate_pext_bishop_table(position, occupancy, bits, keys);
        generate_pext_bishop_table(position, occupancy ^ (1u64 << top_bit), bits, keys);
        bits.push(top_bit);
    } else { 
        let i: usize = pext_const(occupancy, mask) as usize;
        keys[i] = generate_bishop_moves_slow(position_mask, occupancy);
    }
    return keys;
}   */

const fn generate_pext_rook_table<const N: usize>(position_mask: u64,mask: u64, occupancy: u64, max_index: usize, index: usize, bits: &[usize], mut keys: [u64;N]) -> [u64;N] {
    if index < max_index {
        keys = generate_pext_rook_table(position_mask, mask, occupancy, max_index, index + 1, bits, keys);
        keys = generate_pext_rook_table(position_mask, mask, occupancy ^ (1u64 << bits[index]), max_index, index + 1, bits, keys);
        
    } else {
        let i: usize = pext_const(occupancy, mask) as usize;
        keys[i] = generate_rook_moves_slow(position_mask, occupancy);
    }
    return keys;
}   


//#[cfg(target_feature = "bmi2")]
pub const PEXT_BISHOP_MAGIC: [[u64;512];64] = {
    let mut magic = [[0u64; 512]; 64];
    let mut i = 0;
    while i < 64 {
        let mut bits = [0usize; 9];
        let mut keys = [0u64; 512];

        let mut tmp = BISHOP_MASKS[i];
        let mut max_index = 0;
        while tmp > 0 {
            bits[max_index] = tmp.trailing_zeros() as usize;
            tmp &= tmp - 1;
            max_index += 1;
        }
        //magic[i] = generate_pext_bishop_table(1u64 << i,BISHOP_MASKS[i], 0, max_index, 0, &bits, keys);
        i += 1;
    }
    magic
};


//#[cfg(target_feature = "bmi2")]
pub const PEXT_ROOK_MAGIC: [[u64;4096];64] = {
    let mut magic = [[0u64; 4096]; 64];
    let mut i = 0;
    while i < 64 {
        let mut bits = [0usize; 12];
        let mut keys = [0u64; 4096];

        let mut tmp = ROOK_MASKS[i];
        let mut max_index = 0;
        while tmp > 0 {
            bits[max_index] = tmp.trailing_zeros() as usize;
            tmp &= tmp - 1;
            max_index += 1;
        }

        //magic[i] = generate_pext_rook_table(1u64 << i,ROOK_MASKS[i], 0, max_index, 0, &bits, keys);
        i += 1;
    }
    magic
};
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BitboardRuntimeConstants {
    pub bishop_magic_table: Vec<u64>,
    pub zoobrist_keys: [u64;13*64 + 4 + 8 + 1]
}

impl BitboardRuntimeConstants{
    pub fn create() -> BitboardRuntimeConstants {
        BitboardRuntimeConstants {
            bishop_magic_table: Self::create_magic_table(),
            zoobrist_keys: Self::create_zoobrist_keys()
        }
    }

    fn create_magic_table() -> Vec<u64> {
        return vec![];
    }

    fn create_zoobrist_keys() -> [u64;13*64 + 4 + 8 + 1] {
        let mut keys = [0u64; 13*64 + 4 + 8 + 1];
        let mut rng = rand::thread_rng();
        for i in 0..keys.len() {
            keys[i] = rng.gen::<u64>();
        }
        for i in 0..64 {
            keys[(Piece::Empty.to_u8() as usize) * 64 + i] = 0;
        }
        return keys;
    }
}

pub fn bishop_magic(position: u8, occupancy: u64) -> u64 {
    let mask = BISHOP_MASKS[position as usize];
    let key;
    #[cfg(target_feature = "bmi2")]
    unsafe {
        key = _pext_u64(occupancy, mask);
    }
    #[cfg(not(target_feature = "bmi2"))] 
    {
        key = pext_const(occupancy, mask);
    }
    return PEXT_BISHOP_MAGIC[position as usize][key as usize];
}
