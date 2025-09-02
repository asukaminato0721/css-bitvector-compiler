#!/bin/bash
set -eux -o pipefail

readarray -t WEBSITE_NAMES < <(ls css-gen-op/)

for name in "${WEBSITE_NAMES[@]}"; do
    export WEBSITE_NAME=$name
    cargo run -r --bin naive &> css-gen-op/$name/tmp.txt || true
    cargo run -r --bin bit &> css-gen-op/$name/bit_tmp.txt || true
    cargo run -r --bin tri &> css-gen-op/$name/tri_tmp.txt || true
   # difft <(grep '^MATCH' tmp.txt | cut -d' ' -f2-) <(grep '^MATCH' bit_tmp.txt | cut -d' ' -f2-)
   # difft <(grep '^MATCH' tmp.txt) <(grep '^MATCH' tri_tmp.txt)
done
