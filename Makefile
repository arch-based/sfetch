# Name of your program
PROG = sfetch

# Path to your source file
SRC = src/main.rs

# Destination directory for the binary
DESTDIR = /usr/local/bin

# Compiler
CC = cargo

all: $(PROG)

$(PROG): $(SRC)
	$(CC) build --release

install: $(PROG)
	cp target/release/$(PROG) $(DESTDIR)

clean:
	rm -f target/release/$(PROG)

uninstall:
	rm /usr/local/bin/$(PROG)
