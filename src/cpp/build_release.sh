rm -rf build_release
conan install . --output-folder=build_release --build=missing --profile=default
cd build_release
cmake .. -DCMAKE_TOOLCHAIN_FILE="conan_toolchain.cmake" -DCMAKE_EXPORT_COMPILE_COMMANDS=On -DCMAKE_BUILD_TYPE=Release
make -j