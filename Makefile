prog :=sni-sniffer

debug ?=

ifdef debug
  release :=
  target :=debug
else
  release :=--release
  target :=release
endif

build:
	cargo build $(release)

install:
	cp target/$(target)/$(prog) ~/bin/$(prog)
	sudo cp target/$(target)/$(prog) /usr/sbin/$(prog)

all: build install

help:
	@echo "usage: make $(prog) [debug=1]"

