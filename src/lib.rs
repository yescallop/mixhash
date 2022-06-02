//! A non-cryptographic hash function that mixes the data only.
//!
//! Reference: <http://zimbry.blogspot.com/2011/09/better-bit-mixing-improving-on.html>
//!
//! # Examples
//!
//! ```
//! use mixhash::Mix;
//! use std::collections::HashSet;
//!
//! let set: HashSet<u32, _> = HashSet::with_hasher(Mix);
//! ```

#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

use core::hash::{BuildHasher, Hasher};

/// Builds a [`Mixer`].
#[derive(Clone, Copy, Debug)]
pub struct Mix;

impl BuildHasher for Mix {
    type Hasher = Mixer;

    #[inline]
    fn build_hasher(&self) -> Mixer {
        Mixer(0)
    }
}

/// A hasher that mixes the data only.
#[derive(Clone, Copy, Debug)]
pub struct Mixer(u64);

impl Hasher for Mixer {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut chunks = bytes.chunks_exact(8);
        for chunk in &mut chunks {
            let i = u64::from_ne_bytes(chunk.try_into().unwrap());
            self.write_u64(i);
        }

        let mut i = 0;
        for &byte in chunks.remainder() {
            i = (i << 8) | byte as u64;
        }
        self.write_u64(i);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, mut i: u64) {
        i ^= self.0;
        i = (i ^ (i >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        i = (i ^ (i >> 27)).wrapping_mul(0x94d049bb133111eb);
        self.0 = i ^ (i >> 31);
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write_u64(i as u64);
        self.write_u64((i >> 64) as u64);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        if usize::BITS == u128::BITS {
            self.write_u128(i as u128);
        } else {
            self.write_u64(i as u64);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}
