#!/bin/bash
set -eux -o pipefail

# Ensure we start with a clean slate by removing previously generated files
rm -f src/generated_css_functions.rs examples/google_trace_test.rs

# Step 1: Generate the code. This runs `main` without the `run-benchmark` feature,
# so it doesn't try to compile benchmark.rs and avoids the circular dependency.
cargo run --bin main

# Step 2: Generate the naive code.
cargo run --bin naive-gen

# Step 3: Compare the results.
diff naive_results.txt optimized_results.txt

# Step 4: Run the benchmark. This enables the `run-benchmark` feature,
# which compiles benchmark.rs (and includes the generated file) and runs the benchmark.
cargo flamegraph -r --features run-benchmark --bin main -- benchmark

# Step 5: Plot the results.
uv sync
uv run create_performance_comparison_plot.py
