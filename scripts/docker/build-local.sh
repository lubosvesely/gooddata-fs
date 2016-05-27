#! /usr/bin/env sh

docker build --quiet=false -t gooddata-rust -f ../../Dockerfile ../..
