// https://github.com/ActualIlluminati/sha256fast
// 42631

#include <iostream>
#include <chrono>
#include <ctime>
#include "sha.hxx"

#define DATA_SIZE	(1024 * 1024)
#define LOOPS_COUNT	(16 * 1024)
#define SHA256_DIGEST_LENGTH 32

int main(int argc, char ** argv)
{
    unsigned char data[DATA_SIZE];
    unsigned char hash[SHA256_DIGEST_LENGTH];

    struct sha256_state st;

    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < LOOPS_COUNT; i++)
    {
        sha256_init(&st);
        sha256_update(&st, data, DATA_SIZE);
        sha256_final(&st, hash);
    }
    std::cout << "Test: "
			<< std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::high_resolution_clock::now() - start).count()
			<< std::endl;

  return EXIT_SUCCESS;
}