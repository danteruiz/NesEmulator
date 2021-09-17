#[macro_use]
extern crate lazy_static;

mod nes;

fn main() {
    let clock = nes::system::Clock { timer: 20 };
    println!("timer on the clock {}", clock.timer);

    let mut system = nes::system::System::new();
    let file_path = "test-roms/other/nestest.nes".to_string();
    system.load_rom(file_path);
    system.run();
}
