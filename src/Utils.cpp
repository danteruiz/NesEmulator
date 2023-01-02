#include "Utils.h"

#include <fstream>

bool sync_read_file(const char* file_path, u8* buffer, size_t buffer_size, size_t& bytes_read) {
    std::fstream file_handle(file_path);
    file_handle.seekg(0, std::ios_base::end);
    const u32 file_length = file_handle.tellg();
}
