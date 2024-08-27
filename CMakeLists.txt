# Set the minimum version of CMake that can be used
# To find the cmake version run
# $ cmake --version
cmake_minimum_required(VERSION 3.26)

# Set the project name
project(rvemu)

set(CMAKE_CXX_STANDARD 20)

include_directories("inc")

# Create a sources variable with a link to all cpp files to compile
set(SOURCES
    "src/cxx/main.cxx"
)

# Add an executable with the above sources
add_executable(${PROJECT_NAME} ${SOURCES})

find_package(verilator HINTS $ENV{VERILATOR_ROOT})
if (NOT verilator_FOUND)
    message(FATAL_ERROR "Verilator not found!")
endif()

verilate(${PROJECT_NAME} SOURCES "src/sv/miniRV_SoC.sv" TRACE)

target_link_libraries(${PROJECT_NAME} PRIVATE ${PROJECT_SOURCE_DIR}/target/release/librvemu_hitsz.a)