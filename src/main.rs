#[macro_use]
extern crate lazy_static;
mod nes_cpu;
mod nes_rom;
mod nes_system;

fn main() {
    let clock = nes_system::Clock { timer: 20 };
    println!("timer on the clock {}", clock.timer);

    let mut system = nes_system::System::new();

    let file_path =
        "test-roms/nes_instr_test/rom_singles/02-immediate.nes".to_string();
    system.load_rom(file_path);

    system.run();
}
