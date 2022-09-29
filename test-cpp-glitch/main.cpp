// https://github.com/System-Glitch/SHA256
// 317837

#include <iostream>
#include <chrono>
#include <ctime>
#include "SHA256.h"

#define DATA_SIZE	(1024 * 1024)
#define LOOPS_COUNT	(16 * 1024)
#define SHA256_DIGEST_LENGTH 32

int main(int argc, char ** argv)
{
    unsigned char data[DATA_SIZE];
    //unsigned char hash[SHA256_DIGEST_LENGTH];

    uint8_t * digest;
    SHA256 sha;

    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < LOOPS_COUNT; i++)
    {
        sha.update(data, DATA_SIZE);
        digest = sha.digest();
	//std::cout << SHA256::toString(digest) << std::endl;
	delete[] digest;
    }
    std::cout << "Test: "
			<< std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::high_resolution_clock::now() - start).count()
			<< std::endl;

  return EXIT_SUCCESS;
}
