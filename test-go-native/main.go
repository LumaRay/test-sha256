// export PATH=$PATH:/usr/local/go/bin
// 42559
package main

import (
	"crypto/sha256"
	"fmt"
	"time"
)

const DATA_SIZE = (1024 * 1024)
const LOOPS_COUNT = (16 * 1024)

func main() {
	block := make([]byte, DATA_SIZE)
	//sum := sha256.Sum256([]byte("hello world\n"))

	sum := sha256.Sum256(block)

	start := time.Now()
	for i := 0; i < LOOPS_COUNT; i++ {
		sum = sha256.Sum256(block)
	}
	fmt.Println("sha256: ", time.Since(start).Milliseconds())
	fmt.Printf("%x", sum)
}
