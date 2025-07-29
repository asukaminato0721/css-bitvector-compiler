use std::collections::HashSet;
use std::sync::OnceLock;

use css_bitvector_compiler::{rdtsc};
use serde_json::{self, Value};

// Use generated CSS processing functions from both modules

#[derive(Debug, Clone)]
pub struct BenchResult {
    pub frame_id: usize,
    pub operation_type: String,
    pub frame_description: String,
    pub nodes_affected: usize,
    pub total_nodes: usize,
    pub bitvector_cycles: u64,
    pub trivector_cycles: u64,
    pub speedup: f64,
    pub bitvector_hits: usize,
    pub bitvector_misses: usize,
    pub trivector_hits: usize,
    pub trivector_misses: usize,
}
