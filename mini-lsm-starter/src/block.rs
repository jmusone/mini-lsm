// Copyright (c) 2022-2025 Alex Chi Z
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::{Buf, BufMut, Bytes};
pub use iterator::BlockIterator;

pub(crate) const U16_SIZE: usize = std::mem::size_of::<u16>();
/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted key-value pairs.
pub struct Block {
    pub(crate) data: Vec<u8>,
    pub(crate) offsets: Vec<u16>,
}

impl Block {
    /// Encode the internal data to the data layout illustrated in the course
    /// Note: You may want to recheck if any of the expected field is missing from your output
    pub fn encode(&self) -> Bytes {
        let mut block = self.data.clone();
        for offset in &self.offsets {
            block.put_u16(*offset);
        }
        block.put_u16(self.offsets.len() as u16);
        block.into()
    }

    /// Decode from the data layout, transform the input `data` to a single `Block`
    pub fn decode(data: &[u8]) -> Self {
        /*
            1. Take the u16 at the end to get the count
            2. Using that count, iterate through the offsets preceding it
            3. The rest is data
        */

        let count = (&data[(data.len() - U16_SIZE)..]).get_u16() as usize;
        let data_end = data.len() - U16_SIZE - (count * U16_SIZE);

        let offsets = data[data_end..(data.len() - U16_SIZE)]
            .chunks(U16_SIZE)
            .map(|mut x| x.get_u16())
            .collect();
        let data = data[0..data_end].to_vec();

        Self { data, offsets }
    }
}
