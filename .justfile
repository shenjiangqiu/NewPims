build_cmake:
    rm -rf build
    conan install ./src/cpp --output-folder=build --build=missing --profile=debug
    cmake -B build -S src/cpp -DCMAKE_TOOLCHAIN_FILE="conan_toolchain.cmake" -DCMAKE_EXPORT_COMPILE_COMMANDS=On -DCMAKE_BUILD_TYPE=Debug
build:
    cmake --build build -- Simulator_lib -j
build_all: build_cmake build
    echo "Build done"

build_release_cmake:
    rm -rf build_release
    conan install ./src/cpp --output-folder=build_release --build=missing --profile=default
    cmake -B build_release -S src/cpp -DCMAKE_TOOLCHAIN_FILE="conan_toolchain.cmake" -DCMAKE_EXPORT_COMPILE_COMMANDS=On -DCMAKE_BUILD_TYPE=Release
    # cmake --build build_release
build_release:
    cmake --build build_release -- Simulator_lib -j
build_release_all: build_release_cmake build_release
    echo "Build release done"

clean:
    rm -rf ./build
clean_release:
    rm -rf ./build_release

run_release:
    bash ./brun_release.sh
run:
    bash ./brun.sh