#!/bin/sh

dir=$(dirname "$0")

img="ghcr.io/nblxa/kache:latest"

podman build \
  -t "$img" \
  --cache-from "$img" \
  --platform linux/amd64 \
  -f "$dir/build.dockerfile" \
  "$dir"
