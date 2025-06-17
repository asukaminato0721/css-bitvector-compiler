#!/bin/bash
set -eux -o pipefail
cargo run -r
cargo run --features generated-css -r
cargo run benchmark --features generated-css
python3 create_performance_comparison_plot.py

