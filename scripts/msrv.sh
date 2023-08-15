#!/usr/bin/env bash

MSRV=$(grep -oP '"msrv":"\K\d+\.\d+\.\d+' <<< $(cargo msrv --output-format json | tail -1))

CURRENT_MSRV=$(sed -rn 's/"(.*)"/\1/p' <<< $(grep -oP 'rust-version = \K"(\d+\.\d+\.\d+)"' Cargo.toml))

if [ "$MSRV" != "$CURRENT_MSRV" ]; then
  echo "MSRV changed!";
  echo "Old: $CURRENT_MSRV";
  echo "New: $MSRV";
  exit 1;
fi
