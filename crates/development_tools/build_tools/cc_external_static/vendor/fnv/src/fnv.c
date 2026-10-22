#include "fnv.h"

#define FNV_OFFSET_BASIS 0xcbf29ce484222325ULL
#define FNV_PRIME 0x100000001b3ULL

uint64_t fnv1a_64(const uint8_t *data, size_t len) {
    uint64_t hash = FNV_OFFSET_BASIS;
    for (size_t i = 0; i < len; i++) {
        hash ^= data[i];
        hash *= FNV_PRIME;
    }
    return hash;
}
