// https://github.com/amosnier/sha-2
// 94601

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/timeb.h>

#include "sha-256.h"

#define DATA_SIZE	(1024 * 1024)
#define LOOPS_COUNT	(16 * 1024)
#define SHA256_DIGEST_LENGTH 32

int main()
{
    unsigned char data[DATA_SIZE];
    unsigned char hash[SHA256_DIGEST_LENGTH];
    struct timeb start, end;

    ftime(&start);
    for (int i = 0; i < LOOPS_COUNT; i++)
    {
	calc_sha_256(hash, data, sizeof(data));
    }
    ftime(&end);
    printf("Test %d \n", (int) (1000.0 * (end.time - start.time) + (end.millitm - start.millitm)));

    return 0;
}