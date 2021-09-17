// cpu.rs
//
// Created on 2021/09/16 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use crate::nes::opcode;
use crate::nes::rom;

const MAX_CPU_MEMORY: usize = 0x10000;

mod status_flags {
    pub const CARRY: u8 = 1 << 0;
    pub const ZERO: u8 = 1 << 1;
    pub const INTER: u8 = 1 << 2;
    pub const DEC: u8 = 1 << 3;
    pub const OVERFLOW: u8 = 1 << 6;
    pub const NEGATIVE: u8 = 1 << 7;
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

pub fn load_rom(cpu: &mut Cpu, rom: rom::Rom) {
    let program = rom.prog_rom;
    // for index in 0..program.len() {
    //     println!("{:X} - {:02X}", 0x8000 + index, program[index]);
    // }
    cpu.memory[0xC000..(0xC000 + program.len())].copy_from_slice(&program[..]);
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

        // 0xFFFC
        self.program_pointer = 0xC000; //self.read_16_bits(0xC000);
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
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, data: u16) {
        let hi: u8 = (data >> 8) as u8;
        let lo: u8 = (data & 0xff) as u8;
        let mut address = STACK_ADDRESS as u16 + self.stack_pointer as u16;
        self.write_memory(address, hi);
        address = address.wrapping_sub(1);
        self.write_memory(address, lo);
        self.stack_pointer = self.stack_pointer.wrapping_sub(2);
    }

    fn read_memory_mode(&mut self, address_mode: &opcode::AddressMode) -> u16 {
        match address_mode {
            opcode::AddressMode::Immediate => {
                return self.program_pointer;
            }
            opcode::AddressMode::Implied => {
                return self.program_pointer;
            }
            opcode::AddressMode::ZeroPage => {
                return self.read_memory(self.program_pointer) as u16;
            }
            opcode::AddressMode::ZeroPageX => {
                let address = self.read_memory(self.program_pointer);
                return address.wrapping_add(self.register_x) as u16;
            }
            opcode::AddressMode::ZeroPageY => {
                let address = self.read_memory(self.program_pointer);
                return address.wrapping_add(self.register_y) as u16;
            }
            opcode::AddressMode::Absolute => {
                return self.read_16_bits(self.program_pointer) as u16;
            }
            opcode::AddressMode::AbsoluteX => {
                let address = self.read_16_bits(self.program_pointer);
                return address.wrapping_add(self.register_x as u16);
            }
            opcode::AddressMode::AbsoluteY => {
                let address = self.read_16_bits(self.program_pointer);
                return address.wrapping_add(self.register_y as u16);
            }
            opcode::AddressMode::IndirectX => {
                let base = self.read_memory(self.program_pointer);
                let address = base.wrapping_add(self.register_x);
                return u16::from_le_bytes([address, address.wrapping_add(1)]);
            }
            opcode::AddressMode::IndirectY => {
                let base = self.read_memory(self.program_pointer);
                let address = u16::from_le_bytes([base, base.wrapping_add(1)]);
                return address.wrapping_add(self.register_y as u16);
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let code = self.read_memory(self.program_pointer);
            println!(
                "{:X}  {:X}     A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:X}",
                self.program_pointer,
                code,
                self.register_a,
                self.register_x,
                self.register_y,
                self.status,
                self.stack_pointer
            );
            self.program_pointer += 1;
            let last_program_pointer = self.program_pointer;

            let opcode = opcode::OPCODE_MAP
                .get(&code)
                .expect(&format!("Opcode:${:X} is not supported", code));

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

    pub fn sei(&mut self, _address_mode: &opcode::AddressMode) {
        self.status |= status_flags::INTER;
    }

    pub fn adc(&mut self, _address_mode: &opcode::AddressMode) {}

    pub fn jmp(&mut self, address_mode: &opcode::AddressMode) {
        self.program_pointer = self.read_memory_mode(address_mode);
    }

    pub fn lda(&mut self, address_mode: &opcode::AddressMode) {
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

    pub fn sta(&mut self, address_mode: &opcode::AddressMode) {
        let address = self.read_memory_mode(address_mode);
        self.write_memory(address, self.register_a);
    }

    pub fn cld(&mut self, _address_mode: &opcode::AddressMode) {
        self.status |= !status_flags::DEC;
    }

    pub fn ldx(&mut self, address_mode: &opcode::AddressMode) {
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

    pub fn txs(&mut self, address_mode: &opcode::AddressMode) {
        let data = self.read_memory_mode(address_mode);
        self.stack_push(data as u8);
    }

    pub fn jsr(&mut self, address_mode: &opcode::AddressMode) {
        let previous_address = self.program_pointer.wrapping_add(1);
        self.stack_push_u16(previous_address);
        self.program_pointer = self.read_memory_mode(address_mode);
    }

    pub fn tax(&mut self, _address_mode: &opcode::AddressMode) {
        let value = self.register_a;
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
        self.register_x = value;
    }

    pub fn brk(&mut self, _address_mode: &opcode::AddressMode) {}
}
