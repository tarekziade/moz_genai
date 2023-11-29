cmake_minimum_required(VERSION 3.0)
project(my_cpp_project)

add_executable(my_cpp_app main.cpp)

# Link the Rust library to the C++ executable
target_link_libraries(my_cpp_app ${CMAKE_SOURCE_DIR}/../rust_lib/target/debug/libmy_rust_lib.so)

