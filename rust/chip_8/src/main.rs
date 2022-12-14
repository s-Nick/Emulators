#![allow(dead_code)]

use std::convert::TryInto;
use rand::Rng;

#[derive(Debug,Clone)]
struct Cpu{
    registers: [u8; 16],
    curr_operation: u16,
    memory: [u8; 4096], 
    pc: u16,
    stack: Vec<u16>,
    stack_pointer: u8,
    index: u16,
}

impl Cpu{
    pub fn new() -> Cpu {
        Cpu{
            registers: [0; 16],
            curr_operation: 0x8014,
            memory: [0; 4096],
            pc: 0x202,
            stack: vec![0; 16], 
            stack_pointer: 0,
            // I do not like using the vec in this case, more over some fields should
            // not be mutable. I will check how to do it.
            index: 0,
        }
    }
    fn decode_op(&mut self) {
        let c = ((self.curr_operation & 0xF000) >> 12) as u8;
        let last_12 = (self.curr_operation & 0x0FFF) as u16;
        /*
         * keeping it and use it where the last 12 bits need 
         * to be split in 3 different vars.
        let x = ((self.curr_operation & 0x0F00) >> 8) as u8;
        let y = ((self.curr_operation & 0x00F0) >> 4) as u8;
        let d = (elf.curr_operation & 0x000F)  as u8;
        */
        match (c, last_12) {
            (0, _) => self.zeroes_c(last_12),
            (1, _) => self.jump_to_nnn(last_12),
            (2, _) => self.call_function(last_12),
            (3|4|6|7 | 0xC, _) => self.xkk_instructions(c, last_12),
            (5, _) => self.skip_from_register_comp(last_12),
            (8, _) => self.two_registers_op(last_12),
            (0xA, _) => self.load_addr_i(last_12),
            (0xB, _) => self.jump_nnn_v0(last_12),
            (_, _) => todo!(),
        }
    }

    fn zeroes_c(&mut self, last_12_bits: u16){
       let last_nibble = (last_12_bits & 0x000F) as u16;
       match last_nibble {
            0x000E => self.return_func(),
            0x0000 => todo!(),
            _ => todo!(),
       }
    }

    fn xkk_instructions(&mut self, d: u8, last_12_bits: u16) {
        let x: u8 = ((last_12_bits & 0x0F00) >> 8).try_into().unwrap();
        let kk: u8 = (last_12_bits & 0x00FF).try_into().unwrap();
        match d {
            3 => self.skip_equal(x, kk),
            4 => self.skip_not_equal(x, kk),
            6 => self.set_vx_to_kk(x, kk),
            7 => self.sum_vx_to_kk(x, kk),
            0xC => self.random_vx_kk(x, kk),
            _ => panic!(),
        };
    }

    fn two_registers_op(&mut self, last_12_bits: u16) {
        let x = ((last_12_bits & 0x0F00) >> 8) as u8;
        let y = ((last_12_bits & 0x00F0) >> 4) as u8;
        let d = ( last_12_bits & 0x000F)  as u8;
        match (x, y, d) {
            (_, _, 0) => self.registers[x as usize] = self.registers[y as usize],
            (_, _, 1) => self.registers[x as usize] |= self.registers[y as usize],
            (_, _, 2) => self.registers[x as usize] &= self.registers[y as usize],
            (_, _, 3) => self.registers[x as usize] ^= self.registers[y as usize],
            (_, _, 4) => self.registers[x as usize] += self.registers[y as usize],
            (_, _, 5) => self.registers[x as usize] -= self.registers[y as usize],
            (_, _, 6) => self.shift_right_x(x),
            (_, _, 7) => self.sub_yx(x, y),
            (_, _, 0xE) => self.shift_left_x(x),
            (_, _, _) => todo!(),
        }
    } 

    fn add_xy(&mut self, a: u8, b: u8) {
        self.registers[a as usize] += self.registers[b as usize];
    }

    fn setx_to_y(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] &= self.registers[y as usize];
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] -= self.registers[y as usize];
    }

    fn sub_yx(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize] - 
                                     self.registers[x as usize];
    }

    fn shift_right_x(&mut self, x: u8) {
        self.registers[0xF] = self.registers[x as usize] & 0x01;
        self.registers[x as usize] >>= 1;
    }

    fn shift_left_x(&mut self, x: u8) {
        self.registers[0xF] = match self.registers[x as usize] & 0x80 >> 7 { // check the mlb value
                                1 => 1,
                                0 => 0,
                                _ => panic!(),
                            };
        self.registers[x as usize] <<= 1;
    }

    fn jump_to_nnn(&mut self, last_12_bits: u16) {
        self.stack.push(self.pc);
        /*
        let nnn: u16 = (0xFFF as u16) |
                       ( ((x as u16) << (12 as u16)) & 
                       ((y as u16) << 8 as u16) & 
                       ((d as u16) << 4 as u16)) as u16;
        */
         self.pc =  last_12_bits;
    }

    fn return_func(&mut self){
        self.pc = self.stack.pop().unwrap();
        self.stack_pointer -= 1;
    }

    fn call_function(&mut self, last_12_bits: u16) {
        if self.stack.len() >= 16 {
            panic!();
        } 
        else {
            self.stack_pointer += 1;
            self.stack.push(self.pc);
            self.pc = last_12_bits;
        }
    }

    fn set_vx_to_kk(&mut self, x: u8, kk: u8 ) {
        self.registers[x as usize] = kk;
    }

    fn sum_vx_to_kk(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = self.registers[x as usize] + kk;
    }

    fn skip_equal(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] == kk {
            self.pc += 2;
        }
    }

    fn skip_not_equal(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] != kk {
            self.pc += 2;
        }
    }

    fn skip_from_register_comp(&mut self, last_12_bits: u16) {
        let x = ((last_12_bits & 0x0F00) >> 8) as u8;
        let y = ((last_12_bits & 0x00F0) >> 4) as u8;
        if self.registers[x as usize] == self.registers[y as usize] {
            self.pc += 2;
        }
    }

    fn load_addr_i(&mut self, last_12_bits: u16) {
        self.index = last_12_bits;
    }

    fn jump_nnn_v0(&mut self, last_12_bits: u16) {
        self.pc = last_12_bits + (self.registers[0] as u16);
    }

    fn random_vx_kk(&mut self, x: u8, kk: u8) {
        let rng = rand::thread_rng();
        let r_n1 = rng.clone().gen_range(0 ..=255);
        self.registers[x as usize] = r_n1 & kk;
    }
}

fn main() {
    println!("Hello, world!");
    let mut tmp = Cpu::new();
    println!("{:?} operation value 0x{:x}", tmp.registers[0], &tmp.curr_operation);
    tmp.registers[0] = 5;
    tmp.decode_op();
    println!("{:?} operation value 0x{:x}", tmp.registers[0], &tmp.curr_operation);
    tmp.registers[1] = 6;
    tmp.decode_op();
    tmp.curr_operation = 0x8034;
    tmp.registers[3] = 9;
    tmp.decode_op();
    tmp.curr_operation = 0x8010;
    for _i  in 0 ..5  {
        tmp.curr_operation += 1;
        tmp.decode_op();
        println!("{} operation value 0x{:x}", tmp.registers[0], &tmp.curr_operation);
    }
    tmp.stack[0] = 0;
    println!("{:?} operation value 0x{:x}", tmp.registers[0], &tmp.curr_operation);
}
