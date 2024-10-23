rm -rf build
conan install . --output-folder=build --build=missing --profile=debug
cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE="conan_toolchain.cmake" -DCMAKE_EXPORT_COMPILE_COMMANDS=On -DCMAKE_BUILD_TYPE=Debug
make -j