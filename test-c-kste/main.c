// https://github.com/kste/sha256_avx
// 124983 (8x=12617)

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <sys/timeb.h>
#include <math.h>
#include "sha256avx.h"
#include "sha256.h"

#define DATA_SIZE	(1024 * 1024)
#define LOOPS_COUNT	(16 * 1024)
#define SHA256_DIGEST_LENGTH 32

int main()
{
    unsigned char data[DATA_SIZE];
    unsigned char hash[SHA256_DIGEST_LENGTH];
    struct timeb start, end;

    ftime(&start);
    for (long i = 0; i < (long)LOOPS_COUNT * (long)(DATA_SIZE / 64); i++)
    {
        sha256(hash, data);
    }
    ftime(&end);
    printf("sha256 %d \n", (int) (1000.0 * (end.time - start.time) + (end.millitm - start.millitm)));

    /*ftime(&start);
    for (long i = 0; i < (long)LOOPS_COUNT * (long)(DATA_SIZE / 64 / 8); i++)
    {
        sha256_8x(hash, data);
    }
    ftime(&end);
    printf("sha256_8x %d \n", (int) (1000.0 * (end.time - start.time) + (end.millitm - start.millitm)));*/

    return 0;
}