// https://github.com/minio/sha256-simd
// https://www.howtoinstall.me/ubuntu/18-04/golang-github-minio-sha256-simd-dev/
// export PATH=$PATH:/usr/local/go/bin
// go get github.com/minio/sha256-simd
// 42920
package main

import (
	"fmt"
	"github.com/minio/sha256-simd"
	"time"
)

const DATA_SIZE = (1024 * 1024)
const LOOPS_COUNT = (16 * 1024)

func main() {
	/*server := sha256.NewAvx512Server()
	h512 := sha256.NewAvx512(server)
	h512.Write(block)
	digest := h512.Sum([]byte{})
	fmt.Printf("digest:\t %x \n", []byte{digest[0]})//digest)*/

	block := make([]byte, DATA_SIZE)

	h := sha256.New()

	digest := h.Sum([]byte{})

	start := time.Now()
	for i := 0; i < LOOPS_COUNT; i++ {
		h.Write(block)
		digest = h.Sum([]byte{})
		h.Reset()
	}
	fmt.Println("sha256: ", time.Since(start).Milliseconds())
	fmt.Printf("%x", digest)
}
