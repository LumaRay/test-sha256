// https://github.com/minio/sha256-simd
// https://www.howtoinstall.me/ubuntu/18-04/golang-github-minio-sha256-simd-dev/
// export PATH=$PATH:/usr/local/go/bin
// remove requirements from go.mod
// go install github.com/Luzifer/go-openssl@latest
// go get github.com/Luzifer/go-openssl/v4
// uses crypto (((
package main

import (
	"fmt"
	"github.com/Luzifer/go-openssl/v4"
	"time"
)


const DATA_SIZE = (1024 * 1024)
const LOOPS_COUNT = (16 * 1024)

func main() {
	block := make([]byte, DATA_SIZE)

        o := openssl.New()

	start := time.Now()
	for i := 0; i < LOOPS_COUNT; i++ {
		digest, err := openssl::DigestSHA256Sum(block)
	}
	fmt.Println("sha256: ", time.Since(start).Milliseconds())
	fmt.Printf("%x", digest)
}
