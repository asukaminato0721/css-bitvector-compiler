#!/bin/bash
set -eux -o pipefail

readarray -t WEBSITE_NAMES < <(ls css-gen-op/)

for name in "${WEBSITE_NAMES[@]}"; do
    export WEBSITE_NAME=$name
    cargo run -r --bin naive &> tmp.txt
    cargo run -r --bin bit &> bit_tmp.txt
    cargo run -r --bin tri &> tri_tmp.txt
    difft <(grep '^MATCH' tmp.txt | cut -d' ' -f2-) <(grep '^MATCH' bit_tmp.txt | cut -d' ' -f2-)
   # difft <(grep '^MATCH' tmp.txt) <(grep '^Descendant' tri_tmp.txt)
done
