#!/bin/bash
set -eux -o pipefail

# Ensure we start with a clean slate by removing previously generated files
rm -f src/generated_css_functions.rs examples/google_trace_test.rs

# Step 1: Run the benchmark.
# The build process (triggered by `cargo run` or `cargo build`) will automatically
# execute `build.rs` which now handles code generation.
# This command enables the `run-benchmark` feature,
# which compiles benchmark.rs (and includes the generated file) and runs the benchmark.
cargo run -r --features run-benchmark -- benchmark

# Step 2: Plot the results.
uv sync
uv run create_performance_comparison_plot.py
