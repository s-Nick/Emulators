#![allow(dead_code)]


#[derive(Debug,Clone)]
struct Cpu{
    registers: [u8; 16],
    curr_operation: u16,
    memory: [u16; 4096], // atm the memory can take twice the amount of data than common chip8
                         // due to the utilization of u16 instead of u8.
    pc: u16,
    stack: Vec<u16>,
    stack_pointer: u8,
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
            (8, _) => self.two_registers_op(last_12),
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
            (_, _, 0xE) => todo!(),
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
        self.registers[0xF] = self.registers[x as usize] & 0x8;
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
