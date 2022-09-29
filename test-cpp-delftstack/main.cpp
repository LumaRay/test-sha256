// https://www.delftstack.com/howto/cpp/sha256-cpp/
// 71239

#include <iostream> // for std::cout only, not needed for hashing library
#include <chrono>
#include "sha256.h"

#define DATA_SIZE	(1024 * 1024)
#define LOOPS_COUNT	(16 * 1024)
#define SHA256_DIGEST_LENGTH 32

int main(int, char**)
{
    unsigned char data[DATA_SIZE];
    unsigned char hash[SHA256_DIGEST_LENGTH];

    hash_functions obj = hash_functions();

    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < LOOPS_COUNT; i++)
    {
        obj.stateregister();
        obj.adjust_digest(data, DATA_SIZE);
        obj.digest_final(hash);
    }
    std::cout << "Test: "
			<< std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::high_resolution_clock::now() - start).count()
			<< std::endl;

  return 0;
}
