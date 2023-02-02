cmake_minimum_required(VERSION 3.11)

project(emu-nes)
set(TARGET_NAME emu-nes)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED_ON)

add_compile_options(
  -W
  -Wall
  -Wextra
  -Werror
  -Wpedantic
  -fvisibility=hidden
  -Wconversion
  -fsanitize=address
  -std=c++20
)

add_link_options(-fsanitize=address)

file(GLOB source_files "src/*.h" "src/*.cpp")

add_executable(${PROJECT_NAME} ${source_files})
target_include_directories(${TARGET_NAME} PUBLIC ${CMAKE_CURRENT_BINARY_DIR}/src)
