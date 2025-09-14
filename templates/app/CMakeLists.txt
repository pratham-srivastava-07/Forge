cmake_minimum_required(VERSION 3.10)

project(cpp_framework VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Output exe inside build/
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

# Add include/ folder to header search path
include_directories(src/include)

# Collect all .cpp files automatically
file(GLOB SOURCES "src/*.cpp")

# Build executable
add_executable(cpp_framework ${SOURCES})
 