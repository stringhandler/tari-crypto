#include "tari_crypto.h"
#include <stdio.h>

int main() {
    const char *ver = version();
    printf("Tari Crypto (v%s)\n", ver);
    return 0;
}