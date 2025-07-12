#!/bin/bash
set -eux -o pipefail

# Ensure we start with a clean slate by removing previously generated files
rm -f examples/google_trace_test.rs
rm -f *results.txt

# Create placeholder for generated functions to avoid compilation error
if [ ! -f src/generated_css_functions.rs ]; then
    cat > src/generated_css_functions.rs << 'EOF'
// Placeholder file - will be overwritten by code generator
use crate::{BitVector, HtmlNode};

pub const BITVECTOR_CAPACITY: usize = 0;

pub fn process_node_generated_incremental(
    _node: &mut HtmlNode,
    _parent_state: &BitVector,
) -> BitVector {
    BitVector::new()
}

pub fn process_node_generated_from_scratch(
    _node: &mut HtmlNode,
    _parent_state: &BitVector,
) -> BitVector {
    BitVector::new()
}
EOF
fi


# Step 1: Generate the code. This runs `main` without the `run-benchmark` feature,
# so it doesn't try to compile benchmark.rs and avoids the circular dependency.

export WEBSITE_NAME="$1"

cargo run --bin main

# Step 2: Generate the naive code.
cargo run --bin naive-gen

cargo run --example bitvector_only_test

cargo fmt

# Step 3: Compare the results.
diff naive_results.txt optimized_results.txt

diff bitvector_results.txt naive_results.txt

cargo flamegraph -r --bin benchmark

# Step 5: Plot the results.
uv sync
uv run create_performance_comparison_plot.py
