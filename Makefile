RSS=\
	player.rs \
	ma_wrapper.rs \
	playlist.rs

OBJS=\
	 ma_wrapper.o \
	 miniaudio.o

all: main

run: main
	./main

main: $(RSS) $(OBJS) main.rs
	rustc -g -C link-args="$(OBJS) -lm -lpthread" main.rs -o main 

ma_wrapper.o: ma_wrapper.c
	gcc -ggdb -fPIC -c ma_wrapper.c -o ma_wrapper.o 

miniaudio.o: miniaudio.h
	gcc -ggdb -fPIC -x c -c miniaudio.h -o miniaudio.o -DMINIAUDIO_IMPLEMENTATION -D_MA_DEBUG_OUTPUT
