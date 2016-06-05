# gooddata-fs

GoodData as Filesystem (using FUSE)

For more information about GoodData Filesystem Structure see [detailed documentation](https://github.com/korczis/gooddata-fs/blob/master/doc/Filesystem.md) - [root](https://github.com/korczis/gooddata-fs/blob/master/doc/Filesystem.md#root).

## Notice

**This is not official GoodData project nor is supported or recommended for production use. You have been warned.**

## Status

[![Build Status](https://travis-ci.org/korczis/gooddata-fs.svg?branch=master)](https://travis-ci.org/korczis/gooddata-fs)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/korczis/gooddata-rust/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/korczis/gooddata-rust.svg)](https://github.com/korczis/gooddata-rust/issues)

## Binary Information [this version](https://github.com/korczis/gooddata-fs/tree/d086fe54dba29d842e0098ad6521fbb99e24079b)

### Resources Consumption

- Binary size - 612KB 
- Peak RAM Usage - 4.1MB

### Dependencies

otool -L ./target/debug/gooddata-fs
./target/debug/gooddata-fs:
	/usr/local/lib/libosxfuse_i64.2.dylib (compatibility version 10.0.0, current version 10.3.0)
	/usr/lib/libiconv.2.dylib (compatibility version 7.0.0, current version 7.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1226.10.1)
	/usr/lib/libssl.0.9.8.dylib (compatibility version 0.9.8, current version 0.9.8)
	/usr/lib/libcrypto.0.9.8.dylib (compatibility version 0.9.8, current version 0.9.8)
	/usr/lib/libz.1.dylib (compatibility version 1.0.0, current version 1.2.5)

## Supported Operating Systems
* [Unix-like](https://en.wikipedia.org/wiki/Unix-like)

## Prerequisites

* [Rust language](https://www.rust-lang.org/)
* [libfuse](https://github.com/libfuse/libfuse) or [FUSE OSX](https://osxfuse.github.io/)

*Optional*

* [Docker](https://www.docker.com/)

### Mac Specific

* [Homebew](http://brew.sh/)

*Optional*

* [Virtualbox](https://www.virtualbox.org/)
* [Docker Toolbox](https://www.docker.com/products/docker-toolbox)

## Getting Started

### Clone & Build

```
git clone https://github.com/korczis/gooddata-fs
cd gooddata-fs
cargo build
```

### Install (optional)

```
cargo install
```

## Mounting GoodData as Filesystem

### Running built binary

```
# ./target/debug/gooddata-fs <USERNAME> <PASSWORD> <MOUNTPOINT>

RUST_BACKTRACE=1 RUST_LOG=debug ./target/debug/gooddata-fs joe.doe@gooddata.com secretpassword /Users/joe.doe/gd
```

### Running installed binary

```
# gooddata-fs <USERNAME> <PASSWORD> <MOUNTPOINT>

RUST_BACKTRACE=1 RUST_LOG=debug gooddata-fs joe.doe@gooddata.com secretpassword /Users/joe.doe/gd
```

## Filesystem structure

### root

```
.
├── projects
│   ├── $ocka
│   ├── Boot Camp 3 Exercises 01
│   ├── Date Dictionary
│   ├── GDC Git 0.2
│   ├── GoodDuty & Calendars
│   ├── GoodStatistics Demo
│   ├── MAQL Boot Camp 01
│   ├── MS ETL 3.0
│   ├── Ruby downloaders
│   └── Training March
└── user.json
```

## Development

### Docker

```
cd scripts/docker
./build-local.sh
```

### Environment

### Mac Specific

Make sure you have openssl installed and linked.

```
brew install openssl
brew link --force openssl
```

### Tools

```
cargo install racer
cargo install rustfmt
```

## Customization

- [/src/rest/url.rs](https://github.com/korczis/gooddata-fs/blob/master/src/rest/url.rs)
- [/src/fs/constants.rs](https://github.com/korczis/gooddata-fs/blob/master/src/fs/constants.rs)
