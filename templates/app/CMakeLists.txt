cmake_minimum_required(VERSION 3.10)

project(cpp_framework VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Always put exe inside build/
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

# Add the executable (main.cpp must exist in src/)
add_executable(cpp_framework src/main.cpp)
