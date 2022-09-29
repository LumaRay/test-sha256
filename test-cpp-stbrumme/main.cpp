// https://github.com/stbrumme/hash-library
// 72729
// SHA2 test program
#include "sha256.h"
#include <iostream> // for std::cout only, not needed for hashing library
#include <chrono>

#define DATA_SIZE	(1024 * 1024)
#define LOOPS_COUNT	(16 * 1024)
#define SHA256_DIGEST_LENGTH 32

int main(int, char**)
{
  // create a new hashing object
  SHA256 sha256;
    unsigned char data[DATA_SIZE];
    unsigned char hash[SHA256_DIGEST_LENGTH];

  // hashing an std::string
  //std::cout << sha256("Hello World") << std::endl;
  // => a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e

  // hashing a buffer of bytes
  //const char* buffer = "How are you";
  //std::cout << sha256(buffer, 11) << std::endl;
  // => 9c7d5b046878838da72e40ceb3179580958df544b240869b80d0275cc07209cc

  // or in a streaming fashion (re-use "How are you")
  /*SHA256 sha256stream;
  const char* url = "create.stephan-brumme.com"; // 25 bytes
  int step = 5;
  for (int i = 0; i < 25; i += step)
    sha256stream.add(url + i, step); // add five bytes at a time
  std::cout << sha256stream.getHash() << std::endl;*/
  // => 82aa771f1183c52f973c798c9243a1c73833ea40961c73e55e12430ec77b69f6

    auto start = std::chrono::high_resolution_clock::now();
    //for (*((int*)data) = 0; *((int*)data) < LOOPS_COUNT; (*((int*)data))++)
    for (int i = 0; i < LOOPS_COUNT; i++)
    {
        sha256.add(data, sizeof(data));
        sha256.getHash(hash);
        sha256.reset();
    }
    std::cout << "Test: "
			<< std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::high_resolution_clock::now() - start).count()
			<< std::endl;

  return 0;
}
