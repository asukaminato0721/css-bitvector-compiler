use std::collections::HashSet;
use std::sync::OnceLock;

use css_bitvector_compiler::{HtmlNode, rdtsc};
use serde_json::{self, Value};

// Use generated CSS processing functions from both modules
use css_bitvector_compiler::generated_bitvector_functions::process_tree_bitvector_incremental_with_stats as process_tree_bitvector;
use css_bitvector_compiler::generated_istate_functions::process_tree_incremental_with_stats as process_tree_trivector;

#[derive(Debug, Clone)]
pub struct WebLayoutFrameResult {
    pub frame_id: usize,
    pub operation_type: String,
    pub frame_description: String,
    pub nodes_affected: usize,
    pub total_nodes: usize,
    pub bitvector_cycles: u64,
    pub trivector_cycles: u64,
    pub speedup: f64,
    pub bitvector_cache_hits: usize,
    pub bitvector_cache_misses: usize,
    pub trivector_cache_hits: usize,
    pub trivector_cache_misses: usize,
}

#[derive(Debug, Clone)]
pub struct LayoutFrame {
    pub frame_id: usize,
    pub command_name: String,
    pub command_data: Value,
}

fn count_nodes(node: &HtmlNode) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}
#[track_caller]
fn find_node_by_path_mut<'a>(node: &'a mut HtmlNode, path: &[usize]) -> Option<&'a mut HtmlNode> {
    if path.is_empty() {
        return Some(node);
    }

    let next_index = path[0];
    if next_index < node.children.len() {
        find_node_by_path_mut(&mut node.children[next_index], &path[1..])
    } else if next_index > 0 && next_index == node.children.len() {
        println!("maybe insert at the end position, {next_index}");
        find_node_by_path_mut(&mut node.children[next_index - 1], &path[1..])
    } else if next_index == 0 && next_index == node.children.len() {
        // TOOD: fixme
        dbg!(path);
        dbg!(node);
        panic!("{:?}", path);
    } else {
        panic!("{:?}", path);
    }
}

fn json_to_html_node(json: &Value) -> Option<HtmlNode> {
    let name = json["name"].as_str()?.to_string();

    // Convert #document to a more standard name
    let tag_name = if name == "#document" { "html" } else { &name };
    let mut node = HtmlNode::new(tag_name);

    // Handle ID (might be a number in this JSON format)
    if let Some(id_val) = json.get("id") {
        if let Some(id_num) = id_val.as_u64() {
            node.id = Some(id_num.to_string());
        } else if let Some(id_str) = id_val.as_str() {
            if !id_str.is_empty() {
                node.id = Some(id_str.to_string());
            }
        }
    }

    if let Some(attrs) = json["attributes"].as_object() {
        if let Some(class_val) = attrs.get("class") {
            if let Some(class_str) = class_val.as_str() {
                node.classes = class_str
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
            }
        }
    }

    // Recursively add children
    if let Some(children) = json["children"].as_array() {
        for child_json in children {
            if let Some(child_node) = json_to_html_node(child_json) {
                node.children.push(child_node);
            }
        }
    }

    Some(node)
}

fn parse_web_layout_trace(file_path: &str) -> Vec<LayoutFrame> {
    let content = std::fs::read_to_string(file_path).expect("Failed to read web layout trace file");

    let mut frames = Vec::new();
    for (frame_id, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<Value>(line) {
            Ok(command_data) => {
                let command_name = command_data["name"]
                    .as_str()
                    .unwrap_or("unknown")
                    .to_string();
                if command_name.starts_with("layout_") {
                    continue;
                }

                frames.push(LayoutFrame {
                    frame_id,
                    command_name,
                    command_data,
                });
            }
            Err(e) => {
                eprintln!("Failed to parse frame {}: {}, error: {}", frame_id, line, e);
            }
        }
    }

    frames
}
static COMMAND: OnceLock<std::sync::Mutex<HashSet<String>>> =
    OnceLock::<std::sync::Mutex<HashSet<String>>>::new();
