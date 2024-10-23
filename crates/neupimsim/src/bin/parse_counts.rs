use clap::Parser;
use neupimrust::global_counts::*;
use std::{fs::File, path::PathBuf};

#[derive(clap::ValueEnum, Clone)]
enum App {
    ShowStage,
    Loads,
    Stores,
}

#[derive(Parser)]
struct Cli {
    file_path: PathBuf,
    #[arg(short, long, default_value = "show-stage")]
    app: App,
}
fn main() {
    let cli = Cli::parse();
    let counts: GlobalCountsCtx =
        serde_json::from_reader(File::open(cli.file_path).unwrap()).unwrap();
    match cli.app {
        App::Loads => {
            loads(&counts);
        }
        App::Stores => {
            stores(&counts);
        }
        App::ShowStage => {
            show_stage(&counts);
        }
    }
}
fn show_stage(counts: &GlobalCountsCtx) {
    let event_vec = &counts.event_vec;
    let all_stage_event = event_vec.iter().filter(|e| {
        matches!(
            e.event,
            EventType::StageStart
                | EventType::StageEnd
                | EventType::NpuStart
                | EventType::PimStart
                | EventType::NpuFinished
                | EventType::PimFinished
        )
    });
    for e in all_stage_event {
        println!(
            "event: {:20?}:{:?} at cycle: {:8?}",
            e.event, e.stage, e.cycle
        );
    }
}

fn loads(counts: &GlobalCountsCtx) {
    let loads = counts.event_vec.iter().filter(|e| {
        matches!(
            e.event,
            EventType::MemEventStart(MemOp::Load) | EventType::MemEventEnd(MemOp::Load)
        )
    });
    for e in loads {
        println!(
            "{:6}: {:20} {:?} at cycle: {:8}",
            "load",
            format!("{:?}", e.event),
            e.stage,
            e.cycle
        );
    }
}

fn stores(counts: &GlobalCountsCtx) {
    let stores = counts.event_vec.iter().filter(|e| {
        matches!(
            e.event,
            EventType::MemEventStart(MemOp::Store) | EventType::MemEventEnd(MemOp::Store)
        )
    });
    for e in stores {
        println!(
            "{:6}: {:20} {:?} at cycle: {:8}",
            "store",
            format!("{:?}", e.event),
            e.stage,
            e.cycle
        );
    }
}
