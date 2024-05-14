# sfetch

The **s***uckless* **fetch***ing utility*, abbreviated as sfetch is a suckless utility that aims to beat neofetch and its derivatives in terms of speed, and power. If you know C, you'll know how to make this utility yours! This is a source-based, no-binary, fetching utility that sucks less amongst neofetch and all of its derivatives. Fastfetch is written in the suckless programming language which is C. However, it is written in non-pure C, it contains C++, and Objective-C, it also has over 800+ lines of code and that is fastfetch.c alone. Our suckless fetching utility aims to be lightweight so it can beat any thing in terms of speed and power. It has only less than 90 lines of code.

# Requirements

In order to build, and install sfetch, you may install any of the following compilers:
- GCC (GNU Compiler Collection)
- Clang (Computer Based LLVM)
- Intel (Intel *C++* Compiler)
- ...

In order to change the compiler being used on the Makefile, change it through the config.mk file.

# Installation

Edit config.mk to match your local setup (sfetch is installed into
the /usr/local/bin namespace by default).

Afterwards enter the following command to build and install sfetch (if
necessary as root):

    make clean install


# Usage

Use /usr/local/bin/sfetch or use the $PATH variable: sfetch.

# Credits 
Based on my fetcher-cpp source code just suck less but rewritten in Pure C with a little bit of Makefile.
README based on the suckless terminal's README.md.
