/*
 * System.cpp
 *
 * Created on 2023/02/01 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "System.h"

#include "Cartridge.h"
System::System() : _cpu(_memory_bus) {}

System::~System() {}

bool System::load_rom(const std::string &path)
{

    nes::Cartridge cartridge;

    return cartridge.load_from_file(path);
}
