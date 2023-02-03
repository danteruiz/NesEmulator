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
#include "System.h"

#include <iostream>
#include <string>

int main()
{

    System system;
    system.load_rom("/home/dante/code/NesEmulator/test-roms/other/nestest.nes");
    return 0;
}
