use clap::Parser;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
pub mod allocator;
pub mod global_config;
pub mod global_counts;
pub mod instruction;
pub mod no_icnt;
pub mod settings;
pub mod tensor;
#[repr(C)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

#[derive(Parser)]
struct Cli {
    sjqconfig: String,
    config: String,
    mem_config: String,
    cli_config: String,
    model_config: String,
}
pub fn run() {}
/// 初始化日志记录器
///
/// # 参数
///
/// * `level` - 日志级别
#[no_mangle]
pub extern "C" fn init_logger(level: LogLevel) {
    let level = LevelFilter::from(level);

    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy(),
        )
        .try_init()
        .unwrap_or_else(|err| {
            eprintln!("Failed to init logger: {}", err);
        });
    info!("Logger initialized");
}

#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn main() {
        println!("Hello, world!");
        let _bx = Box::new("asdfadsf");
        println!("a:{}", mem::size_of::<&str>());
        println!("b:{}", mem::size_of::<&i32>());
        let _c = [1, 2, 3];
        println!("c:{}", mem::size_of::<Box<str>>());
        println!("d:{}", mem::size_of::<Box<i32>>());
        println!("e:{}", mem::size_of::<Box<[i32]>>());
        println!("f:{}", mem::size_of::<Box<[i32; 3]>>());
        println!("g:{}", mem::size_of::<[i32; 3]>());
    }
}
