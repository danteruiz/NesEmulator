#include <iostream>

#include "Common.h"
#include "Utils.h"

int main() {
    u8 data[512];
    size_t bytes_read = 0;
    bool success = sync_read_file("path to file", &data, sizeof(data), bytes_read);
    return 0;
}
