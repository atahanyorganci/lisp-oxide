BIN := $(wildcard src/bin/step*.rs)
STEP := $(patsubst src/bin/step%.rs, step%, $(BIN))

.PHONY: all $(STEP) clean
all: $(STEP)

$(STEP):
	cargo build --bin $@
	cp target/debug/$@ $@

clean:
	cargo clean
	rm -f $(STEP) step*.log
