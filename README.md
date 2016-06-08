# gooddata-fs

GoodData as Filesystem (using FUSE)

For more information about GoodData Filesystem Structure see [detailed documentation](https://github.com/korczis/gooddata-fs/blob/master/doc/Filesystem.md) - [root](https://github.com/korczis/gooddata-fs/blob/master/doc/Filesystem.md#root).

## Notice

**This is not official GoodData project nor is supported or recommended for production use. You have been warned.**

## Status

[![Build Status](https://travis-ci.org/korczis/gooddata-fs.svg?branch=master)](https://travis-ci.org/korczis/gooddata-fs)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/korczis/gooddata-rust/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/korczis/gooddata-rust.svg)](https://github.com/korczis/gooddata-rust/issues)

## Binary Information 

[Version used](https://github.com/korczis/gooddata-fs/tree/d086fe54dba29d842e0098ad6521fbb99e24079b)

### Resources Consumption

- Binary size - 424 KB
- Real Memory Size - 5.3 MB
- Shared Memory Size - 228 KB
- Private Memory Size - 3.6 MB

**Command**

```
tree gd

...

15 directories, 49 files
```

*Test account has access to 14 projects*

### Dependencies

```
otool -L ./target/debug/gooddata-fs
./target/debug/gooddata-fs:
	/usr/local/lib/libosxfuse_i64.2.dylib (compatibility version 10.0.0, current version 10.3.0)
	/usr/lib/libiconv.2.dylib (compatibility version 7.0.0, current version 7.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1226.10.1)
	/usr/lib/libssl.0.9.8.dylib (compatibility version 0.9.8, current version 0.9.8)
	/usr/lib/libcrypto.0.9.8.dylib (compatibility version 0.9.8, current version 0.9.8)
	/usr/lib/libz.1.dylib (compatibility version 1.0.0, current version 1.2.5)
```

## Security

Thanks to [Rust language](https://www.rust-lang.org/) features [gooddata-fs](https://github.com/korczis/gooddata-fs) is immune to:

- [Stack Overflow](https://en.wikipedia.org/wiki/Stack_buffer_overflow) -  occurs when a program writes to a memory address on the program's call stack outside of the intended data structure, which is usually a fixed-length buffer.
- [Heap Overflow](https://en.wikipedia.org/wiki/Heap_overflow) - type of buffer overflow that occurs in the heap data area.
- [Integer Overflow](https://en.wikipedia.org/wiki/Integer_overflow) -  occurs when an arithmetic operation attempts to create a numeric value that is too large to be represented within the available storage space.
- [Dangling Pointers](https://en.wikipedia.org/wiki/Dangling_pointer) -  special cases of memory safety violations.

## Why Rust?

- Zero-cost abstractions
- Move semantics
- Guaranteed memory safety
- Threads without data races
- Trait-based generics
- Pattern matching
- Type inference
- Minimal runtime

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