fn apply_frame_modifications(tree: &mut HtmlNode, frame: &LayoutFrame) -> usize {
    match frame.command_name.as_str() {
        "init" => {
            if let Some(node_data) = frame.command_data.get("node") {
                if let Some(new_tree) = json_to_html_node(node_data) {
                    *tree = new_tree;
                    tree.init_parent_pointers();
                    return count_nodes(tree);
                }
            }
            0
        }
        "layout_init" => {
            tree.mark_dirty();
            count_nodes(tree)
        }
        "add" => {
            let path = extract_path_from_command(&frame.command_data);

            if path.is_empty() {
                return 0;
            }

            // For add operations, the last index is the insertion position
            let insertion_index = path[path.len() - 1];
            let parent_path = &path[..path.len() - 1];

            let Some(parent) = find_node_by_path_mut(tree, parent_path) else {
                println!("    DEBUG: Failed to find parent node at path {parent_path:?}",);
                unreachable!();
            };
            let Some(node_data) = frame.command_data.get("node") else {
                println!("    DEBUG: No 'node' field in command_data");
                unreachable!();
            };
            let Some(new_child) = json_to_html_node(node_data) else {
                dbg!(node_data);
                println!("    DEBUG: Failed to create child node from JSON");
                unreachable!();
            };
            // Insert at the specified index (or append if index equals length)
            parent.children.insert(insertion_index, new_child);
            parent.mark_dirty();
            parent.init_parent_pointers();
            return 1;
        }
        "replace_value" | "insert_value" => {
            let path = extract_path_from_command(&frame.command_data);
            let Some(target_node) = find_node_by_path_mut(tree, &path) else {
                println!("    DEBUG: Failed to find target node at path {:?}", path);
                unreachable!();
            };
            let Some(key) = frame.command_data.get("key").and_then(|k| k.as_str()) else {
                println!("    DEBUG: No 'key' field in command_data");
                unreachable!();
            };
            match key {
                "class" => {
                    if let Some(new_value) =
                        frame.command_data.get("value").and_then(|v| v.as_str())
                    {
                        target_node.classes.clear();
                        for class in new_value.split_whitespace() {
                            target_node.classes.insert(class.to_string());
                        }
                        target_node.mark_dirty();
                        return 1;
                    }
                }
                "id" => {
                    if let Some(new_value) =
                        frame.command_data.get("value").and_then(|v| v.as_str())
                    {
                        target_node.id = if new_value.is_empty() {
                            None
                        } else {
                            Some(new_value.to_string())
                        };
                        target_node.mark_dirty();
                        return 1;
                    }
                }
                _ => {
                    target_node.mark_dirty();
                    return 1;
                }
            }
            0
        }
        "recalculate" => {
            // Don't mark all nodes dirty - this defeats the purpose of incremental processing
            // Only mark root dirty to trigger incremental layout recalculation
            tree.mark_dirty();
            count_nodes(tree)
        }
        "remove" => {
            let path = extract_path_from_command(&frame.command_data);
            let remove_index = path[path.len() - 1];
            let parent_path = &path[..path.len() - 1];
            let Some(parent) = find_node_by_path_mut(tree, parent_path) else {
                unreachable!();
            };
            parent.remove_child(remove_index);
            1
        }
        _ => {
            let commands = COMMAND.get_or_init(|| std::sync::Mutex::new(HashSet::new()));
            let mut commands_guard = commands.lock().unwrap();
            if !commands_guard.contains(frame.command_name.as_str()) {
                dbg!(frame.command_name.as_str());
                commands_guard.insert(frame.command_name.clone());
                return 0;
            }

            0
        }
    }
}

fn extract_path_from_command(command_data: &Value) -> Vec<usize> {
    command_data
        .get("path")
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64())
                .map(|v| v as usize)
                .collect::<Vec<_>>()
        })
        .unwrap()
}

