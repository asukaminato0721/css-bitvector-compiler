#!/bin/bash
set -eux -o pipefail

readarray -t WEBSITE_NAMES < <(ls css-gen-op/)
echo > new_result.txt
for i in "${WEBSITE_NAMES[@]}"; do
	rm -f *results.txt
	echo >src/generated_bitvector_functions.rs
	echo >src/generated_istate_functions.rs
	echo >src/generated_naive_functions.rs
	export WEBSITE_NAME="$i"

	cargo run --bin main

	cargo run --example get_match_result

	cargo fmt

	diff naive_results.txt optimized_results.txt

	diff bitvector_results.txt naive_results.txt
	cargo flamegraph -r --bin benchmark
	cargo run -r --bin tri &>> new_result.txt
	cargo run -r --bin bit &>> new_result.txt
	cargo run -r --bin naive &>> new_result.txt

	uv sync
	uv run create_performance_comparison_plot.py
	uv run create_miss_count_comparison_plot.py
done
