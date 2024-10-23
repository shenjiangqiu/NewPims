rm -rf build_rel_dbg
conan install . --output-folder=build_rel_dbg --build=missing --profile=RelWithDebInfo
cd build_rel_dbg
cmake .. -DCMAKE_TOOLCHAIN_FILE="conan_toolchain.cmake" -DCMAKE_EXPORT_COMPILE_COMMANDS=On -DCMAKE_BUILD_TYPE=RelWithDebInfo
make -j