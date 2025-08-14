cmake_minimum_required(VERSION 3.10)
project({{project_name}})

set(CMAKE_CXX_STANDARD 17)

add_library({{project_name}} src/lib.cpp)
target_include_directories({{project_name}} PUBLIC include)
