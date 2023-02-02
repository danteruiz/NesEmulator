/*
 * Utils.cpp
 *
 * Created on 2023/01/02 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Utils.h"

#include <fstream>

bool sync_read_file(const char *file_path, [[maybe_unused]] u8 *buffer,
                    [[maybe_unused]] size_t buffer_size, [[maybe_unused]] size_t &bytes_read)
{
    std::fstream file_handle(file_path);
    file_handle.seekg(0, std::ios_base::end);
    const u32 file_length = static_cast<u32>(file_handle.tellg());
    bytes_read = file_length;

    return false;
}
