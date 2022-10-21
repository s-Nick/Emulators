#![allow(dead_code)]

#[derive(Debug,Clone, Copy)]
struct Cpu{
    registers: [u8; 16],
    operation: u16,
}

impl Cpu{
    pub fn new() -> Cpu {
        Cpu{
            registers: [0; 16],
            operation: 0x8014,
        }
    }
    fn decode_op(&mut self) {
        let c = ((self.operation & 0xF000) >> 12) as u8;
        let x = ((self.operation & 0x0F00) >> 8) as u8;
        let y = ((self.operation & 0x00F0) >> 4) as u8;
        let d = (self.operation & 0x000F)  as u8;
        match (c, x, y, d) {
           (8, _, _, 0) => self.setx_to_y(x,y),
           (8, _ ,_, 1) => self.or_xy(x, y),
           (8, _ ,_, 2) => self.and_xy(x, y),
           (8, _ ,_, 3) => self.xor_xy(x, y),
           (8, _, _, 4) => self.add_xy(x,y),
           (8, _ ,_, 5) => self.sub_xy(x, y),
           (8, _ ,_, 6) => todo!(),//self.or_xy(x, y),
           (8, _ ,_, 7) => todo!(),//self.or_xy(x, y),
           (_, _, _, _) => todo!(),
        }
    }
    fn two_registers_op(&mut self, x: u8, y: u8, d: u8) {
        match (x, y, d) {
            (_, _, 0) => self.registers[x as usize] = self.registers[y as usize],
            (_, _, 1) => self.registers[x as usize] |= self.registers[y as usize],
            (_, _, 2) => self.registers[x as usize] &= self.registers[y as usize],
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

}

fn main() {
    println!("Hello, world!");
    let mut tmp = Cpu::new();
    println!("{:?} operation value 0x{:x}", tmp.registers[0], &tmp.operation);
    tmp.registers[0] = 5;
    tmp.decode_op();
    println!("{:?} operation value 0x{:x}", tmp.registers[0], &tmp.operation);
    tmp.registers[1] = 6;
    tmp.decode_op();
    tmp.operation = 0x8034;
    tmp.registers[3] = 9;
    tmp.decode_op();
    tmp.operation = 0x8010;
    for _i  in 0 ..5  {
        tmp.operation += 1;
        tmp.decode_op();
        println!("{} operation value 0x{:x}", tmp.registers[0], &tmp.operation);
    }
    println!("{:?} operation value 0x{:x}", tmp.registers[0], &tmp.operation);
}
