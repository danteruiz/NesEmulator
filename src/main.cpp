/*
 * main.cpp
 *
 * Created on 2023/01/03 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Clock.h"
#include "Common.h"
#include "Utils.h"

#include <iostream>
#include <string>

int main()
{
    u8 data[512];
    size_t bytes_read = 0;
    sync_read_file("path to file", data, sizeof(data), bytes_read);

    Clock clock;
    while (true)
    {
        auto delta_time = clock.delta_time();
        std::cout << "delta time: " << delta_time << std::endl;
    }
    return 0;
}
