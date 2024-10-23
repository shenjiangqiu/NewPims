use neupimrust::{init_logger, LogLevel};
fn main() {
    init_logger(LogLevel::Info);
    neupimrust::run();
}

#[cfg(test)]
mod tests {
    use tracing::debug;
    #[test]
    fn test_init_logger() {
        super::init_logger(super::LogLevel::Debug);
        debug!("This is a debug message");
    }

    #[test]
    fn test_string() {
        let a = "-L/home/sjq/.conan2/p/b/boost1180d7bad9b4b/p/lib \
-L/home/sjq/.conan2/p/b/bzip2838d787afafd8/p/lib \
-L/home/sjq/.conan2/p/b/zlib2a7e3b0641960/p/lib \
-L/home/sjq/.conan2/p/b/libbad1e079e9b66c0/p/lib \
-L/home/sjq/.conan2/p/b/spdlo8b7eb6342a16d/p/lib \
-L/home/sjq/.conan2/p/b/fmtf09e886be2602/p/lib \
-Wl,-rpath,/home/sjq/.conan2/p/b/boost1180d7bad9b4b/p/lib:/home/sjq/.conan2/p/b/bzip2838d787afafd8/p/lib:/home/sjq/.conan2/p/b/zlib2a7e3b0641960/p/lib:/home/sjq/.conan2/p/b/libbad1e079e9b66c0/p/lib:/home/sjq/.conan2/p/b/spdlo8b7eb6342a16d/p/lib:/home/sjq/.conan2/p/b/fmtf09e886be2602/p/lib \
-lSimulator_lib \
-lboost_log_setup \
-lboost_unit_test_framework \
-lboost_type_erasure \
-lboost_log \
-lboost_locale \
-lboost_fiber_numa \
-lboost_contract \
-lboost_wave \
-lboost_thread \
-lboost_test_exec_monitor \
-lboost_process \
-lboost_prg_exec_monitor \
-lboost_nowide \
-lboost_iostreams \
-lbz2 \
-lz \
-lboost_graph \
-lboost_fiber \
-lboost_wserialization \
-lboost_url \
-lboost_stacktrace_noop \
-lboost_stacktrace_from_exception \
-lboost_stacktrace_basic \
-lboost_stacktrace_backtrace \
-lbacktrace \
-lboost_stacktrace_addr2line \
-lboost_random \
-lboost_math_tr1l \
-lboost_math_tr1f \
-lboost_math_tr1 \
-lboost_math_c99l \
-lboost_math_c99f \
-lboost_math_c99 \
-lboost_json \
-lboost_filesystem \
-lboost_coroutine \
-lboost_chrono \
-lboost_timer \
-lboost_serialization \
-lboost_regex \
-lboost_program_options \
-lboost_exception \
-lboost_date_time \
-lboost_context \
-lboost_container \
-lboost_charconv \
-lboost_atomic \
-lspdlogd \
-lfmtd \
-lm \
-ldramsim3 \
-lbooksim2 \
        -ldl";
        println!("{}", a);
    }
}
