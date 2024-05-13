# sfetch - suckless fetching utility

.POSIX:

include config.mk

all: sfetch
	
sfetch: $(SRC)
	$(CC) build --release

install: sfetch
	cp $(TARGETDIR)/sfetch $(DESTDIR)

clean:
	rm -rf $(TARGETDIR)/sfetch

uninstall:
	rm $(DESTDIR)/sfetch