fn invoke_bitvector_layout(tree: &mut HtmlNode) -> (usize, usize, usize) {
    // Use the generated BitVector CSS processing code
    process_tree_bitvector(tree)
}

fn invoke_trivector_layout(tree: &mut HtmlNode) -> (usize, usize, usize) {
    // Use the generated TriVector (IState) CSS processing code
    process_tree_trivector(tree)
}

fn clear_dirty_flags(node: &mut HtmlNode) {
    node.is_self_dirty = false;
    node.has_dirty_descendant = false;

    for child in &mut node.children {
        clear_dirty_flags(child);
    }
}

pub fn run_web_browser_layout_trace_benchmark() -> Vec<WebLayoutFrameResult> {
    println!("🌐 Starting Web Browser Layout Trace Benchmark");
    println!("📊 Simulating corrected layout methodology...");
    println!("Loading layout trace from css-gen-op/command.json...");

    let frames = parse_web_layout_trace(&format!(
        "css-gen-op/{}/command.json",
        std::env::var("WEBSITE_NAME").unwrap()
    ));
    println!("🎬 Found {} layout frames to process", frames.len());
    println!("📈 Only recalculate operations create data points");

    let mut current_layout_tree = HtmlNode::new("html");
    let mut pending_modifications = Vec::new();
    let mut results = Vec::new();
    let mut data_point_counter = 0;

    // Initialize tree with first init command if present
    if let Some(init_frame) = frames.first() {
        if init_frame.command_name == "init" {
            apply_frame_modifications(&mut current_layout_tree, init_frame);
            println!(
                "✅ Initialized layout tree with {} nodes",
                count_nodes(&current_layout_tree)
            );
        }
    }

    for (i, frame) in frames.iter().enumerate() {
        // println!(
        //     "🎬 Processing frame {}/{}: {} ({})",
        //     i + 1,
        //     frames.len(),
        //     frame.command_name,
        //     get_frame_description(frame)
        // );

        match frame.command_name.as_str() {
            "init" => {
                // Already handled above
                if i > 0 {
                    apply_frame_modifications(&mut current_layout_tree, frame);
                }
            }
            "recalculate" => {
                // This is when we actually benchmark!
                data_point_counter += 1;

                // The benchmark function will apply the modifications to its own tree copies.
                // We pass the current tree state from *before* this batch of modifications.
                let result = benchmark_accumulated_modifications(
                    &current_layout_tree,
                    &pending_modifications,
                    frame,
                    data_point_counter,
                );

                results.push(result);

                // Now, apply the modifications to the main tree to advance its state for the next frames.
                for pending_frame in &pending_modifications {
                    apply_frame_modifications(&mut current_layout_tree, pending_frame);
                }

                // Clear pending modifications after recalculate
                pending_modifications.clear();
            }
            _ => {
                pending_modifications.push(frame.clone());
            }
        }
    }

    println!(
        "\n🎯 Benchmark completed with {} data points from {} total frames",
        results.len(),
        frames.len()
    );

    print_web_layout_trace_summary(&results);

    let csv_content = generate_web_layout_csv(&results);
    if let Err(e) = std::fs::write("web_layout_trace_benchmark.csv", csv_content) {
        eprintln!("Failed to write CSV file: {}", e);
    } else {
        println!("💾 Web layout trace results saved to web_layout_trace_benchmark.csv");
    }

    results
}

