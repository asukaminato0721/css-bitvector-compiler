#!/bin/bash
set -eux -o pipefail

# Ensure we start with a clean slate by removing previously generated files
rm -f *results.txt
echo > src/generated_bitvector_functions.rs
echo > src/generated_istate_functions.rs
echo > src/generated_naive_functions.rs
# Step 1: Generate the code.

export WEBSITE_NAME="$1"

cargo run --bin main

cargo run --example get_match_result

cargo fmt

# Step 3: Compare the results.
diff naive_results.txt optimized_results.txt

diff bitvector_results.txt naive_results.txt

cargo flamegraph -r --bin benchmark

# Step 5: Plot the results.
uv sync
uv run create_performance_comparison_plot.py
