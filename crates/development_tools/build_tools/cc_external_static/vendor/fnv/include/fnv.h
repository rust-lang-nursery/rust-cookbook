#ifndef FNV_H
#define FNV_H

#include <stddef.h>
#include <stdint.h>

/* 64-bit FNV-1a hash of `len` bytes starting at `data`. */
uint64_t fnv1a_64(const uint8_t *data, size_t len);

#endif
