SRC := $(wildcard src/bin/step*.rs)
BIN := $(patsubst src/bin/step%.rs, target/debug/step%, $(src))
STEP := $(patsubst src/bin/step%.rs, %, $(SRC))

.PHONY: all $(STEP)
all: build $(STEP)

step%: build
	cp target/debug/$@ $@

.PHONY: build clean
build:
	cargo build

clean:
	cargo clean
	rm -f $(STEP)
