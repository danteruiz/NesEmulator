#![allow(non_snake_case)]
use crate::nes_rom;
use std::collections::HashMap;

const MAX_CPU_MEMORY: usize = 0x10000;

mod StatusFlags {
    pub const CARRY: u8 = 1 << 0;
    pub const ZERO: u8 = 1 << 1;
    pub const INTER: u8 = 1 << 2;
    pub const DEC: u8 = 1 << 3;
    pub const OVERFLOW: u8 = 1 << 6;
    pub const NEGATIVE: u8 = 1 << 7;
}

enum AddressMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
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
        memory: [0; MAX_CPU_MEMORY],
    };
    return cpu;
}

pub fn load_rom(cpu: &mut Cpu, rom: nes_rom::Rom) {
    let program = rom.prog_rom;
    cpu.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
}

impl Cpu {
    fn readLeMemory(&self, address: u16) -> u16 {
        return u16::from_le_bytes([
            self.readMemory(address),
            self.readMemory(address + 1),
        ]);
    }
    pub fn reset(&mut self) {
        self.register_x = 0;
        self.register_a = 0;
        self.register_z = 0;

        self.program_pointer = self.readLeMemory(0xFFFC);
        self.stack_pointer = STACK_RESET;
    }

    pub fn readMemory(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    pub fn writeMemory(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    fn stack_push(&mut self, data: u8) {
        let address = STACK_ADDRESS as u16 + self.stack_pointer as u16;
        self.writeMemory(address, data);
        println!("stack pointer {}", self.stack_pointer);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.readMemory(self.program_pointer);
            self.program_pointer += 1;
            match opcode {
                0x78 => {
                    self.status |= StatusFlags::INTER;
                }
                0x4C => {
                    self.program_pointer =
                        self.readLeMemory(self.program_pointer);
                }
                0xA9 => {
                    let value = self.readMemory(self.program_pointer);

                    if value == 0 {
                        self.status |= StatusFlags::ZERO;
                    } else {
                        self.status &= !StatusFlags::ZERO;
                    }

                    if value & 0b1 > 0 {
                        self.status |= StatusFlags::NEGATIVE;
                    } else {
                        self.status |= !StatusFlags::NEGATIVE;
                    }
                    self.register_a = value;
                    self.program_pointer += 1;
                }
                0x8D => {
                    let address = self.readLeMemory(self.program_pointer);
                    self.writeMemory(address, self.register_a);
                    self.program_pointer += 2;
                }
                0xD8 => {
                    self.status |= !StatusFlags::DEC;
                }
                0xA2 => {
                    let value = self.readMemory(self.program_pointer);

                    if value == 0 {
                        self.status |= StatusFlags::ZERO;
                    } else {
                        self.status &= !StatusFlags::ZERO;
                    }

                    if value & 0b1 > 0 {
                        self.status |= StatusFlags::NEGATIVE;
                    } else {
                        self.status |= !StatusFlags::NEGATIVE;
                    }
                    self.register_x = value;
                    self.program_pointer += 1;
                }
                0x9A => {
                    let data = self.readMemory(self.program_pointer);
                    self.stack_push(data);
                    self.program_pointer += 1
                }
                0x00 => break,
                _ => {
                    println!("opcode {:x} is not supported", opcode);
                }
            }
        }
    }
}
