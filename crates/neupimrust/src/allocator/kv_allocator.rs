use crate::global_config::get_config;

pub struct KVCacheAllocator {
    pub base_addr: usize,
    pub top_addr: usize,
}

impl KVCacheAllocator {
    pub fn new(base_addr: usize, top_addr: usize) -> Self {
        KVCacheAllocator {
            base_addr,
            top_addr,
        }
    }
    pub fn allocate(&mut self, size: usize) -> usize {
        let unit = get_config().dram_req_size;
        let unit = unit as usize;
        let result = self.top_addr;
        self.top_addr += (size + unit - 1) / unit;
        return result;
    }
    pub fn reset(&mut self) {
        self.top_addr = self.base_addr;
    }
    pub fn get_static() -> &'static KVCacheAllocator {
        todo!()
    }
}
