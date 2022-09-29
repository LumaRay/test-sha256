// https://github.com/ckolivas/cgminer
// 70336
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <sys/timeb.h>
#include <math.h>
#include "sha2.h"

#define DATA_SIZE	(1024 * 1024)
#define LOOPS_COUNT	(16 * 1024)
#define SHA256_DIGEST_LENGTH 32

int main()
{
    unsigned char data[DATA_SIZE];
    unsigned char hash[SHA256_DIGEST_LENGTH];
    struct timeb start, end;

    ftime(&start);
    //for (*((int*)data) = 0; *((int*)data) < LOOPS_COUNT; (*((int*)data))++)
    for (int i = 0; i < LOOPS_COUNT; i++)
    {
        sha256(data, sizeof(data), hash);
    }
    ftime(&end);
    printf("Test %d \n", (int) (1000.0 * (end.time - start.time) + (end.millitm - start.millitm)));
    //print_hex(hash, sizeof(hash));
    return 0;
}