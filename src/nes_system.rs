use crate::nes_cpu;
use crate::nes_rom;
pub struct Clock {
    pub timer: u32,
}

enum SystemError {
    None,
    InvalidRom,
}

pub struct System {
    cpu: nes_cpu::Cpu,
    system_error: SystemError,
}

impl System {
    pub fn new() -> System {
        let system = System {
            cpu: nes_cpu::create_cpu(),
            system_error: SystemError::None,
        };
        return system;
    }

    pub fn load_rom(&mut self, file_path: String) {
        let rom_result = nes_rom::create_rom(file_path);
        match rom_result {
            Some(rom) => {
                nes_cpu::load_rom(&mut self.cpu, rom);
            }
            None => {
                self.system_error = SystemError::InvalidRom;
            }
        }
    }
}
