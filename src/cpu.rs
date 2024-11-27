pub enum AddressingMode {
    Implicit,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

static CYCLE_TABLE: [u8; 256] = [
    /*0x00*/ 7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, /*0x10*/ 2, 5, 2, 8, 4, 4,
    6, 6, 2, 4, 2, 7, 4, 4, 7, 7, /*0x20*/ 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6,
    /*0x30*/ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, /*0x40*/ 6, 6, 2, 8, 3, 3,
    5, 5, 3, 2, 2, 2, 3, 4, 6, 6, /*0x50*/ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /*0x60*/ 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6, /*0x70*/ 2, 5, 2, 8, 4, 4,
    6, 6, 2, 4, 2, 7, 4, 4, 7, 7, /*0x80*/ 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
    /*0x90*/ 2, 6, 2, 6, 4, 4, 4, 4, 2, 5, 2, 5, 5, 5, 5, 5, /*0xA0*/ 2, 6, 2, 6, 3, 3,
    3, 3, 2, 2, 2, 2, 4, 4, 4, 4, /*0xB0*/ 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4,
    /*0xC0*/ 2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, /*0xD0*/ 2, 5, 2, 8, 4, 4,
    6, 6, 2, 4, 2, 7, 4, 4, 7, 7, /*0xE0*/ 2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
    /*0xF0*/ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
];

/// 状态寄存器标志位
pub const FLAG_C: u8 = 0b00000001; // Carry Flag (进位标志)
pub const FLAG_Z: u8 = 0b00000010; // Zero Flag (零标志)
pub const FLAG_I: u8 = 0b00000100; // Interrupt Disable (中断禁用)
pub const FLAG_D: u8 = 0b00001000; // Decimal Mode (十进制模式，2A03 不支持)
pub const FLAG_B: u8 = 0b00010000; // Break Command (中断请求)
pub const FLAG_V: u8 = 0b01000000; // Overflow Flag (溢出标志)
pub const FLAG_N: u8 = 0b10000000; // Negative Flag (负数标志)

pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: u8,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.a = value;

        // TODO update p
    }
}

impl CPU {
    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.p |= flag;
        } else {
            self.p &= !flag;
        }
    }

    pub fn get_glag(&mut self, flag: u8) -> bool {
        (self.p & flag) != 0
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;
        (hi << 8) | lo
    }

    pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let lo = data as u8;
        let hi = (data >> 8) as u8;
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage => self.mem_read(self.pc) as u16,
            AddressingMode::ZeroPageX => {
                let addr = self.mem_read(self.pc) as u16;
                addr.wrapping_add(self.x as u16)
            }
            AddressingMode::ZeroPageY => {
                let addr = self.mem_read(self.pc) as u16;
                addr.wrapping_add(self.y as u16)
            }
            AddressingMode::Absolute => self.mem_read_u16(self.pc),
            AddressingMode::AbsoluteX => {
                let addr = self.mem_read_u16(self.pc);
                addr.wrapping_add(self.x as u16)
            }
            AddressingMode::AbsoluteY => {
                let addr = self.mem_read_u16(self.pc);
                addr.wrapping_add(self.y as u16)
            }
            // val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256)
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.pc);
                let ptr: u8 = (base as u8).wrapping_add(self.x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            // val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y)
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.pc);
                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.y as u16);
                deref
            }
            _ => panic!("Unimplemented addressing mode"),
        }
    }

    pub fn run(&mut self) {
        // note: we move  intialization of program_counter from here to load function
        loop {
            let opscode = self.mem_read(self.pc);
            self.pc += 1;
            match opscode {
                0xA9 => {
                    self.lda(&AddressingMode::Immediate);
                    self.pc += 1;
                }
                0xA5 => {
                    self.lda(&AddressingMode::ZeroPage);
                    self.pc += 1;
                }
                0xB5 => {
                    self.lda(&AddressingMode::ZeroPageX);
                    self.pc += 1;
                }
                0xAD => {
                    self.lda(&AddressingMode::Absolute);
                    self.pc += 2;
                }
                0xBD => {
                    self.lda(&AddressingMode::AbsoluteX);
                    self.pc += 2;
                }
                0xB9 => {
                    self.lda(&AddressingMode::AbsoluteY);
                    self.pc += 2;
                }
                0xA1 => {
                    self.lda(&AddressingMode::IndirectX);
                    self.pc += 1;
                }
                0xB1 => {
                    self.lda(&AddressingMode::IndirectY);
                    self.pc += 1;
                }
                _ => {}
            }
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.p = 0;
        self.pc = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }
}