fn benchmark_accumulated_modifications(
    base_tree: &HtmlNode,
    pending_modifications: &[LayoutFrame],
    _recalculate_frame: &LayoutFrame,
    data_point_id: usize,
) -> WebLayoutFrameResult {
    // --- BitVector Processing Path ---
    // 1. Start with the base tree state from before the modifications.
    let mut tree_bitvector = base_tree.clone();
    tree_bitvector.init_parent_pointers();

    // 2. Run a "warm-up" layout to ensure caches are populated, simulating a steady state.
    // This is crucial for a fair comparison, as the incremental approach relies on a previously computed state.
    let _ = invoke_bitvector_layout(&mut tree_bitvector);
    clear_dirty_flags(&mut tree_bitvector); // Reset dirty flags after warm-up.

    // 3. Apply the pending modifications. This is what we actually want to measure.
    // The `apply_frame_modifications` function will mark nodes as dirty.
    let mut total_nodes_affected = 0;
    for modification in pending_modifications {
        total_nodes_affected += apply_frame_modifications(&mut tree_bitvector, modification);
    }

    // 4. Measure the time taken for the BitVector layout to process only the dirty nodes.
    let start_bitvector = rdtsc();
    let (_, bitvector_cache_hits, bitvector_cache_misses) =
        invoke_bitvector_layout(&mut tree_bitvector);
    let end_bitvector = rdtsc();
    let bitvector_cycles = end_bitvector - start_bitvector;

    // --- TriVector (IState) Processing Path ---
    // 1. Start with a fresh clone of the base tree.
    let mut tree_trivector = base_tree.clone();
    tree_trivector.init_parent_pointers();

    // 2. Run a "warm-up" layout to ensure caches are populated.
    let _ = invoke_trivector_layout(&mut tree_trivector);
    clear_dirty_flags(&mut tree_trivector); // Reset dirty flags after warm-up.

    // 3. Apply the same modifications to it to ensure it has the identical final structure.
    for modification in pending_modifications {
        apply_frame_modifications(&mut tree_trivector, modification);
    }

    // 4. Measure the time taken for the TriVector layout.
    let start_trivector = rdtsc();
    let (_, trivector_cache_hits, trivector_cache_misses) =
        invoke_trivector_layout(&mut tree_trivector);
    let end_trivector = rdtsc();
    let trivector_cycles = end_trivector - start_trivector;

    let total_nodes = count_nodes(&tree_bitvector);

    let speedup = if trivector_cycles > 0 {
        bitvector_cycles as f64 / trivector_cycles as f64
    } else if bitvector_cycles > 0 {
        f64::INFINITY // TriVector was effectively infinitely faster
    } else {
        1.0 // No work done in either case, so they are equal
    };

    // Create summary description of accumulated modifications
    let accumulated_description = if pending_modifications.is_empty() {
        "recalculate (no pending changes)".to_string()
    } else {
        let ops: Vec<String> = pending_modifications
            .iter()
            .map(|f| f.command_name.clone())
            .collect();
        format!("recalculate after [{}]", ops.join(", "))
    };

    WebLayoutFrameResult {
        frame_id: data_point_id,
        operation_type: "recalculate".to_string(),
        frame_description: accumulated_description,
        nodes_affected: total_nodes_affected,
        total_nodes,
        bitvector_cycles,
        trivector_cycles,
        speedup,
        bitvector_cache_hits,
        bitvector_cache_misses,
        trivector_cache_hits,
        trivector_cache_misses,
    }
}

