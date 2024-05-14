sfetch - **s***uckless* **fetch***ing utility*
------------
sfetch is a suckless utility that aims to beat neofetch and its derivatives in terms of speed, and power. If you know C, you'll know how to make this utility yours! This is a source-based, no-binary, fetching utility that sucks less amongst neofetch and all of its derivatives. Fastfetch is written in the suckless programming language which is C. However, it is written in non-pure C, it contains C++, and Objective-C, it also has over 800+ lines of code and that is fastfetch.c alone. Our suckless fetching utility aims to be lightweight so it can beat any thing in terms of speed and power. It has only less than 90 lines of code.

Requirements
------------
In order to build suckless fetch or sfetch, make sure to install rust or rustup and run rustup default stable to avoid any errors.


Installation
------------
Edit config.mk to match your local setup (sfetch is installed into
the /usr/local/bin namespace by default).

Afterwards enter the following command to build and install st (if
necessary as root):

    make clean install


Running sfetch
----------
Use /usr/local/bin/sfetch or use the $PATH variable: sfetch.

Credits
-------
Based on anonyneon's / arch-based on github's <anonyneon4@gmail.com> fetcher-rs source code just suck less.
README based on suckless terminal's README.md.
