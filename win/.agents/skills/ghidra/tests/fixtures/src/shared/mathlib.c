/* Shared library fixture: exported symbols consumed by an executable. */
#include "mathlib.h"

int lib_add(int a, int b) {
    return a + b;
}

int lib_scale(int a, int factor) {
    return a * factor;
}
