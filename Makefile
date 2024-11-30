TARGETS = README.html

.PHONY: all build run clean

all build: $(TARGETS)
	cargo doc
	cargo build --release

run: $(TARGETS)
	cargo doc
	cargo run --release

README.html: README.md
	pandoc $^ -o $@

clean:
	cargo clean
	rm -fr $(TARGETS)
