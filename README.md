# Testing SHA256 vs Blake3 performance with different languages

The testing has been performed on Windows 7x64 host with Ubuntu 18.04 x64 virtual machine with 16Gb RAM and 4 cores, Intel-VT enabled.

Host:
- CPU: Intel Core i7-4790 3.6 GHz 4 cores / 8 threads
- RAM: 32 Gb

AVX2 is used whenever possible.

Full release optimizations.

SHA256/Blake3 are computed over 1MB byte array with 16 * 1024 cycles.

All tests use 1 core.

Test results are given in milliseconds of total execution time, in descending order, worst to best.

You can find links to the algorithms' web pages in the first comments of the corresponding main source files.

## Test Results with SHA-256

| Language | Author  | Time, ms  |
|:-------:|:---------:|:---------:|
|    cpp | glitch   | 317837 |
|    cpp | picosha2   | 149811 |
|    c | kste   |  124983 (8x=12617) |
|    c | amosnier   | 94601 |
|    rust | alexv   |  88485 |
|    rust | rustcrypto   |  80306 |
|    cpp | stbrumme   |  72729 |
|    c | defltstack   |  71239 |
|    c | cgminer   |  70336 |
|    rust | jedisct1   |  68292 |
|    go | minio   |  42920 |
|    cpp | actualilluminati   |  42631 |
|    go | native   |  42559 |
|    rust | ring   |  42541 |
|    rust | openssl   |  37666 |
|    c | openssl   |  36693 |

## Test Results with Blake3

| Language | Author  | Time, ms  |
|:-------:|:-------------:|:---------:|
|    rust |  BLAKE3-team   | 4692 |

There is also some interesting benchmark chart: https://bench.cr.yp.to/results-hash.html
