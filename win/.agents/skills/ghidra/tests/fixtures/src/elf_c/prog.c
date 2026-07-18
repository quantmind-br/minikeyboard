/* Clean-room ELF C fixture: main -> bridge -> hot, with a branch and strings. */
#include <stdio.h>
#include <string.h>

__attribute__((noinline))
int hot(int x) {
    /* branch so the decompiler recovers control flow */
    if (x > 10) {
        return x * 3 + 1;
    }
    return x + 7;
}

__attribute__((noinline))
int bridge(int x) {
    int y = hot(x);
    return y ^ 0x5a;
}

int main(int argc, char **argv) {
    const char *tag = "fixture-elf-c";
    int seed = argc > 1 ? (int)strlen(argv[1]) : 3;
    int r = bridge(seed);
    printf("%s: %d\n", tag, r);
    return r & 0x7f;
}
