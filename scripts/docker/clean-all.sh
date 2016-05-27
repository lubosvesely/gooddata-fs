#! /usr/bin/env bash

# Delete all containers
docker rm --force $(docker ps -a -q)

# Delete all images
docker rmi --force $(docker images -q)