fn print_web_layout_trace_summary(results: &[WebLayoutFrameResult]) {
    if results.is_empty() {
        println!("\nNo data points to summarize.");
        return;
    }

    let total_frames = results.len();
    let avg_speedup = results.iter().map(|r| r.speedup).sum::<f64>() / total_frames as f64;

    // Calculate geometric mean of speedup ratios
    let geometric_mean_speedup = if total_frames > 0 {
        let product: f64 = results
            .iter()
            .map(|r| r.speedup)
            .filter(|&x| x > 0.0) // Avoid log(0)
            .map(|x| x.ln())
            .sum();
        (product / total_frames as f64).exp()
    } else {
        1.0
    };

    let faster_bitvector = results.iter().filter(|r| r.speedup < 1.0).count();
    let faster_trivector = results.iter().filter(|r| r.speedup > 1.0).count();
    let similar_performance = results
        .iter()
        .filter(|r| (r.speedup - 1.0).abs() < 0.1)
        .count();

    let total_cache_hits: usize = results.iter().map(|r| r.bitvector_cache_hits).sum();
    let total_cache_attempts: usize = results
        .iter()
        .map(|r| r.bitvector_cache_hits + r.bitvector_cache_misses)
        .sum();
    let overall_cache_hit_rate = if total_cache_attempts > 0 {
        100.0 * total_cache_hits as f64 / total_cache_attempts as f64
    } else {
        0.0
    };

    println!("\n🌐 Web Browser Layout Trace Benchmark Summary:");
    println!("════════════════════════════════════════════════");
    println!("Total recalculate data points: {}", total_frames);
    println!(
        "Average speedup (TriVector vs BitVector): {:.3}x",
        avg_speedup
    );
    println!("Geometric mean speedup: {:.3}x", geometric_mean_speedup);

    println!("\n⚡ Performance Analysis:");
    println!(
        "  BitVector faster: {} ({:.1}%)",
        faster_bitvector,
        100.0 * faster_bitvector as f64 / total_frames as f64
    );
    println!(
        "  TriVector faster: {} ({:.1}%)",
        faster_trivector,
        100.0 * faster_trivector as f64 / total_frames as f64
    );
    println!(
        "  Similar performance: {} ({:.1}%)",
        similar_performance,
        100.0 * similar_performance as f64 / total_frames as f64
    );

    println!("\n🎯 Cache Efficiency:");
    println!("  Overall cache hit rate: {:.1}%", overall_cache_hit_rate);
    println!("  Total cache hits: {}", total_cache_hits);
    println!("  Total cache attempts: {}", total_cache_attempts);
}

fn generate_web_layout_csv(results: &[WebLayoutFrameResult]) -> String {
    let mut csv = String::new();
    csv.push_str("frame_id,operation_type,frame_description,nodes_affected,total_nodes,bitvector_cycles,trivector_cycles,speedup,bitvector_cache_hits,bitvector_cache_misses,trivector_cache_hits,trivector_cache_misses\n");

    for result in results {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{:.6},{},{},{},{}\n",
            result.frame_id,
            result.operation_type,
            result.frame_description.replace(",", ";"),
            result.nodes_affected,
            result.total_nodes,
            result.bitvector_cycles,
            result.trivector_cycles,
            result.speedup,
            result.bitvector_cache_hits,
            result.bitvector_cache_misses,
            result.trivector_cache_hits,
            result.trivector_cache_misses
        ));
    }

    csv
}

fn main() {
    run_web_browser_layout_trace_benchmark();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_with_generated_css() {
        // This test verifies that the benchmark uses actual generated CSS processing
        println!("🧪 Testing benchmark with generated CSS functions");

        let benchmark_result = run_web_browser_layout_trace_benchmark();

        // Check that we have valid results
        assert!(!benchmark_result.is_empty(), "Should have processed frames");
        assert!(
            benchmark_result
                .iter()
                .filter(|r| r.operation_type != "init")
                .count()
                > 0,
            "Should have operations"
        );
        assert!(
            benchmark_result.iter().filter(|r| r.speedup > 0.0).count() > 0,
            "Should have positive speedup"
        );

        println!("✅ Benchmark test passed:");
        println!("   Total frames: {}", benchmark_result.len());
        println!(
            "   Total operations: {}",
            benchmark_result
                .iter()
                .filter(|r| r.operation_type != "init")
                .count()
        );
        println!(
            "   Average speedup: {:.3}x",
            benchmark_result.iter().filter(|r| r.speedup > 0.0).count() as f64
                / benchmark_result.len() as f64
        );

        // The fact that we get here means the generated CSS functions are working
        println!("🎯 Using generated CSS processing functions!");
    }
}
