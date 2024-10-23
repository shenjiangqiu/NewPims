cmake_minimum_required(VERSION 3.15.0)
set(project_name "AiFrameworkSim")
project(${project_name})
# include("${CMAKE_BINARY_DIR}/conanbuildinfo.cmake")
# conan_basic_setup()

# find_package(Boost 1.70 REQUIRED COMPONENTS program_options)
# message("LIB ${Boost_LIBRARY_DIRS}")
list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/extern)

# C++ setttings
set(CMAKE_CXX_STANDARD 17)
set(ONNX_ML 1)
set(JSON_BuildTests OFF CACHE INTERNAL "")
add_compile_definitions(_GLIBCXX_USE_CXX11_ABI=1)
message("BINARY DIR ${CMAKE_BINARY_DIR}")

# For debug

# if(CMAKE_BUILD_TYPE STREQUAL "Release")
#     message("Release mode")
#     set(CMAKE_CXX_FLAGS "-O3 -Wall -Wextra -rdynamic")
# else()
#     set(CMAKE_CXX_FLAGS "-g -O0 -Wall -Wextra -rdynamic")
#     message("Debug mode")
# endif()
# Build source

# add rust
# add_subdirectory("${PROJECT_SOURCE_DIR}/src/rust/neupimrust")

# Add library NewtonSim
add_subdirectory("${PROJECT_SOURCE_DIR}/extern/NewtonSim")

# Add libaray booksim
add_subdirectory("${PROJECT_SOURCE_DIR}/extern/booksim")

add_subdirectory("${PROJECT_SOURCE_DIR}/src")


enable_testing()
# add_subdirectory("${PROJECT_SOURCE_DIR}/tests")

