# gooddata-rust

GoodData Rust Language SDK

## Status

[![Build Status](https://travis-ci.org/korczis/gooddata-rust.svg?branch=master)](https://travis-ci.org/korczis/gooddata-rust)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/korczis/gooddata-rust/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/korczis/gooddata-rust.svg)](https://github.com/korczis/gooddata-rust/issues)

## Prerequisites

* [Rust language](https://www.rust-lang.org/)
* [libfuse](https://github.com/libfuse/libfuse) or [FUSE OSX](https://osxfuse.github.io/) 

## Getting Started

```
git clone https://github.com/korczis/gooddata-rust
cd gooddata-rust
cargo build
cargo install
```

## Mounting GoodData as Filesystem

```
# gooddata-rust <USERNAME> <PASSWORD> <MOUNTPOINT>

gooddata-rust joe.doe@gooddata.com secretpassword /Users/joe.doe/gd
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
