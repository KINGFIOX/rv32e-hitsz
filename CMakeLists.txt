# Set the minimum version of CMake that can be used
# To find the cmake version run
# $ cmake --version
cmake_minimum_required(VERSION 3.21)

# Set the project name
project(rvemu)

set(CMAKE_CXX_STANDARD 20)

include_directories("${PROJECT_SOURCE_DIR}/inc")

# Create a sources variable with a link to all cpp files to compile
set(SOURCES
    "src/cxx/main.cxx"
)

# Add an executable with the above sources
add_executable(${PROJECT_NAME} ${SOURCES})

target_link_libraries(${PROJECT_NAME} ${PROJECT_SOURCE_DIR}/target/release/librvemu_hitsz.a)
