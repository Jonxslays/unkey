#!/usr/bin/env bash

MSRV=$(grep -oP '"msrv":"\K\d+\.\d+\.\d+' <<< $(cargo msrv --output-format json | tail -1))

if [ "$MSRV" != "1.60.0" ]; then
  echo "MSRV changed!";
  echo "Old: 1.60.0";
  echo "New: $MSRV";
  exit 1;
fi
