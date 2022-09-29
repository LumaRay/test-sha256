C=gcc
RM=rm -f
CFLAGS=-O3 -mavx2 -g -funroll-all-loops -Wall
LDLIBS=
SRCS=main.c sha256.c sha256avx.c
OBJS=$(subst .c,.o,$(SRCS))
OUTFILE="main"

main: $(OBJS)
	$(C) -o $(OUTFILE) $(OBJS) $(LDLIBS) 

clean:
	$(RM) $(OBJS)
