use std::sync::{Mutex, OnceLock};

use crate::global_config::get_config;
pub static ALLOCATOR: OnceLock<Mutex<WeightAllocator>> = OnceLock::new();
pub struct WeightAllocator {
    pub base_addr: usize,
    pub top_addr: usize,
}

impl WeightAllocator {
    pub fn new(base_addr: usize, top_addr: usize) -> Self {
        WeightAllocator {
            base_addr,
            top_addr,
        }
    }
    pub fn allocate(&mut self, size: usize) -> usize {
        let unit = get_config().dram_req_size * get_config().dram_channels;
        let unit = unit as usize;
        let result = self.top_addr;
        self.top_addr += (size + unit - 1) / unit;
        return result;
    }
    pub fn reset(&mut self) {
        self.top_addr = self.base_addr;
    }

    pub fn get_next_addr(&self) -> usize {
        super::get_aligned_addr(self.top_addr) + get_config().dram_req_size as usize
    }
}
