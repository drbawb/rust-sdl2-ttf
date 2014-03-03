RUSTC=rustc
RUSTFLAGS=--out-dir build
LDFLAGS=-L lib
CRATE_ROOT=src/lib.rs

.PHONY : all clean doc 

all: clean compile

compile:
	mkdir -p build
	$(RUSTC) $(RUSTFLAGS) $(LDFLAGS) $(CRATE_ROOT)

veyron: RUSTFLAGS += -O -Z time-passes
veyron: all

debug: RUSTFLAGS += -g -Z time-passes
debug: compile

test: RUSTFLAGS += --test
test: compile
	build/sdl2_ttf

deps:	
	git submodule update --init	
	mkdir -p lib	
	rm -f lib/libsdl2*	
	cd lib/rust-sdl2; make
	cp lib/rust-sdl2/build/lib/libsdl2* lib/

doc:
	rustdoc $(LDFLAGS) $(CRATE_ROOT)

clean:
	rm -f bin/**
