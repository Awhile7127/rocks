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

### Setting up the database

`rocks` writes to a database in the directory in which it is executed, called `statuses.db`.

In order for this program to function, you must first create this database. It is recommended
to have installed `sqlite3` alongside the rest of the above dependencies:

```
sudo apt install sqlite3
```

Then, create the database and create a new table:

```
sqlite3 statuses.db
CREATE TABLE statuses (time TEXT, status TEXT);
.exit
```

Then, you can run `rocks` like so:

```
rocks --help
rocks <url> <rate>
```

- url **REQUIRED**: The URL of the server to ping
- rate **OPTIONAL**: The time, in seconds, between pings
