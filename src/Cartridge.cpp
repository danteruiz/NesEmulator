/*
 * Cartridge.cpp
 *
 * Created on 2023/02/01 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Cartridge.h"

#include <fstream>
#include <ios>
#include <iostream>
#include <vector>

static const std::size_t HEADER_SIZE = 16;
static const std::string NES_TAG = {0x4E, 0x45, 0x53, 0x1A};

constexpr std::size_t PRG_SIZE = 16384;
constexpr std::size_t CHR_SIZE = 8192;
namespace nes
{

Cartridge::Cartridge() {}
Cartridge::~Cartridge() {}

bool Cartridge::load_from_file(const std::string &path)
{
    std::ifstream ustream(path, std::ios_base::binary | std::ios_base::in);

    if (!ustream)
    {
        std::cout << "Failed to open file: " << path << std::endl;
        return false;
    }
    u8 header[HEADER_SIZE] = {0};

    std::cout << "header size: " << HEADER_SIZE << std::endl;
    if (!ustream.read(reinterpret_cast<char *>(&header[0]), HEADER_SIZE))
    {
        std::cout << "failed to read header" << std::endl;
        return false;
    }

    std::string nes_tag = std::string(&header[0], &header[4]);
    if (nes_tag != NES_TAG)
    {
        std::cout << "header does not match" << std::endl;
        return false;
    }

    auto rom_prg_len = static_cast<std::size_t>(header[4]) * PRG_SIZE;
    auto rom_chr_len = static_cast<std::size_t>(header[5]) * CHR_SIZE;

    return true;
}

} // namespace nes
