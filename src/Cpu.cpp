/*
 * Cpu.cpp
 *
 * Created on 2023/02/01 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Cpu.h"

namespace nes
{
Cpu::Cpu(MemoryBus &memory_bus) : _memory_bus(memory_bus) {}
} // namespace nes
