BUILD=build
SRC=src
RSS=\
	$(SRC)/player.rs \
	$(SRC)/ma_wrapper.rs \
	$(SRC)/playlist.rs

OBJS=\
	 $(BUILD)/ma_wrapper.o \
	 $(BUILD)/miniaudio.o

all: main

main: $(RSS) $(OBJS) $(SRC)/main.rs | $(BUILD)
	rustc -g -C link-args="$(OBJS) -lm -lpthread" $(SRC)/main.rs -o ncmp

$(BUILD)/%.o:./clib/%.c | $(BUILD)
	gcc -g -fPIC -c $< -o $@

$(BUILD):
	mkdir -pv $(BUILD)
