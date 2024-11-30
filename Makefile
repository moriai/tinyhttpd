TARGETS = README.html

.PHONY: all build clean

all: build $(TARGETS)

build:
	cargo build --release
	cargo doc

README.html: README.md
	pandoc $^ -o $@

clean:
	cargo clean
	rm -fr $(TARGETS)
