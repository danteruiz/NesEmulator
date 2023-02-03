/*
 * Cpu.h
 *
 * Created on 2023/02/01 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Common.h"
#include "MemoryBus.h"

namespace nes
{

enum StatusFlags
{
    Carry = 1 << 0,
    Zero = 1 << 1,
    Inter = 1 << 2,
    Dec = 1 << 3,
    Overflow = 1 << 4,
    Negative = 1 << 5
};

constexpr u16 STACK_ADDRESS = 0x0100;
constexpr u8 STACK_RESET = 0xFF;

class Cpu
{
public:
    Cpu(MemoryBus &memory_bus);
    ~Cpu() = default;

private:
    MemoryBus _memory_bus;
};
} // namespace nes
