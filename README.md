# Phantasma

Phantasma is an alternative masterserver for Source-based games. It's designed to be easy to host to yourself.

## Features

- Easy to host
- Configurable
- Lightweight

## Usage

See [docs/usage.md](docs/usage.md).

## API

Phantasma implements the Valve's Master Server Query Protocol, documented here on [developer.valvesoftware.com](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol).

## Building

Install build dependencies. Rust 1.70 or later is required:

```
sudo apt install cargo rust    # Debian
sudo dnf install cargo rust    # Fedora
sudo pacman -S cargo rust      # Arch Linux
```

Then build the program with:

```
cargo build
```

Run the tests with:

```
cargo test
```

## License

[GPLv3](LICENSE)

## Credits

[xash3d-master](https://git.mentality.rip/numas13/xash3d-master)
