/*
 * MemoryBus.h
 *
 * Created on 2023/02/01 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */
#pragma once

#include "Common.h"

#include <array>

namespace nes
{

class MemoryBus
{
public:
    MemoryBus();
    ~MemoryBus();

    void write(Address address, Byte data);
    Byte read(Address address);

private:
    std::array<u8, 0xFFFF> _ram;
};

} // namespace nes
