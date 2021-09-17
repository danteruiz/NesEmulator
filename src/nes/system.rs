// system.rs
//
// Created on 2021/09/16 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use crate::nes::cpu;
use crate::nes::rom;

pub struct Clock {
    pub timer: u32,
}

enum SystemError {
    None,
    InvalidRom,
}

pub struct System {
    cpu: cpu::Cpu,
    system_error: SystemError,
}

impl System {
    pub fn new() -> System {
        let system = System {
            cpu: cpu::create_cpu(),
            system_error: SystemError::None,
        };
        return system;
    }

    pub fn load_rom(&mut self, file_path: String) {
        let rom_result = rom::create_rom(file_path);
        match rom_result {
            Some(rom) => {
                cpu::load_rom(&mut self.cpu, rom);
            }
            None => {
                self.system_error = SystemError::InvalidRom;
            }
        }
    }

    pub fn run(&mut self) {
        self.cpu.reset();
        self.cpu.run();
    }
}
