#!/bin/bash
set -eux -o pipefail

readarray -t WEBSITE_NAMES < <(ls css-gen-op/)

WEBSITE_NAME=google cargo run -r --bin naive &> tmp
WEBSITE_NAME=google cargo run -r --bin bit &> bit_tmp
