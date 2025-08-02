#!/bin/bash
set -eux -o pipefail

readarray -t WEBSITE_NAMES < <(ls css-gen-op/)
export WEBSITE_NAME=google
cargo run -r --bin naive &> tmp.txt
cargo run -r --bin bit &> bit_tmp.txt
cargo run -r --bin tri &> tri_tmp.txt
difft <(grep '^Descendant' tmp.txt) <(grep '^Descendant' bit_tmp.txt)
difft <(grep '^Descendant' tmp.txt) <(grep '^Descendant' tri_tmp.txt)
