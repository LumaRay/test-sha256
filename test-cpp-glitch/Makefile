CC=g++
RM=rm -f
CPPFLAGS=-Ofast -mavx2 -g -funroll-all-loops -Wall -Wextra -fPIC -std=c++17
#  -std=c++11
LDFLAGS=-g
LDLIBS=
SRCS=SHA256.cpp main.cpp
OBJS=$(subst .cpp,.o,$(SRCS))
OUTFILE="main"

main: $(OBJS)
	$(CC) $(LDFLAGS) -o $(OUTFILE) $(OBJS) $(LDLIBS) 

main.o: 
	$(CC) $(CPPFLAGS) -c main.cpp

SHA256.o: 
	$(CC) $(CPPFLAGS) -c SHA256.cpp

clean:
	$(RM) $(OBJS)



