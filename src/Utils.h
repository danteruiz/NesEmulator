
/*
 * Utils.h
 *
 * Created on 2023/01/02 by Dante Ruiz
 * Copyright 2023 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Common.h"

bool sync_read_file(const char *file_path, u8 *buffer, size_t buffer_size, size_t &bytes_read);
