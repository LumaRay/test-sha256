// https://github.com/okdshin/PicoSHA2
// 149811

#include <iostream>
#include <chrono>
#include <ctime>
#include "picosha2.h"

#define DATA_SIZE	(1024 * 1024)
#define LOOPS_COUNT	(16 * 1024)
#define SHA256_DIGEST_LENGTH 32

int main(int argc, char ** argv)
{
    unsigned char data[DATA_SIZE];
    //unsigned char hash[SHA256_DIGEST_LENGTH];

    std::vector<unsigned char> hash(picosha2::k_digest_size);
    std::vector<unsigned char> src_vect(data, data + DATA_SIZE);

    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < LOOPS_COUNT; i++)
    {
        picosha2::hash256(src_vect.begin(), src_vect.end(), hash.begin(), hash.end());
    }
    std::cout << "Test: "
			<< std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::high_resolution_clock::now() - start).count()
			<< std::endl;

  return EXIT_SUCCESS;
}
