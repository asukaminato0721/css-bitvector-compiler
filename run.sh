#!/bin/bash
set -eux -o pipefail

export WEBSITE_NAME=${1:-google}
cargo run -r --bin naive
