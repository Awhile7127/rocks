# rocks


## Description

A simple command-line utility to track the uptime of websites.

Written in Rust, so should be platform independent.


## Compiling

### Dependencies

You need to have installed:

- `libssl-dev`
- `pkg-config`
- `libsqlite3-dev`

For Ubtuntu / Debian-based systems, these are easy to install:

```
sudo apt install libssl-dev pkg-config libsqlite3-dev
```

### Building

Simply run `cargo build --release` in the cloned directory.


## Usage

```
rocks --help
rocks <url> <rate>
```

- url **REQUIRED**: The URL of the server to ping
- rate **OPTIONAL**: The time, in seconds, between pings
