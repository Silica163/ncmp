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

%.o:%.c
	gcc -g -fPIC -c $< -o $@
