// https://stackoverflow.com/questions/2262386/generate-sha256-with-openssl-and-c
// sudo apt-get install openssl libssl-dev
// 36809

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <sys/timeb.h>
#include <math.h>
#include "openssl/sha.h"

#define DATA_SIZE	(1024 * 1024)
#define LOOPS_COUNT	(16 * 1024)

/*void sha256_hash_string (unsigned char hash[SHA256_DIGEST_LENGTH], char outputBuffer[65])
{
    int i = 0;

    for(i = 0; i < SHA256_DIGEST_LENGTH; i++)
    {
        sprintf(outputBuffer + (i * 2), "%02x", hash[i]);
    }

    outputBuffer[64] = 0;
}*/

void sha256_bench(SHA256_CTX* ctx, unsigned char *in, int in_len, unsigned char *out)
{
    SHA256_Init(ctx);
    SHA256_Update(ctx, in, in_len);
    SHA256_Final(out, ctx);
}

void print_hex(unsigned char* buf, size_t buf_len)
{
    for (int i = 0; i < buf_len; i++)
    {
        printf("%02x", buf[i]);
    }
    printf("\n");
}

/*void sha256_string(char *string, char outputBuffer[65])
{
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, string, strlen(string));
    SHA256_Final(hash, &sha256);
    int i = 0;
    for(i = 0; i < SHA256_DIGEST_LENGTH; i++)
    {
        sprintf(outputBuffer + (i * 2), "%02x", hash[i]);
    }
    outputBuffer[64] = 0;
}

int sha256_file(char *path, char outputBuffer[65])
{
    FILE *file = fopen(path, "rb");
    if(!file) return -534;

    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    const int bufSize = 32768;
    unsigned char *buffer = malloc(bufSize);
    int bytesRead = 0;
    if(!buffer) return 12; // Out of Memory
    while((bytesRead = fread(buffer, 1, bufSize, file)))
    {
        SHA256_Update(&sha256, buffer, bytesRead);
    }
    SHA256_Final(hash, &sha256);

    sha256_hash_string(hash, outputBuffer);
    fclose(file);
    free(buffer);
    return 0;
}*/

int main()
{
    SHA256_CTX sha256;
    unsigned char data[DATA_SIZE];
    unsigned char hash[SHA256_DIGEST_LENGTH];
    struct timeb start, end;

    ftime(&start);
    //for (*((int*)data) = 0; *((int*)data) < LOOPS_COUNT; (*((int*)data))++)
    for (int i = 0; i < LOOPS_COUNT; i++)
    {
        sha256_bench(&sha256, data, sizeof(data), hash);
    }
    ftime(&end);
    printf("Test %d \n", (int) (1000.0 * (end.time - start.time) + (end.millitm - start.millitm)));
    //print_hex(hash, sizeof(hash));
    return 0;
}