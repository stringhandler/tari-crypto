#include "tari_crypto.h"
#include <stdio.h>

void print_key(uint8_t key[]) {
  int i;
  for (i = 0; i < KEY_LENGTH; i++) {
    printf("%02X", key[i]);
  }
  printf("\n");
}

/*
 * This demo generates a key pair, signs a message and then validates the signature.
 * All memory in this FFI is managed by the caller. In this demo, the data is kept on the stack, and so explicit
 * memory management is not done, but in general, you have to allocate and free memory yourself.
 */
int main() {
    const char *ver = version();
    printf("Tari Crypto (v%s)\n", ver);

    uint8_t pub_key[KEY_LENGTH], priv_key[KEY_LENGTH];

    random_keypair(&priv_key, &pub_key);
    printf("Keys generated\n");
    print_key(priv_key);
    print_key(pub_key);

    // Sign and verify message
    const char msg[] = "Hello world";

    uint8_t r[KEY_LENGTH], sig[KEY_LENGTH];

    sign(&priv_key, &msg, &r, &sig);

    printf("Signed message\n");
    print_key(r);
    print_key(sig);

    printf("Check signature: ");
    if (verify(&pub_key, &msg, &r, &sig)) {
        printf("SUCCESS\n");
    } else {
        printf("FAILED\n");
    }
    return 0;
}

