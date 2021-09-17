// rom.rs
//
// Created on 2021/09/16 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::fs;
use std::io::prelude::*;

pub struct Rom {
    pub prog_rom: Vec<u8>,
    pub char_rom: Vec<u8>,
}

fn get_raw_data(file_path: String) -> Option<Vec<u8>> {
    let file = fs::File::open(file_path);

    let mut file = match file {
        Ok(f) => f,
        Err(error) => {
            println!("Failed to load file: {}", error);
            return None;
        }
    };

    let mut data: Vec<u8> = Vec::new();
    let result = file.read_to_end(&mut data);

    if result.is_err() {
        return None;
    }

    return Some(data);
}

static NES_TAG: &'static [u8] = &[0x4E, 0x45, 0x53, 0x1A];
static ROM_PRG_SIZE: usize = 16384;
static ROM_CHAR_SIZE: usize = 8192;
static ROM_TRAINER_SIZE: usize = 512;

pub fn create_rom(file_path: String) -> Option<Rom> {
    let raw_rom: Option<Vec<u8>> = get_raw_data(file_path);

    let bytes = match raw_rom {
        Some(data) => data,
        None => {
            return None;
        }
    };

    //check the header of the rom.
    let nes_tag = &bytes[0..=3];

    if nes_tag != NES_TAG {
        return None;
    }

    let rom_prg_len = bytes[4] as usize * ROM_PRG_SIZE;
    let rom_chr_len = bytes[5] as usize * ROM_CHAR_SIZE;

    let rom_header_f6 = bytes[6];

    let trainer_offset = if rom_header_f6 & 0b100 != 0 {
        ROM_TRAINER_SIZE
    } else {
        0
    };

    let prog_offset = 16 + trainer_offset;
    let char_offset = prog_offset + rom_prg_len;

    let rom: Rom = Rom {
        prog_rom: bytes[prog_offset..(prog_offset + rom_prg_len)].to_vec(),
        char_rom: bytes[char_offset..(char_offset + rom_chr_len)].to_vec(),
    };

    for index in 0..rom.prog_rom.len() - 1 {
        if rom.prog_rom[index] == 0xF5 && rom.prog_rom[index + 1] == 0xC5 {
            println!("{:X} {:X}", index, rom.prog_rom[index - 1]);
        }
    }

    println!("done");

    return Some(rom);
}
