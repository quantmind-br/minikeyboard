/* Consumer executable linking the shared mathlib for import/export testing. */
#include <stdio.h>
#include "mathlib.h"

int main(void) {
    int s = lib_add(20, 22);
    int p = lib_scale(s, 2);
    printf("consumer: %d %d\n", s, p);
    return p & 0x7f;
}
