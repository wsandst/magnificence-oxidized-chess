// Copyright 2022 Latiang.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(test)]
mod tests;

struct Board {
    piece_sets : [u64; 12],
    occupancy : u64,
    white_occupancy : u64,
    black_occupancy : u64,
    ep_history : Vec<u8>,
    castling_history : Vec<u8>,
    ep : u8,
    castling : u8
}