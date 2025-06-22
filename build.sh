#!/bin/sh

dir=$(dirname "$0")

podman build \
  -t "ghcr.io/nblxa/kache:latest" \
  --platform linux/amd64 \
  -f "$dir/build.dockerfile" \
  "$dir"
