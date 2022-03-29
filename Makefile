all: install

install:
	cargo build --release
	sudo cp target/release/typrint /usr/bin/typrint

uninstall:
	sudo rm -f /usr/bin/typrint