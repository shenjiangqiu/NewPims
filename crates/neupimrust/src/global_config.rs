
pub fn get_config() -> &'static SimulationConfig {
    lazy_static::lazy_static! {
        static ref CONFIG: SimulationConfig = {
            let config_str = std::fs::read_to_string("sjq_config.toml").expect("Unable to read file");
            toml::from_str(&config_str).expect("Failed to parse TOML")
        };
    }
    &CONFIG
}

#[derive(Debug, serde::Deserialize)]
pub struct SimulationConfig {
    // gpt model config
    pub model_name: String,
    pub model_params_b: u32,
    pub model_block_size: u32,
    pub model_vocab_size: u32,
    pub model_n_layer: u32,
    pub model_n_head: u32,
    pub model_n_embd: u32,

    /* Custom Config */
    pub run_mode: RunMode, // NPU
    pub sub_batch_mode: bool,
    pub ch_load_balancing: bool,
    pub kernel_fusion: bool,
    pub max_batch_size: u32,
    pub max_active_reqs: u32, // max size of (ready_queue + running_queue) in scheduler
    pub max_seq_len: u32,
    pub hbm_size: u64,         // HBM size in bytes
    pub hbm_act_buf_size: u64, // HBM activation buffer size in bytes

    /* Core config */
    pub num_cores: u32,
    pub core_type: CoreType,
    pub core_freq: u32,
    pub core_width: u32,
    pub core_height: u32,

    pub n_tp: u32,

    pub vector_core_count: u32,
    pub vector_core_width: u32,

    /* Vector config*/
    pub process_bit: u32,

    pub layernorm_latency: usize,
    pub softmax_latency: usize,
    pub add_latency: usize,
    pub mul_latency: usize,
    pub exp_latency: usize,
    pub gelu_latency: usize,
    pub add_tree_latency: usize,
    pub scalar_sqrt_latency: usize,
    pub scalar_add_latency: usize,
    pub scalar_mul_latency: usize,

    /* SRAM config */
    pub sram_width: u32,
    pub sram_size: u32,
    pub spad_size: u32,
    pub accum_spad_size: u32,

    /* DRAM config */
    pub dram_type: DramType,
    pub dram_freq: u32,
    pub dram_channels: u32,
    pub dram_req_size: u32,

    /* PIM config */
    pub pim_config_path: String,
    pub dram_page_size: u32, // DRAM row buffer size (in bytes)
    pub dram_banks_per_ch: u32,
    pub pim_comp_coverage: u32, // # params per PIM_COMP command

    /* Log config */
    pub operation_log_output_path: String,
    pub log_dir: String,

    /* Client config */
    pub request_input_seq_len: u32,
    pub request_interval: u32,
    pub request_total_cnt: u32,
    pub request_dataset_path: String,

    /* ICNT config */
    pub icnt_type: IcntType,
    pub icnt_config_path: String,
    pub icnt_freq: u32,
    pub icnt_latency: u32,

    /* Scheduler config */
    pub scheduler_type: String,

    /* Other configs */
    pub precision: u32,
    pub layout: String,
}

impl SimulationConfig {
    pub fn align_address(&self, addr: u64) -> u64 {
        addr - (addr % self.dram_req_size as u64)
    }
}

#[derive(Debug, serde::Deserialize)]
pub enum CoreType {
    SystolicOs,
    SystolicWs,
}

#[derive(Debug, serde::Deserialize)]
pub enum DramType {
    Dram,
    Newton,
    Neupims,
}

#[derive(Debug, serde::Deserialize)]
pub enum IcntType {
    Simple,
    Booksim2,
}

#[derive(Debug, serde::Deserialize)]
pub enum RunMode {
    NpuOnly,
    NpuPim,
}
