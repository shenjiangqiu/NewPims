use crate::global_config::get_config;

pub struct ActivationAllocator {
    pub base_addr: usize,
    pub top_addr: usize,
    pub buf_size: usize,
    pub buf_limit: usize,
}

impl ActivationAllocator {
    pub fn new(base_addr: usize) -> Self {
        ActivationAllocator {
            base_addr,
            top_addr: base_addr,
            buf_size: get_config().hbm_act_buf_size as usize,
            buf_limit: base_addr + get_config().hbm_act_buf_size as usize,
        }
    }
    pub fn allocate(&mut self, size: usize) -> usize {
        assert!(self.top_addr + size < self.buf_limit);
        let alignment = get_config().dram_req_size as usize;

        let result = self.top_addr;
        self.top_addr += size;
        if self.top_addr & (alignment - 1) != 0 {
            self.top_addr += alignment - (self.top_addr & (alignment - 1));
        }
        return result;
    }
    pub fn reset(&mut self) {
        self.top_addr = self.base_addr;
    }

    pub fn get_next_aligned_addr(&self) -> usize {
        todo!()
    }
    pub fn get_static() -> &'static ActivationAllocator {
        todo!()
    }
}
