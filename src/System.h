/*
 * System.h
 *
 * Created on 2023/02/01 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once

#include <iostream>
#include <memory>
#include <string>

#include "Cpu.h"
#include "MemoryBus.h"

class System
{
    enum class Error
    {
        None,
        InvalidRom
    };

public:
    System();
    ~System();

    bool load_rom(const std::string &rom_path);
    void run();

private:
    nes::MemoryBus _memory_bus;
    nes::Cpu _cpu;
};
