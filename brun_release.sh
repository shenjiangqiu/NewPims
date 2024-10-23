#!/bin/bash
cd build_release; 
make -j; 
# if error , return error message
if [ $? -ne 0 ]; then
    echo "Error: make failed"
    exit 1
fi
cd ..;

# config file
config=./configs/systolic_ws_128x128_dev.json
mem_config=./configs/memory_configs/neupims.json
model_config=./configs/model_configs/gpt3-7B.json
sys_config=./configs/system_configs/sub-batch-on.json
cli_config=./request-traces/clb/share-gpt2-bs512-ms7B-tp4-clb-0.csv

# log file
LOG_LEVEL=info
DATE=$(date "+%F_%H:%M:%S")

LOG_DIR=experiment_logs/${DATE}

mkdir -p $LOG_DIR;
LOG_NAME=simulator.log
CONFIG_FILE=${LOG_DIR}/config.log

echo "log directory: $LOG_DIR"



./build_release/src/Simulator \
    --config $config \
    --mem_config $mem_config \
    --cli_config $cli_config \
    --model_config $model_config \
    --sys_config $sys_config \
    --log_dir $LOG_DIR


echo "memory config: $mem_config" > ${CONFIG_FILE}
echo "client config: $cli_config" >> ${CONFIG_FILE}
echo "model config: $model_config" >> ${CONFIG_FILE}
echo "system config: $sys_config" >> ${CONFIG_FILE}
cat ${CONFIG_FILE}
# create a link from $LOG_DIR/_summary.tsv to latest.tsv
ln -sf $LOG_DIR/_summary.tsv latest.tsv
