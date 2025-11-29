#!/bin/bash
set -eux -o pipefail

if command -v difft >/dev/null 2>&1; then
   DIFF_CMD=(difft)
else
   DIFF_CMD=(diff -u)
fi

readarray -t WEBSITE_NAMES < <(ls css-gen-op/)
rm -rf css-gen-op/__pycache__

for name in "${WEBSITE_NAMES[@]}"; do
   if test ! -d "css-gen-op/$name"; then
      continue
   fi
    export WEBSITE_NAME=$name
    cargo run -r --bin naive &> css-gen-op/$name/tmp.txt || true
    cargo run -r --bin bit &> css-gen-op/$name/bit_tmp.txt || true
    cargo run -r --bin tri &> css-gen-op/$name/tri_tmp.txt || true
    cargo run -r --bin quad &> css-gen-op/$name/quad_tmp.txt || true
    "${DIFF_CMD[@]}" \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/tmp.txt | sort) \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/bit_tmp.txt | sort)

    "${DIFF_CMD[@]}" \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/tmp.txt | sort) \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/tri_tmp.txt | sort)

    "${DIFF_CMD[@]}" \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/tmp.txt | sort) \
       <(awk '/BEGIN/{flag=1; next} /END/{flag=0} flag' ./css-gen-op/$name/quad_tmp.txt | sort)


done

./scripts/collect_miss_cnt.py
