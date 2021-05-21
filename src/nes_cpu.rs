use crate::nes_rom;

const MAX_CPU_MEMORY: usize = 0x10000;
pub struct Cpu {
    pub program_pointer: u16,
    pub stack_pointer: u8,
    pub processor_status: u8,
    register_a: u8,
    register_x: u8,
    register_z: u8,
    memory: [u8; MAX_CPU_MEMORY],
}

pub fn create_cpu() -> Cpu {
    let cpu = Cpu {
        program_pointer: 0,
        stack_pointer: 0,
        processor_status: 0,
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
