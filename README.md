# gooddata-rust

GoodData as Filesystem (using FUSE) and Rust Language SDK

## Status

[![Build Status](https://travis-ci.org/korczis/gooddata-rust.svg?branch=master)](https://travis-ci.org/korczis/gooddata-rust)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/korczis/gooddata-rust/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/korczis/gooddata-rust.svg)](https://github.com/korczis/gooddata-rust/issues)

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
git clone https://github.com/korczis/gooddata-rust
cd gooddata-rust
cargo build
```

### Install (optional)

```
cargo install
```

## Mounting GoodData as Filesystem

### Running built binary 

```
# gooddata-rust <USERNAME> <PASSWORD> <MOUNTPOINT>

RUST_BACKTRACE=1 RUST_LOG=debug ./target/debug/gooddata-fuse joe.doe@gooddata.com secretpassword /Users/joe.doe/gd
```

### Running installed binary 

```
# gooddata-rust <USERNAME> <PASSWORD> <MOUNTPOINT>

RUST_BACKTRACE=1 RUST_LOG=debug gooddata-fuse joe.doe@gooddata.com secretpassword /Users/joe.doe/gd
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
