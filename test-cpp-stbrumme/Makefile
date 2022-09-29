CC=g++
RM=rm -f
CPPFLAGS=-O3 -mavx2 -g -funroll-all-loops -Wall -std=c++11
LDFLAGS=-g
LDLIBS=
SRCS=sha256.cpp main.cpp
OBJS=$(subst .cpp,.o,$(SRCS))
OUTFILE="main"

main: $(OBJS)
	$(CC) $(LDFLAGS) -o $(OUTFILE) $(OBJS) $(LDLIBS) 

main.o: 
	$(CC) $(CPPFLAGS) -c main.cpp

sha256.o: 
	$(CC) $(CPPFLAGS) -c sha256.cpp

clean:
	$(RM) $(OBJS)



