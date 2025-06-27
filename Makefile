all: main

run: main
	./main

main: Makefile main.rs ma_wrapper.rs ma_wrapper.o miniaudio.o
	rustc -g -C link-args="ma_wrapper.o miniaudio.o -lm -lpthread" main.rs -o main 

ma_wrapper.o: ma_wrapper.c
	gcc -ggdb -fPIC -c ma_wrapper.c -o ma_wrapper.o 

miniaudio.o: miniaudio.h
	gcc -ggdb -fPIC -x c -c miniaudio.h -o miniaudio.o -DMINIAUDIO_IMPLEMENTATION -D_MA_DEBUG_OUTPUT
