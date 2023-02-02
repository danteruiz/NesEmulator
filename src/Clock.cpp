/*
 * Clock.cpp
 *
 * Created on 2023/02/01 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Clock.h"

#include <iostream>

namespace
{
[[maybe_unused]] constexpr float TO_MILISECOND = 1000.0f;
}

using MS = std::chrono::duration<float, std::milli>;
Clock::Clock() : _start(std::chrono::high_resolution_clock::now()) {}

float Clock::delta_time()
{
    auto now = std::chrono::high_resolution_clock::now();
    auto diff = now - _start;
    _start = now;

    return std::chrono::duration_cast<MS>(diff).count();
}
