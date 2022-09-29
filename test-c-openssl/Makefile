C=gcc
RM=rm -f
CFLAGS=-O3 -mavx2 -g -funroll-all-loops -Wall -I/opt/ssl/include/ -L/opt/ssl/lib/
LDLIBS = -lssl -lcrypto
SRCS=main.c
OBJS=$(subst .c,.o,$(SRCS))
OUTFILE="main"

all: main

sha256: $(OBJS)
	$(C) -o $(OUTFILE) $(OBJS) $(LDLIBS) 

depend: .depend

.depend: $(SRCS)
	rm -f ./.depend
	$(C) $(CFLAGS) -MM $^>>./.depend;

clean:
	$(RM) $(OBJS)

dist-clean: clean
	$(RM) *~ .depend

include .depend

