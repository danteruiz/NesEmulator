use crate::nes::cpu;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum AddressMode {
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
        return write!(
            f,
            "{}",
            match self {
                AddressMode::Immediate => "Immediate",
                AddressMode::Implied => "Implied",
                AddressMode::ZeroPage => "ZeroPage",
                AddressMode::ZeroPageX => "ZeroPageX",
                AddressMode::ZeroPageY => "ZeroPageY",
                AddressMode::Absolute => "Absolute",
                AddressMode::AbsoluteX => "AbsoluteX",
                AddressMode::AbsoluteY => "AbsoluteY",
                AddressMode::IndirectX => "IndirectX",
                AddressMode::IndirectY => "IndirectY",
            }
        );
    }
}

pub struct Opcode {
    pub code: u8,
    pub name: &'static str,
    pub bytes: u8,
    pub address_mode: AddressMode,
    pub callback: fn(&mut cpu::Cpu, &AddressMode),
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
        callback: fn(&mut cpu::Cpu, &AddressMode),
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
    pub static ref OPCODE_MAP: HashMap<u8, Opcode> = {
        let mut opcode_map = HashMap::new();
        opcode_map.insert(
            0x69,
            Opcode::new(0x69, "ADC", 2, AddressMode::Immediate, cpu::Cpu::adc),
        );
        opcode_map.insert(
            0x78,
            Opcode::new(0x78, "SEI", 1, AddressMode::Immediate, cpu::Cpu::sei),
        );
        opcode_map.insert(
            0x00,
            Opcode::new(0x00, "BRK", 1, AddressMode::Immediate, cpu::Cpu::brk),
        );
        opcode_map.insert(
            0x4C,
            Opcode::new(0x4C, "JMP", 3, AddressMode::Absolute, cpu::Cpu::jmp),
        );
        opcode_map.insert(
            0xA9,
            Opcode::new(0xA9, "LDA", 2, AddressMode::Immediate, cpu::Cpu::lda),
        );
        opcode_map.insert(
            0x8D,
            Opcode::new(0x8D, "STA", 3, AddressMode::Absolute, cpu::Cpu::sta),
        );
        opcode_map.insert(
            0x95,
            Opcode::new(0x95, "STA", 2, AddressMode::ZeroPageX, cpu::Cpu::sta),
        );
        opcode_map.insert(
            0x9D,
            Opcode::new(0x9D, "STA", 3, AddressMode::AbsoluteX, cpu::Cpu::sta),
        );
        opcode_map.insert(
            0xD8,
            Opcode::new(0xD8, "CLD", 1, AddressMode::Implied, cpu::Cpu::cld),
        );
        opcode_map.insert(
            0xA2,
            Opcode::new(0xA2, "LDX", 2, AddressMode::Immediate, cpu::Cpu::ldx),
        );
        opcode_map.insert(
            0x9A,
            Opcode::new(0x9A, "TXS", 1, AddressMode::Implied, cpu::Cpu::txs),
        );
        opcode_map.insert(
            0x20,
            Opcode::new(0x20, "JSR", 3, AddressMode::Absolute, cpu::Cpu::jsr),
        );
        opcode_map.insert(
            0xAA,
            Opcode::new(0xAA, "TAX", 1, AddressMode::Implied, cpu::Cpu::tax),
        );

        return opcode_map;
    };
}
