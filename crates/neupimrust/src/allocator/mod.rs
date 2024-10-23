use crate::global_config::get_config;

pub mod act_allocator;
pub mod kv_allocator;
pub mod weight_allocator;
pub fn get_aligned_addr(addr: usize) -> usize {
    let config = get_config();
    let unit = config.dram_req_size;
    addr - (addr & (unit as usize - 1))
}
