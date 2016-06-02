.PHONY: list

OS := $(shell uname)
ifeq ($(OS),Darwin)
	# Mac specific
	LINKER_TOOL = otool -L
else
	# Linux specific
	LINKER_TOOL = ldd
endif

all: outdated build stats test

install_deps:
		cargo install cargo-count
		cargo install cargo-graph
		cargo install cargo-multi
		cargo install cargo-outdated

build-debug:
		cargo build

build-release:
		cargo build --release

build: build-debug build-release

clean-debug:
		rm -rf ./target/debug

clean-release:
		rm -rf ./target/release

clean: clean-debug clean-release

deps-debug:
		${LINKER_TOOL} ./target/debug/gooddata-fs

deps-release:
		$LINKER_TOOL ./target/release/gooddata-fs

dot:
		cargo graph --optional-line-style dashed --optional-line-color red --optional-shape box --build-shape diamond --build-color green --build-line-color orange > doc/deps/cargo-count.dot
		dot -Tpng > doc/deps/rainbow-graph.png doc/deps/cargo-count.dot

list:
		@$(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$@$$' | xargs

outdated:
		cargo outdated

rebuild-debug: clean-debug build-debug

rebuild-release: clean-release build-release

rebuild: rebuild-debug rebuild-release

stats:
		cargo count --separator , --unsafe-statistics

test:
		cargo test

update:
		cargo multi update

watch:
		cargo watch
