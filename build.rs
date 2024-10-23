use std::env;
fn main() {
    let is_release = env::var("PROFILE").unwrap() == "release";
    let cmake_out_dir = if is_release { "build_release" } else { "build" };
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let sim_lib_out_dir = format!("{}/{}/src", project_dir, cmake_out_dir);
    let booksim_lib_out_dir = format!("{}/{}/extern/booksim", project_dir, cmake_out_dir);
    let dramsim_lib_out_dir = format!("{}/{}/extern/NewtonSim", project_dir, cmake_out_dir);

    let cxx_flags = format!(
        "-L/home/sjq/.conan2/p/b/boost1180d7bad9b4b/p/lib \
-L/home/sjq/.conan2/p/b/bzip2838d787afafd8/p/lib \
-L/home/sjq/.conan2/p/b/zlib2a7e3b0641960/p/lib \
-L/home/sjq/.conan2/p/b/libbad1e079e9b66c0/p/lib \
-L/home/sjq/.conan2/p/b/spdlo8b7eb6342a16d/p/lib \
-L/home/sjq/.conan2/p/b/fmtf09e886be2602/p/lib \
-L{sim_lib_out_dir} \
-L{booksim_lib_out_dir} \
-L{dramsim_lib_out_dir} \
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
-ldl"
    );
    // println!("cargo:warning=Building in {} mode", cmake_out_dir);
    println!("cargo:rustc-flags={}", cxx_flags);
    // use c++11 abi, set env: _GLIBCXX_USE_CXX11_ABI=1
    cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .cpp(true)
        .cpp_link_stdlib("stdc++")
        .includes(["include", "src/cpp/src"])
        .compile("new_pims_lib");

    println!("cargo:rerun-if-changed=include/cpp_main.h");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!(
        "cargo:rerun-if-changed={}/{}",
        sim_lib_out_dir, "libSimulator_lib.a"
    );
    println!(
        "cargo:rerun-if-changed={}/{}",
        booksim_lib_out_dir, "libbooksim2.a"
    );
}
