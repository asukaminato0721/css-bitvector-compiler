#!/bin/bash
set -eux -o pipefail

readarray -t WEBSITE_NAMES < <(ls css-gen-op/)
export WEBSITE_NAME=google
cargo run -r --bin naive &> tmp.txt
cargo run -r --bin bit &> bit_tmp.txt
cargo run -r --bin tri &> tri_tmp.txt
diff <(grep '^Descendant' tmp.txt | cut -d' ' -f2-) <(grep '^Done' bit_tmp.txt | cut -d' ' -f2-)
diff <(grep '^Descendant' tmp.txt) <(grep '^Descendant' tri_tmp.txt)
