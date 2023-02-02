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

#include <memory>

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

    bool load_rom();
    void run();
};
