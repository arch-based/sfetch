# Name of your program
PROG = sfetch

# Path to your source file
SRC = src/main.rs

# Destination directory for the binary
DESTDIR = /usr/local/bin

# Compiler
CC = rustc

# Compiler flags
CFLAGS = --release

all: $(PROG)

$(PROG): $(SRC)
	$(CC) $(CFLAGS) $< -o $@

install: $(PROG)
	cp $(PROG) $(DESTDIR)

clean:
	rm -f $(PROG)

