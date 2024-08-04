#![allow(long_running_const_eval)]

use std::{io::{self, Write}, iter::zip};


use rand::Rng;
use super::super::*;

pub const CASTLING_RIGHTS_INDEX: usize = 13*64;
pub const EP_INDEX: usize = 13 * 64 + 4;
pub const PLAYER_INDEX: usize = 13 * 64 + 4 + 8;
#[cfg(any(test,debug_assertions))]
const NUMBER_OF_MAGIC_TABLE_TRIES: usize = 10;
#[cfg(not(any(test,debug_assertions)))]
const NUMBER_OF_MAGIC_TABLE_TRIES: usize = 100000;
#[cfg(any(test,debug_assertions))]
const MINIMUM_QUALITY: u32 = 5;
#[cfg(not(any(test,debug_assertions)))]
const MINIMUM_QUALITY: u32 = 1;

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





#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BitboardRuntimeConstants {
    bishop_magic_pext_table: Vec<Vec<u64>>,
    bishop_magic_magic_table: ([u8;64], [u64; 64], [Vec<u64>; 64]),
    rook_magic_magic_table: ([u8;64], [u64; 64], [Vec<u64>; 64]),
    rook_magic_pext_table: Vec<Vec<u64>>,
    pub zoobrist_keys: [u64;13*64 + 4 + 8 + 1]
}

impl BitboardRuntimeConstants{
    pub fn create() -> BitboardRuntimeConstants {
        BitboardRuntimeConstants {
            bishop_magic_pext_table: Self::generate_bishop_tables(),
            bishop_magic_magic_table: Self::generate_bishop_magic_numbers(),
            rook_magic_magic_table: Self::generate_rook_magic_numbers(),
            rook_magic_pext_table: Self::generate_rook_tables(),
            zoobrist_keys: Self::create_zoobrist_keys()
        }
    }

    /// Creates all possible variations of the bits set in full
    fn generate_all_occupancies(full: u64) -> Vec<u64> {
        fn help(full: u64, curr: u64, result: &mut Vec<u64>) {
            if full > 0 {
                //remove last bit
                let next_full = full & (full - 1);
                //Last bit of full as 1
                let lowest_bit = full ^ next_full;
                help(next_full, curr, result);
                //Toggle lowest bit
                help(next_full, curr ^ lowest_bit, result);
            } else {
                result.push(curr);
            }
        }
    
        let mut result = Vec::new();
        help(full, 0, &mut result);
        return result;
    }

    /// Generates a valid pext table for the square ```position```(\[1, 64\]). ```mask``` is the 
    /// pext mask for given position. ```move_gen``` is a function to give valid moves 
    /// for specific position and occupancy. First argument is position and bit encoded. 
    /// Second argument is occupancy where any bit from ```mask``` can be set.
    fn generate_pext_table(position: usize, mask: u64, keys: &mut Vec<u64>, move_gen: fn(u64, u64) -> u64) {
        let occupancies = BitboardRuntimeConstants::generate_all_occupancies(mask);
        for occupancy in occupancies { 
            let i: usize = pext_const(occupancy, mask) as usize;
            while i >= keys.len() {
                keys.push(0);
            }
            keys[i] = move_gen(1u64 << position, occupancy);
        }
    }

    fn generate_pext_bishop_table(position: usize, keys: &mut Vec<u64>) {
        BitboardRuntimeConstants::generate_pext_table(
            position, 
            BISHOP_MASKS[position], 
            keys, 
            generate_bishop_moves_slow
        );
    }

    fn generate_pext_rook_table(position: usize, keys: &mut Vec<u64>) {
        BitboardRuntimeConstants::generate_pext_table(
            position, 
            ROOK_MASKS[position], 
            keys, 
            generate_rook_moves_slow
        );
    }

    fn generate_magic_number<T: Rng>(variations: &Vec<u64>, expected: &Vec<u64>, optimal_size: u32, rng: &mut T) -> (u64, u8) {
        let mut best = 1;
        let mut val = 64;

        fn test_magic(magic: u64, target_bits: u32, variations: &Vec<u64>, expected: &Vec<u64>, observed: &mut Vec<u64>) -> bool {
            observed.resize(0, 0);
            observed.resize(1<<target_bits, 0);

            let mut success = true;
            for (variation, value) in zip(variations, expected) {
                let key = ((magic.wrapping_mul(*variation)) >> (64 - target_bits)) as usize;
                if observed[key] == 0 {
                    observed[key] = *value;
                } else if observed[key] != *value {
                    success = false;
                    break;
                }
            }
            success
        }

        let mut target_bits = optimal_size + MINIMUM_QUALITY; 
        let mut observed = Vec::new();
        while val == 64 {
            for _ in 0..NUMBER_OF_MAGIC_TABLE_TRIES {
                let magic = rng.next_u64() & rng.next_u64() & rng.next_u64();
                while target_bits >= optimal_size && test_magic(magic, target_bits, &variations, &expected, &mut observed) {
                    best = magic;
                    val = target_bits;
                    target_bits = target_bits - 1;
                }
                if val == optimal_size {
                    break;
                }
            }
        }
        (best, val as u8)
    }

