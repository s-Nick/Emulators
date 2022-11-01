cmake_minimum_required(VERSION 3.14)

set(CMAKE_INSTALL_PREFIX /home/p4rs3c/Documents/emulators/cpp/chip8_cpu)
set(CMAKE_CXX_COMPILER /usr/bin/g++)

project(Chip8Emulator)
#add_library(cpu_library STATIC cpu.cpp)
set(LIBRARY_OUTPU_PATH lib)

add_library(cpu_library STATIC cpu.cpp)
add_executable(test test.cpp )
#add_library(cpu_library STATIC cpu.cpp)
target_link_libraries(test cpu_library)

#install(TARGETS cpu_library)
install(TARGETS test
        CONFIGURATIONS Release
        RUNTIME DESTINATION
        ) 

