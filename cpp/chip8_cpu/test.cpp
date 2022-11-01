#include <iostream>
#include "cpu.hpp"

int main() {
    std::cout << "testing cpu compilation and simple implementation\n";
    auto& t_cpu = Chip8_cpu::get_instance();
    std::cout << "instanciated chip8 cpu\n";
    std::cout << "lalaland\n";
    t_cpu.get()->exec_op(0x8014);
    return 0;
}