    fn generate_magic_numbers(masks: &[u64; 64], move_gen: fn(u64, u64) -> u64) -> ([u8; 64], [u64; 64], [Vec<u64>; 64]) {
        let mut rng = rand_pcg::Pcg64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let mut num_bits = [64u8; 64];
        let mut magics = [1u64; 64];
        let mut lookup_tables : [Vec<u64>; 64] = core::array::from_fn(|_| Vec::new());
        for i in 0..64 {
            let mask = masks[i];
            let variations = BitboardRuntimeConstants::generate_all_occupancies(mask);
            let expected: Vec<u64> = variations.iter().map(
                |x| move_gen(1u64 << i, *x)).collect();
            let (magic, bits) = BitboardRuntimeConstants::generate_magic_number(&variations, &expected, mask.count_ones(), &mut rng);
            magics[i] = magic;
            num_bits[i] = bits;
            let keys: Vec<usize> = variations.iter().map(
                |x| (magic.wrapping_mul(*x) >> (64 - bits)) as usize
            ).collect();
            let max_key = *(keys.iter().max().unwrap());
            lookup_tables[i] = Vec::with_capacity(max_key + 1);
            for _ in 0..(max_key+1) {
                lookup_tables[i].push(0);
            }
            for (key, value) in zip(keys, expected) {
                assert!(lookup_tables[i][key] == 0 || lookup_tables[i][key] == value);
                lookup_tables[i][key] = value;
            }
        }
        return (num_bits, magics, lookup_tables);
    }

    fn generate_bishop_magic_numbers() -> ([u8; 64], [u64; 64], [Vec<u64>; 64]) {
        return BitboardRuntimeConstants::generate_magic_numbers(&BISHOP_MASKS, generate_bishop_moves_slow);
    }

    
    fn generate_rook_magic_numbers() -> ([u8; 64], [u64; 64], [Vec<u64>; 64]) {
        return BitboardRuntimeConstants::generate_magic_numbers(&ROOK_MASKS, generate_rook_moves_slow);
    }

    fn generate_bishop_tables() -> Vec<Vec<u64>> {
        let mut magic = Vec::with_capacity(64);
        for i in 0..64 {
            let mut keys = Vec::<u64>::with_capacity(1usize << BISHOP_MASKS[i].count_ones());
    
            BitboardRuntimeConstants::generate_pext_bishop_table(i, &mut keys);
            magic.push(keys);
        }
        magic
    }

    fn generate_rook_tables() -> Vec<Vec<u64>> {
        let mut magic = Vec::with_capacity(64);
        for i in 0..64 {
            let mut keys = Vec::<u64>::with_capacity(1usize << ROOK_MASKS[i].count_ones());
    
            BitboardRuntimeConstants::generate_pext_rook_table(i, &mut keys);
            magic.push(keys);
        }
        magic
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

    /// Access valid bishop moves (ignoring king safety and piece color) using 
    /// given occupancy. ```position``` is given as a index of square.
    pub fn bishop_magic(&self, position: usize, occupancy: u64) -> u64 {
        let mask = BISHOP_MASKS[position];
        #[cfg(target_feature = "bmi2")]
        unsafe {
            let key = _pext_u64(occupancy, mask);
            return self.bishop_magic_pext_table[position as usize][key as usize];
        }
        #[cfg(not(target_feature = "bmi2"))] 
        {
            let bits = self.bishop_magic_magic_table.0[position];
            let magic = self.bishop_magic_magic_table.1[position];
            let table = &self.bishop_magic_magic_table.2[position];
            let key = ((magic.wrapping_mul(occupancy & mask)) >> (64 - bits)) as usize;
            return table[key];
        }
    }

    /// Access valid rook moves (ignoring king safety and piece color) using 
    /// given occupancy. ```position``` is given as a index of square.
    pub fn rook_magic(&self, position: usize, occupancy: u64) -> u64 {
        let mask = ROOK_MASKS[position];
        #[cfg(target_feature = "bmi2")]
        unsafe {
            let key = _pext_u64(occupancy, mask);
            return self.rook_magic_pext_table[position as usize][key as usize];
        }
        #[cfg(not(target_feature = "bmi2"))] 
        {
            let bits = self.rook_magic_magic_table.0[position];
            let magic = self.rook_magic_magic_table.1[position];
            let table = &self.rook_magic_magic_table.2[position];
            let key = ((magic.wrapping_mul(occupancy & mask)) >> (64 - bits)) as usize;
            return table[key];
        }
    }
}
