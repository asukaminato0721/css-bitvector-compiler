#!/bin/bash
set -eux -o pipefail

# Ensure we start with a clean slate by removing previously generated files
rm -f src/generated_css_functions.rs examples/google_trace_test.rs

# Step 1: Generate the code. This runs `main` without the `run-benchmark` feature,
# so it doesn't try to compile benchmark.rs and avoids the circular dependency.
cargo run -r

# Step 2: Run the benchmark. This enables the `run-benchmark` feature,
# which compiles benchmark.rs (and includes the generated file) and runs the benchmark.
cargo run -r --features run-benchmark -- benchmark

# Step 3: Plot the results.
uv sync
uv run create_performance_comparison_plot.py

