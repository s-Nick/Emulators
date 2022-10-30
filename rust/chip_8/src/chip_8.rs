pub mod Chip_8 {
    #[derive(Debug)]
    pub struct Cpu {
        register: [u8; 16],
        operation: u16,
    }
}

