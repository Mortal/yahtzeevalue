extern crate byteorder;
extern crate memmap;

use std::fs;
use byteorder::{ByteOrder, LittleEndian};
use crate::Result;

pub struct Store {
    mmap: memmap::Mmap,
}

impl Store {
    pub fn new(path: &str) -> Result<Store> {
        let file = fs::File::open(path)?;
        let mmap = unsafe { memmap::Mmap::map(&file)? };
        Ok(Store {
            mmap: mmap,
        })
    }

    pub fn len(&self) -> u32 {
        let l = self.mmap.len() / 8;
        assert!(l <= std::u32::MAX as usize);
        l as u32
    }

    pub fn get(&self, s: u32) -> f64 {
        assert!(s < self.len());
        let i = 8 * s as usize;
        LittleEndian::read_f64(&self.mmap[i..i+8])
    }
}
