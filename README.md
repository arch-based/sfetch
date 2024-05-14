# sfetch - Suckless Fetching Utility

`sfetch` is a minimalistic, efficient, and "suckless" fetching utility designed to be universal and customizable. It's written in pure C and can be built on any Linux distribution, specifically Debian, Fedora, openSUSE, and Arch Linux. Gentoo next.

## Features

- **Universal Compatibility**: `sfetch` can be built on any Linux distribution, making it highly versatile and accessible to a wide range of users.
- **Customizable Output**: Users can tailor the output to their preferences, enhancing the user experience and allowing for greater flexibility.
- **Source Code Distribution**: `sfetch` is distributed as source code rather than a pre-built binary, promoting transparency and flexibility.
- **Memory Safety**: `sfetch` uses the `free()` function to deallocate memory once it's no longer needed, helping prevent memory leaks and contributing to the overall safety of the utility.

## Building from Source

As per the "suckless" philosophy, `sfetch` is designed to be built from source. Here's how you can do it with the use of the Makefile:

```
# make clean install
```

## Usage

Using the $PATH environment variable you can run it indirectly without typing in the whole path:

```
$ sfetch
```

## Contributing

You may contribute to `sfetch` by building, and making pull requests or opening an issue.

## License

`sfetch` is an open source software licensed under GNU General Public License version 3.0 (GPLv3.0)
