all: run-avx2

clean: run-avx2
	-rm -vf $^

%.o: %.S
	$(CXX) -o $@ $< -c $(CXXFLAGS) -Wa,-mrelax-relocations=yes

%.o: %.cxx
	$(CXX) -std=c++11 -o $@ $< -c $(CXXFLAGS)

run-avx2: sha256-avx2-asm.o sha256-avx2.o main.o
	$(CXX) -o $@ $^ $(CXXFLAGS)

libsha256fast.a: sha256-avx2-asm.o sha256-avx2.o
	$(AR) rc $@ $^