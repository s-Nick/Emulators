#include "cpu.hpp"

const auto split_last_12_bit = [](const uint16_t op_code) -> std::pair<uint8_t,uint8_t> {
  uint8_t x = static_cast<uint8_t>(op_code & 0x0F00u) >> 8u;
  uint8_t kk = static_cast<uint8_t>(op_code & 0x00FFu);
  return std::pair<uint8_t,uint8_t>{x,kk};
};

const auto get_nnn = [](const uint16_t op_code) -> uint16_t {
  uint16_t nnn = (op_code & 0x0FFFu);
  return nnn;
};

std::unique_ptr<Chip8_cpu> Chip8_cpu::cpu_= nullptr;

Chip8_cpu::Chip8_cpu() : registers(16){
}

std::unique_ptr<Chip8_cpu>& Chip8_cpu::get_instance(){
  cpu_ = std::make_unique<Chip8_cpu>(Chip8_cpu());
  return cpu_;
}


void Chip8_cpu::exec_op(const uint16_t op_code) {
  const uint8_t d = static_cast<uint8_t>((op_code & 0xF000u) >> 12);
  switch (d) {
  case 0u:
    Chip8_cpu::return_from_funct(op_code);
    break;
  case 1u:
    Chip8_cpu::jump_nnn(op_code);
    break;
  case 3u:
    Chip8_cpu::skip_equal(op_code);
    break;
  case 4u:
    Chip8_cpu::skip_not_equal(op_code);
    break;
  case 6u:
    Chip8_cpu::loadi_to_vx(op_code);
    break;
  case 7u:
    Chip8_cpu::addi_to_vx(op_code);
    break;
  case 8u:
    Chip8_cpu::op_8XY(op_code);
    break;
  case 0xAu:
    Chip8_cpu::load_mem_addr_I(op_code);
    break;
  case 0xBu:
    Chip8_cpu::jump_to_v0_plus_nnn(op_code);
    break;
  case 0xCu:
    Chip8_cpu::set_random_vx(op_code);
    break;
  default:
    break;
  }
}

void Chip8_cpu::op_8XY(const uint16_t op_code) {
  const uint8_t vx = (op_code & 0x0F00u) >> 8u;
  const uint8_t vy = (op_code & 0x00F0u) >> 4u;
  const uint8_t operation = (op_code & 0x000Fu);
  switch(operation) {
    case 0:
      registers[vx] = vy; //0x8xy0 store ry in rx
      break;
    case 1:
      registers[vx] |= registers[vy];
      break;
    case 2:
      registers[vx] &= registers[vy];
      break;
    case 3:
      registers[vx] ^= registers[vy];
      break;
    case 4:
      registers[15] = ((registers[vx] + registers[vy]) > 0xFFu) ? 1u : 0u;
      registers[vx] += registers[vy];
      break;
    case 5:
      registers[15] = (registers[vx] < registers[vy]) ? 0 : 1;
      registers[vx] -= registers[vy];
      break;
    case 6:
      registers[15] = registers[vx] & 0x1u;
      registers[vx] = registers[vx] >> 0x1u;
      break;
    case 7:
      registers[15] = (registers[vy] > registers[vx]) ? 1 : 0;
      registers[vx] = registers[vy] - registers[vx];
      break;  
    case 0xE:
      registers[15] = static_cast<uint8_t>(registers[vx] & 0x80u );
      registers[vx] <<= 1;
    default:
      std::cout << "should not be here! " << __LINE__ << '\n';
      break;
  };
  return;
}

void Chip8_cpu::skip_equal(const uint16_t op_code) {
  auto last_12_bits = split_last_12_bit(op_code);
  if (registers[last_12_bits.first] == last_12_bits.second) 
    this->pc += 2u;
  return;
}

void Chip8_cpu::skip_not_equal(const uint16_t op_code) {
  auto last_12_bits = split_last_12_bit(op_code);
  if (registers[last_12_bits.first] != last_12_bits.second) 
    this->pc += 2u;
  return;
}

void Chip8_cpu::loadi_to_vx(const uint16_t op_code) {
  auto last_12_bits = split_last_12_bit(op_code);
  registers[last_12_bits.first] = last_12_bits.second;
}

void Chip8_cpu::addi_to_vx(const uint16_t op_code) {
  auto last_12_bits = split_last_12_bit(op_code);
  registers[last_12_bits.first] += last_12_bits.second;
  return;
}

void Chip8_cpu::set_random_vx(const uint16_t op_code) {
  srand(time(NULL));
  auto last_12_bits = split_last_12_bit(op_code);
  registers[last_12_bits.first] = static_cast<uint8_t>((rand() % 255)) & last_12_bits.second; 
  return;
}

void Chip8_cpu::jump_nnn(const uint16_t op_code) {
  this->pc = get_nnn(op_code);
  return;
}

void Chip8_cpu::call_nnn(const uint16_t op_code) {
  // TODO! missing handling situation where stack size is already 16
  this->stack.push(this->pc);
  this->sp++;
  this->pc = get_nnn(op_code);
  return;
}

void Chip8_cpu::load_mem_addr_I(const uint16_t op_code){
  this->address_register_I = get_nnn(op_code);
  return;
}

void Chip8_cpu::jump_to_v0_plus_nnn(const uint16_t op_code) {
  this->pc = registers[0u] + get_nnn(op_code);
  return;
}