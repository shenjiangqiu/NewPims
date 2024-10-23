fn main() {
    //--config ./configs/systolic_ws_128x128_dev.json --mem_config ./configs/memory_configs/neupims.json --model_config ./configs/model_configs/gpt3-7B.json --sys_config ./configs/system_configs/sub-batch-on.json --cli_config ./request-traces/clb/share-gpt2-bs512-ms7B-tp4-clb-0.csv --log_dir experiment_logs/test
    let args = vec![
        "test",
        "--config",
        "./configs/systolic_ws_128x128_dev.json",
        "--mem_config",
        "./configs/memory_configs/neupims.json",
        "--model_config",
        "./configs/model_configs/gpt3-7B.json",
        "--sys_config",
        "./configs/system_configs/sub-batch-on.json",
        "--cli_config",
        "./request-traces/clb/share-gpt2-bs512-ms7B-tp4-clb-0.csv",
        "--log_dir",
        "./experiment_logs/test",
    ];
    new_pims::cpp_main_with_args(args);
}
