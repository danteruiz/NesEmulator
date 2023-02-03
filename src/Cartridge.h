/*
 * Cartridge.h
 *
 * Created on 2023/02/01 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */
#pragma once

#include "Common.h"

#include <string>
#include <vector>

namespace nes
{
class Cartridge
{
public:
    Cartridge();
    ~Cartridge();

    bool load_from_file(const std::string &file);

private:
    std::vector<Byte> _prog_rom;
    std::vector<Byte> _char_rom;
};
} // namespace nes
