use crate::nes_rom;
use std::collections::HashMap;
use std::fmt;

const MAX_CPU_MEMORY: usize = 0x10000;

mod status_flags {
    pub const CARRY: u8 = 1 << 0;
    pub const ZERO: u8 = 1 << 1;
    pub const INTER: u8 = 1 << 2;
    pub const DEC: u8 = 1 << 3;
    pub const OVERFLOW: u8 = 1 << 6;
    pub const NEGATIVE: u8 = 1 << 7;
}

#[derive(Debug)]
enum AddressMode {
    Immediate,
    Implied,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl fmt::Display for AddressMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

struct Opcode {
    code: u8,
    name: &'static str,
    bytes: u8,
    address_mode: AddressMode,
    callback: fn(&mut Cpu, &AddressMode),
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "opcode:${:X} bytes:{} name:{} mode:{}",
            self.code, self.bytes, self.name, self.address_mode
        );
    }
}

impl Opcode {
    fn new(
        code: u8,
        name: &'static str,
        bytes: u8,
        address_mode: AddressMode,
        callback: fn(&mut Cpu, &AddressMode),
    ) -> Opcode {
        return Opcode {
            code: code,
            name: name,
            bytes: bytes,
            address_mode: address_mode,
            callback: callback,
        };
    }
}

lazy_static! {
    static ref OPCODE_MAP: HashMap<u8, Opcode> = {
        let mut opcode_map = HashMap::new();
        opcode_map.insert(
            0x69,
            Opcode::new(0x69, "ADC", 2, AddressMode::Immediate, Cpu::adc),
        );
        opcode_map.insert(
            0x78,
            Opcode::new(0x78, "SEI", 1, AddressMode::Immediate, Cpu::sei),
        );
        opcode_map.insert(
            0x00,
            Opcode::new(0x00, "BRK", 1, AddressMode::Immediate, Cpu::brk),
        );
        opcode_map.insert(
            0x4C,
            Opcode::new(0x4C, "JMP", 3, AddressMode::Absolute, Cpu::jmp),
        );
        opcode_map.insert(
            0xA9,
            Opcode::new(0xA9, "LDA", 2, AddressMode::Immediate, Cpu::lda),
        );
        opcode_map.insert(
            0x8D,
            Opcode::new(0x8D, "STA", 3, AddressMode::Absolute, Cpu::sta),
        );
        opcode_map.insert(
            0xD8,
            Opcode::new(0xD8, "CLD", 1, AddressMode::Implied, Cpu::cld),
        );

        opcode_map.insert(
            0xA2,
            Opcode::new(0xA2, "LDX", 2, AddressMode::Immediate, Cpu::ldx),
        );
        opcode_map.insert(
            0x9A,
            Opcode::new(0x9A, "TXS", 1, AddressMode::Implied, Cpu::txs),
        );

        return opcode_map;
    };
}
const STACK_ADDRESS: u16 = 0x0100;
const STACK_RESET: u8 = 0xff;
pub struct Cpu {
    pub program_pointer: u16,
    pub stack_pointer: u8,
    pub status: u8,
    register_a: u8,
    register_x: u8,
    register_z: u8,
    register_y: u8,
    memory: [u8; MAX_CPU_MEMORY],
}

pub fn create_cpu() -> Cpu {
    let cpu = Cpu {
        program_pointer: 0,
        stack_pointer: 0,
        status: 0,
        register_a: 0,
        register_x: 0,
        register_z: 0,
        register_y: 0,
        memory: [0; MAX_CPU_MEMORY],
    };
    return cpu;
}

pub fn load_rom(cpu: &mut Cpu, rom: nes_rom::Rom) {
    let program = rom.prog_rom;
    cpu.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
}

