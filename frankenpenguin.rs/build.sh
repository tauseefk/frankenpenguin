#!/bin/bash

set -euo pipefail

TARGET=bundler
OUTDIR=../../www/webgl

wasm-pack build webgl --target $TARGET --release --out-dir $OUTDIR
