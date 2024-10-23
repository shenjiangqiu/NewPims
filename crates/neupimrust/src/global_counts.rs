use derive_more::derive::AddAssign;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::File};
use tracing::{error, info};

#[derive(Serialize, Deserialize)]
pub enum MemStatus {
    Idle(u64),
    Busy(u64),
}
impl Default for MemStatus {
    fn default() -> Self {
        MemStatus::Idle(0)
    }
}

#[derive(Default, Serialize, PartialEq, Eq, PartialOrd, Ord, AddAssign, Deserialize)]
pub struct Cycle(u64);

/// record the current ongoing operations
#[derive(Default, Serialize, Deserialize)]
pub struct Counts {
    pub loads: u64,
    pub stores: u64,
    pub computes: u64,
}

#[derive(Default, Serialize, Deserialize)]
pub struct OpCycles {
    pub loads: Cycle,
    pub stores: Cycle,
    pub computes: Cycle,
    pub load_or_stores: Cycle,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CurrentStatus {
    pub loads: MemStatus,
    pub stores: MemStatus,
    pub computes: MemStatus,
    pub load_or_stores: MemStatus,
}

#[derive(Default, Serialize, Deserialize)]
pub struct HistoEntry {
    pub count: u64,
    pub stages: Vec<RunStage>,
}
#[derive(Default, Serialize, Deserialize)]
pub struct CycleHistogram {
    pub loads: BTreeMap<Cycle, HistoEntry>,
    pub stores: BTreeMap<Cycle, HistoEntry>,
    pub computes: BTreeMap<Cycle, HistoEntry>,
    pub load_or_stores: BTreeMap<Cycle, HistoEntry>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct GlobalCountsCtx {
    // the statistics
    /// the last cycle
    pub last_cycle: u64,

    /// 总的busy 的时间
    pub busy_cycles: OpCycles,
    /// 持续idle时间
    pub idle_cycles: OpCycles,

    /// idle 的时间间隔统计
    pub idle_histo: CycleHistogram,
    /// busy 的时间间隔统计
    pub busy_histo: CycleHistogram,

    /// 累计的操作次数
    pub all_counts: Counts,

    // the runtime info
    /// current load store and computes operations
    pub current_counts: Counts,
    pub current_stage: RunStage,
    // 记录上一次加载、存储、计算操作的开始时间
    pub current_status: CurrentStatus,

    pub event_vec: Vec<Event>,
}

#[no_mangle]
pub extern "C" fn update_last_cycle(ctx: &mut GlobalCountsCtx, cycle: u64) {
    ctx.last_cycle = cycle;
}

/// 创建一个新的`GlobalCountsCtx`。
#[no_mangle]
pub extern "C" fn new_global_counts_ctx() -> *mut GlobalCountsCtx {
    info!("创建新的GlobalCountsCtx");
    Box::into_raw(Box::new(GlobalCountsCtx::default()))
}

/// 释放`GlobalCountsCtx`。
#[no_mangle]
pub extern "C" fn drop_global_counts_ctx(ctx: *mut GlobalCountsCtx) {
    info!("释放GlobalCountsCtx");
    if ctx.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ctx));
    }
}