impl Cpu {
    fn read_16_bits(&self, address: u16) -> u16 {
        return u16::from_le_bytes([
            self.read_memory(address),
            self.read_memory(address + 1),
        ]);
    }
    pub fn reset(&mut self) {
        self.register_x = 0;
        self.register_a = 0;
        self.register_z = 0;
        self.register_y = 0;

        self.program_pointer = self.read_16_bits(0xFFFC);
        self.stack_pointer = STACK_RESET;
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    pub fn write_memory(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    fn stack_push(&mut self, data: u8) {
        let address = STACK_ADDRESS as u16 + self.stack_pointer as u16;
        self.write_memory(address, data);
        println!("stack pointer {}", self.stack_pointer);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn read_memory_mode(&mut self, address_mode: &AddressMode) -> u16 {
        match address_mode {
            AddressMode::Immediate => {
                return self.program_pointer;
            }
            AddressMode::Implied => {
                return self.program_pointer;
            }
            AddressMode::ZeroPage => {
                return self.read_memory(self.program_pointer) as u16;
            }
            AddressMode::ZeroPageX => {
                let address = self.read_memory(self.program_pointer);
                return address.wrapping_add(self.register_x) as u16;
            }
            AddressMode::ZeroPageY => {
                let address = self.read_memory(self.program_pointer);
                return address.wrapping_add(self.register_y) as u16;
            }
            AddressMode::Absolute => {
                return self.read_16_bits(self.program_pointer) as u16;
            }
            AddressMode::AbsoluteX => {
                let address = self.read_16_bits(self.program_pointer);
                return address.wrapping_add(self.register_x as u16);
            }
            AddressMode::AbsoluteY => {
                let address = self.read_16_bits(self.program_pointer);
                return address.wrapping_add(self.register_y as u16);
            }
            AddressMode::IndirectX => {
                let base = self.read_memory(self.program_pointer);

                let address = base.wrapping_add(self.register_x);
                return u16::from_le_bytes([address, address.wrapping_add(1)]);
            }
            AddressMode::IndirectY => {
                let base = self.read_memory(self.program_pointer);
                let address = u16::from_le_bytes([base, base.wrapping_add(1)]);
                return address.wrapping_add(self.register_y as u16);
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let code = self.read_memory(self.program_pointer);
            self.program_pointer += 1;
            let last_program_pointer = self.program_pointer;

            let opcode = OPCODE_MAP
                .get(&code)
                .expect(&format!("Opcode:${:X} is not supported", code));

            println!("{}", opcode);
            if code == 0x00 {
                break;
            }
            let callback = opcode.callback;
            callback(self, &opcode.address_mode);

            // none of the command changed the program pointer
            // increment the program pointer by the opcode bytes
            if last_program_pointer == self.program_pointer {
                self.program_pointer += (opcode.bytes - 1) as u16;
            }
        }
    }

    fn sei(&mut self, _address_mode: &AddressMode) {
        self.status |= status_flags::INTER;
    }

    fn adc(&mut self, _address_mode: &AddressMode) {}

    fn jmp(&mut self, address_mode: &AddressMode) {
        self.program_pointer = self.read_memory_mode(address_mode);
    }

    fn lda(&mut self, address_mode: &AddressMode) {
        let value = self.read_memory_mode(address_mode);

        if value == 0 {
            self.status |= status_flags::ZERO;
        } else {
            self.status &= !status_flags::ZERO;
        }

        if value & 0b1 > 0 {
            self.status |= status_flags::NEGATIVE;
        } else {
            self.status |= !status_flags::NEGATIVE;
        }
        self.register_a = value as u8;
    }

    fn sta(&mut self, address_mode: &AddressMode) {
        let address = self.read_memory_mode(address_mode);
        self.write_memory(address, self.register_a);
    }

    fn cld(&mut self, _address_mode: &AddressMode) {
        self.status |= !status_flags::DEC;
    }

    fn ldx(&mut self, address_mode: &AddressMode) {
        let value = self.read_memory_mode(address_mode);

        if value == 0 {
            self.status |= status_flags::ZERO;
        } else {
            self.status &= !status_flags::ZERO;
        }

        if value & 0b1 > 0 {
            self.status |= status_flags::NEGATIVE;
        } else {
            self.status |= !status_flags::NEGATIVE;
        }
        self.register_x = value as u8;
    }

    fn txs(&mut self, address_mode: &AddressMode) {
        let data = self.read_memory_mode(address_mode);
        self.stack_push(data as u8);
    }

    fn brk(&mut self, _address_mode: &AddressMode) {}
}
