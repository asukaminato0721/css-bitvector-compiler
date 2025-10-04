#!/bin/bash
set -eux -o pipefail

readarray -t WEBSITE_NAMES < <(ls css-gen-op/)

for name in "${WEBSITE_NAMES[@]}"; do
    export WEBSITE_NAME=$name
    cargo run -r --bin naive &> css-gen-op/$name/tmp.txt || true
    cargo run -r --bin bit &> css-gen-op/$name/bit_tmp.txt || true
    cargo run -r --bin tri &> css-gen-op/$name/tri_tmp.txt || true
    cargo run -r --bin quad &> css-gen-op/$name/quad_tmp.txt || true
    diff \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/tmp.txt | sort) \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/bit_tmp.txt | sort)

    diff \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/tmp.txt | sort) \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/tri_tmp.txt | sort)

    diff \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/tmp.txt | sort) \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/quad_tmp.txt | sort)


done

./scripts/collect_miss_cnt.py
