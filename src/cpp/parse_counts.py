# read from counts.json
# %%
import json

with open("counts.json", "r") as f:
    counts = json.load(f)
pim_finished = [
    event["cycle"] for event in counts["event_vec"] if event["event"] == "PimFinished"
]
npu_finished = [
    event["cycle"] for event in counts["event_vec"] if event["event"] == "NpuFinished"
]

print(counts)
# %%
# 使用matplotlib画图,途中有3个平行的线,第一个线标出stage_start,stage_end的时间节点

import matplotlib.pyplot as plt

# 提取时间节点
stage_starts = [
    event["cycle"] for event in counts["event_vec"] if event["event"] == "StageStart"
]
stage_ends = [
    event["cycle"] for event in counts["event_vec"] if event["event"] == "StageEnd"
]

# 提取时间节点
mem_event_starts = [
    event
    for event in counts["event_vec"]
    if isinstance(event["event"], dict) and "MemEventStart" in event["event"]
]
mem_event_ends = [
    event
    for event in counts["event_vec"]
    if isinstance(event["event"], dict) and "MemEventEnd" in event["event"]
]
mem_event_starts_loads = [
    event["cycle"]
    for event in mem_event_starts
    if event["event"]["MemEventStart"] == "Load"
]
mem_event_starts_stores = [
    event["cycle"]
    for event in mem_event_starts
    if event["event"]["MemEventStart"] == "Store"
]
mem_event_start_load_or_store = [
    event["cycle"]
    for event in mem_event_starts
    if event["event"]["MemEventStart"] == "LoadOrStore"
]
mem_event_ends_loads = [
    event["cycle"]
    for event in mem_event_ends
    if event["event"]["MemEventEnd"] == "Load"
]
mem_event_ends_stores = [
    event["cycle"]
    for event in mem_event_ends
    if event["event"]["MemEventEnd"] == "Store"
]

# 创建图形
plt.figure(figsize=(12, 8))  # Increase the figure size

# 绘制StageStart的时间节点
plt.plot(stage_starts, [1] * len(stage_starts), "ro", label="StageStart")

# 绘制StageEnd的时间节点
plt.plot(stage_ends, [2] * len(stage_ends), "bo", label="StageEnd")
plt.plot(pim_finished, [3] * len(pim_finished), "go", label="PimFinished")
plt.plot(npu_finished, [4] * len(npu_finished), "yo", label="NpuFinished")
# 绘制MemEventStart Load的时间节点
plt.plot(
    mem_event_starts_loads,
    [5] * len(mem_event_starts_loads),
    "go",
    markersize=5,
    label="MemEventStart Load",
)
# 绘制MemEventEnd Load的时间节点
plt.plot(
    mem_event_ends_loads,
    [6] * len(mem_event_ends_loads),
    "mo",
    markersize=5,
    label="MemEventEnd Load",
)
# 绘制MemEventStart Store的时间节点
plt.plot(
    mem_event_starts_stores,
    [7] * len(mem_event_starts_stores),
    "yo",
    markersize=5,
    label="MemEventStart Store",
)
# 绘制MemEventEnd Store的时间节点
plt.plot(
    mem_event_ends_stores,
    [8] * len(mem_event_ends_stores),
    "co",
    markersize=5,
    label="MemEventEnd Store",
)

# 添加标签和标题
plt.xlabel("Cycle")
plt.ylabel("Event")
plt.title("Stage, MemEvent, PimFinished, and NpuFinished Cycles")

# Place the legend outside the main figure
plt.legend(loc='upper left', bbox_to_anchor=(1, 1))

# 显示图形
plt.show()

# %%
