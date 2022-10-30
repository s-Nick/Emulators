#ifndef CHIP8_CPU_HPP
#define CHIP8_CPU_HPP

#include <vector>
#include <cstdint>
#include <cstdlib>
#include <time.h>
#include <array>
#include <stack>
#include <memory>
#include <iostream>

class Chip8_cpu {

  private:
    
    Chip8_cpu();
    
    /*
    Chip8_cpu(const Chip8_cpu& cpu) = delete;
    void operator=(const Chip8_cpu& cpu) = delete;
    Chip8_cpu(Chip8_cpu&& cpu) = delete;
    void operator=(Chip8_cpu&& cpu) = delete;
    */
    static std::unique_ptr<Chip8_cpu> cpu_;
    
    std::vector<uint8_t> registers;
    std::array<uint8_t,4096> memory();
    // this address should be 12 bits, so probably it should be a bitset. Check later
    // this address store memory address
    uint16_t address_register_I;
    std::stack<uint16_t> stack;
    // no idea how to implement the two timers
    // use uint and then check
    uint16_t delay_timer{};
    uint16_t sound_timer{};
    uint16_t op_code{};
    uint16_t pc{};
    uint8_t sp{};

    // All these operations are related to the value of x and y. You can write a single operation
    // and then switch the operation accordingly with the last four bits of the op_code;
    void return_from_funct(const uint16_t op_code);
    void op_8XY(const uint16_t op_code);    
    void skip_equal(const uint16_t op_code); //0x3xkk
    void skip_not_equal(const uint16_t op_code); // 0x4xkk
    void loadi_to_vx(const uint16_t op_code);
    void addi_to_vx(const uint16_t op_code);
    void set_random_vx(const uint16_t op_code);
    void jump_nnn(const uint16_t op_code); // 0x1nnn
    void call_nnn(const uint16_t op_code); // 0x2nnn
    void load_mem_addr_I(const uint16_t op_code); // 0xAnnn
    void jump_to_v0_plus_nnn(const uint16_t op_code); // 0xBnnn

  public:
    
    ~Chip8_cpu() = default;
    static std::unique_ptr<Chip8_cpu>& get_instance();
    void exec_op(const uint16_t op_code);    
};

#endif