/// Run stages
#[repr(C)]
#[derive(Debug, Serialize, Default, Clone, Copy, Deserialize)]
pub enum RunStage {
    #[default]
    A,
    B,
    C,
    D,
    E,
    F,
    Finished,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Event {
    pub cycle: u64,
    pub stage: RunStage,
    pub event: EventType,
}
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub enum MemOp {
    Load,
    Store,
    Compute,
    LoadOrStore,
}

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub enum EventType {
    MemEventStart(MemOp),
    MemEventEnd(MemOp),
    StageStart,
    StageEnd,
    NpuStart,
    PimStart,
    NpuFinished,
    PimFinished,
}

#[no_mangle]
pub extern "C" fn update_stage(ctx: &mut GlobalCountsCtx, stage: RunStage, cycle: u64) {
    ctx.event_vec.push(Event {
        cycle,
        stage,
        event: EventType::StageStart,
    });
    ctx.current_stage = stage;
}

#[no_mangle]
pub extern "C" fn end_stage(ctx: &mut GlobalCountsCtx, stage: RunStage, cycle: u64) {
    ctx.event_vec.push(Event {
        cycle,
        stage,
        event: EventType::StageEnd,
    });
}

#[no_mangle]
pub extern "C" fn npu_finished(ctx: &mut GlobalCountsCtx, cycle: u64) {
    ctx.event_vec.push(Event {
        cycle,
        stage: ctx.current_stage,
        event: EventType::NpuFinished,
    });
}

#[no_mangle]
pub extern "C" fn pim_finished(ctx: &mut GlobalCountsCtx, cycle: u64) {
    ctx.event_vec.push(Event {
        cycle,
        stage: ctx.current_stage,
        event: EventType::PimFinished,
    });
}

/// 保存累计的数据到文件
#[no_mangle]
#[allow(static_mut_refs)]
pub extern "C" fn save_global_counts_to_file(ctx: &GlobalCountsCtx) {
    let file = File::create("counts.json").expect("无法创建文件");
    serde_json::to_writer_pretty(file, &ctx).expect("无法写入文件");
}
/// 增加加载操作的计数
///
/// # 参数
///
/// * `loads` - 要增加的加载操作数量
#[no_mangle]
#[allow(static_mut_refs)]
pub extern "C" fn add_loads(ctx: &mut GlobalCountsCtx, loads: u64, cycle: u64) {
    ctx.current_counts.loads += loads;
    ctx.all_counts.loads += loads;

    if ctx.current_counts.loads == loads {
        match ctx.current_status.loads {
            MemStatus::Idle(start_cycle) => {
                let idle_duration = cycle - start_cycle;
                if idle_duration != 0 {
                    ctx.idle_histo
                        .loads
                        .entry(Cycle(idle_duration))
                        .and_modify(|entry| {
                            entry.count += 1;
                            entry.stages.push(ctx.current_stage);
                        })
                        .or_insert(HistoEntry {
                            count: 1,
                            stages: vec![ctx.current_stage],
                        });

                    ctx.idle_cycles.loads += Cycle(idle_duration);
                }
                ctx.current_status.loads = MemStatus::Busy(cycle);

                ctx.event_vec.push(Event {
                    cycle,
                    stage: ctx.current_stage,
                    event: EventType::MemEventStart(MemOp::Load),
                });

                match ctx.current_status.load_or_stores {
                    MemStatus::Idle(start_cycle) => {
                        let idle_duration = cycle - start_cycle;
                        if idle_duration != 0 {
                            ctx.idle_histo
                                .load_or_stores
                                .entry(Cycle(idle_duration))
                                .and_modify(|entry| {
                                    entry.count += 1;
                                    entry.stages.push(ctx.current_stage);
                                })
                                .or_insert(HistoEntry {
                                    count: 1,
                                    stages: vec![ctx.current_stage],
                                });
                            ctx.idle_cycles.load_or_stores += Cycle(idle_duration);
                        }
                        ctx.current_status.load_or_stores = MemStatus::Busy(cycle);
                        ctx.event_vec.push(Event {
                            cycle,
                            stage: ctx.current_stage,
                            event: EventType::MemEventStart(MemOp::LoadOrStore),
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    // 添加loads可能会从idle变成busy
}

/// 增加存储操作的计数
///
/// # 参数
///
/// * `stores` - 要增加的存储操作数量
#[no_mangle]
pub extern "C" fn add_stores(ctx: &mut GlobalCountsCtx, stores: u64, cycle: u64) {
    ctx.current_counts.stores += stores;
    ctx.all_counts.stores += stores;

    if ctx.current_counts.stores == stores {
        match ctx.current_status.stores {
            MemStatus::Idle(start_cycle) => {
                let idle_duration = cycle - start_cycle;
                if idle_duration != 0 {
                    ctx.idle_histo
                        .stores
                        .entry(Cycle(idle_duration))
                        .and_modify(|entry| {
                            entry.count += 1;
                            entry.stages.push(ctx.current_stage);
                        })
                        .or_insert(HistoEntry {
                            count: 1,
                            stages: vec![ctx.current_stage],
                        });
                    ctx.idle_cycles.stores += Cycle(idle_duration);
                }
                ctx.current_status.stores = MemStatus::Busy(cycle);
                ctx.event_vec.push(Event {
                    cycle,
                    stage: ctx.current_stage,
                    event: EventType::MemEventStart(MemOp::Store),
                });

                match ctx.current_status.load_or_stores {
                    MemStatus::Idle(start_cycle) => {
                        let idle_duration = cycle - start_cycle;
                        if idle_duration != 0 {
                            ctx.idle_histo
                                .load_or_stores
                                .entry(Cycle(idle_duration))
                                .and_modify(|entry| {
                                    entry.count += 1;
                                    entry.stages.push(ctx.current_stage);
                                })
                                .or_insert(HistoEntry {
                                    count: 1,
                                    stages: vec![ctx.current_stage],
                                });
                            ctx.idle_cycles.load_or_stores += Cycle(idle_duration);
                        }
                        ctx.current_status.load_or_stores = MemStatus::Busy(cycle);
                        ctx.event_vec.push(Event {
                            cycle,
                            stage: ctx.current_stage,
                            event: EventType::MemEventStart(MemOp::LoadOrStore),
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

/// 增加计算操作的计数
///
/// # 参数
///
/// * `computes` - 要增加的计算操作数量
#[no_mangle]
pub extern "C" fn add_computes(ctx: &mut GlobalCountsCtx, computes: u64) {
    ctx.current_counts.computes += computes;
    ctx.all_counts.computes += computes;
}

/// 获取当前的加载操作计数
///
/// # 返回值
///
/// 返回当前的加载操作总数
#[no_mangle]
pub extern "C" fn get_loads(ctx: &GlobalCountsCtx) -> u64 {
    ctx.current_counts.loads
}

/// 获取当前的存储操作计数
///
/// # 返回值
///
/// 返回当前的存储操作总数
#[no_mangle]
pub extern "C" fn get_stores(ctx: &GlobalCountsCtx) -> u64 {
    ctx.current_counts.stores
}

/// 获取当前的计算操作计数
///
/// # 返回值
///
/// 返回当前的计算操作总数
#[no_mangle]
pub extern "C" fn get_computes(ctx: &GlobalCountsCtx) -> u64 {
    ctx.current_counts.computes
}

/// 减少加载操作的计数
///
/// # 参数
///
/// * `loads` - 要减少的加载操作数量
///
/// # 返回值
///
/// 如果减少操作成功，返回`true`；如果减少操作会导致计数变为负值，返回`false`
#[no_mangle]
#[allow(static_mut_refs)]
pub extern "C" fn reduce_loads(ctx: &mut GlobalCountsCtx, loads: u64, cycle: u64) -> bool {
    if ctx.current_counts.loads < loads {
        error!("错误：尝试将GLOBAL_LOADS减少到负值");
        return false;
    }
    ctx.current_counts.loads -= loads;
    // check triggers
    if ctx.current_counts.loads == 0 {
        match ctx.current_status.loads {
            MemStatus::Busy(start_cycle) => {
                let busy_duration = cycle - start_cycle;
                ctx.busy_histo
                    .loads
                    .entry(Cycle(busy_duration))
                    .and_modify(|entry| {
                        entry.count += 1;
                        entry.stages.push(ctx.current_stage);
                    })
                    .or_insert(HistoEntry {
                        count: 1,
                        stages: vec![ctx.current_stage],
                    });
                ctx.busy_cycles.loads += Cycle(busy_duration);
                ctx.current_status.loads = MemStatus::Idle(cycle);
                ctx.event_vec.push(Event {
                    cycle: cycle,
                    stage: ctx.current_stage,
                    event: EventType::MemEventEnd(MemOp::Load),
                });

                match (
                    &ctx.current_status.load_or_stores,
                    &ctx.current_status.stores,
                ) {
                    (MemStatus::Busy(start_cycle), MemStatus::Idle(_)) => {
                        let busy_duration = cycle - start_cycle;
                        ctx.busy_histo
                            .load_or_stores
                            .entry(Cycle(busy_duration))
                            .and_modify(|entry| {
                                entry.count += 1;
                                entry.stages.push(ctx.current_stage);
                            })
                            .or_insert(HistoEntry {
                                count: 1,
                                stages: vec![ctx.current_stage],
                            });
                        ctx.busy_cycles.load_or_stores += Cycle(busy_duration);
                        ctx.current_status.load_or_stores = MemStatus::Idle(cycle);
                        ctx.event_vec.push(Event {
                            cycle: cycle,
                            stage: ctx.current_stage,
                            event: EventType::MemEventEnd(MemOp::LoadOrStore),
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    true
}

/// 减少存储操作的计数
///
/// # 参数
///
/// * `stores` - 要减少的存储操作数量
///
/// # 返回值
///
/// 如果减少操作成功，返回`true`；如果减少操作会导致计数变为负值，返回`false`
#[no_mangle]
pub extern "C" fn reduce_stores(ctx: &mut GlobalCountsCtx, stores: u64, cycle: u64) -> bool {
    if ctx.current_counts.stores < stores {
        error!("错误：尝试将GLOBAL_STORES减少到负值");
        return false;
    }
    ctx.current_counts.stores -= stores;

    if ctx.current_counts.stores == 0 {
        match ctx.current_status.stores {
            MemStatus::Busy(start_cycle) => {
                let busy_duration = cycle - start_cycle;
                ctx.busy_histo
                    .stores
                    .entry(Cycle(busy_duration))
                    .and_modify(|entry| {
                        entry.count += 1;
                        entry.stages.push(ctx.current_stage);
                    })
                    .or_insert(HistoEntry {
                        count: 1,
                        stages: vec![ctx.current_stage],
                    });
                ctx.busy_cycles.stores += Cycle(busy_duration);
                ctx.current_status.stores = MemStatus::Idle(cycle);
                ctx.event_vec.push(Event {
                    cycle: cycle,
                    stage: ctx.current_stage,
                    event: EventType::MemEventEnd(MemOp::Store),
                });

                match (
                    &ctx.current_status.load_or_stores,
                    &ctx.current_status.loads,
                ) {
                    (MemStatus::Busy(start_cycle), MemStatus::Idle(_)) => {
                        let busy_duration = cycle - start_cycle;
                        ctx.busy_histo
                            .load_or_stores
                            .entry(Cycle(busy_duration))
                            .and_modify(|entry| {
                                entry.count += 1;
                                entry.stages.push(ctx.current_stage);
                            })
                            .or_insert(HistoEntry {
                                count: 1,
                                stages: vec![ctx.current_stage],
                            });
                        ctx.busy_cycles.load_or_stores += Cycle(busy_duration);
                        ctx.current_status.load_or_stores = MemStatus::Idle(cycle);
                        ctx.event_vec.push(Event {
                            cycle: cycle,
                            stage: ctx.current_stage,
                            event: EventType::MemEventEnd(MemOp::LoadOrStore),
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    true
}

/// 减少计算操作的计数
///
/// # 参数
///
/// * `computes` - 要减少的计算操作数量
///
/// # 返回值
///
/// 如果减少操作成功，返回`true`；如果减少操作会导致计数变为负值，返回`false`
#[no_mangle]
pub extern "C" fn reduce_computes(ctx: &mut GlobalCountsCtx, computes: u64) -> bool {
    if ctx.current_counts.computes < computes {
        error!("错误：尝试将GLOBAL_COMPUTES减少到负值");
        return false;
    }
    ctx.current_counts.computes -= computes;
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_save() {
        // let file = File::create("counts_test.json").expect("无法创建文件");
        // let mut global_count = GlobalCountsCtx::default();
        // update_global_on_cycle(&mut global_count, 1);
        // add_loads(&mut global_count, 1);
        // add_stores(&mut global_count, 1);
        // update_global_on_cycle(&mut global_count, 2);
        // update_global_on_cycle(&mut global_count, 3);
        // reduce_loads(&mut global_count, 1);
        // update_global_on_cycle(&mut global_count, 4);
        // reduce_stores(&mut global_count, 1);
        // update_global_on_cycle(&mut global_count, 5);
        // update_global_on_cycle(&mut global_count, 6);
        // serde_json::to_writer_pretty(file, &global_count).expect("无法写入文件");
    }
}
