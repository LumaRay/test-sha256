C=gcc
RM=rm -f
CFLAGS=-O3 -mavx2 -g -funroll-all-loops -Wall
LDLIBS=
SRCS=main.c sha-256.c
OBJS=$(subst .c,.o,$(SRCS))
OUTFILE="main"

main: $(OBJS)
	$(C) -o $(OUTFILE) $(OBJS) $(LDLIBS) 

clean:
	$(RM) $(OBJS)
