/*
 * Clock.h
 *
 * Created on 2023/02/01 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */
#pragma once
#include <chrono>

class Clock
{
public:
    Clock();
    ~Clock() = default;
    float delta_time();

private:
    std::chrono::high_resolution_clock::time_point _start;
};
