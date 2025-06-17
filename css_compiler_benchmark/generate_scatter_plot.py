#!/usr/bin/env python3
"""
Generate scatter plot comparing Incremental vs From-Scratch CSS processing performance.
Similar to the reference plot showing Double Dirty Bit vs Spineless Traversal.
"""

from typing import cast

import matplotlib.pyplot as plt
import numpy as np
import numpy.typing as npt
import pandas as pd


def load_benchmark_data(csv_file: str):
    """Load benchmark data from CSV file."""
    try:
        df = pd.read_csv(csv_file)
        print(f"Loaded {len(df)} data points from {csv_file}")
        return df
    except FileNotFoundError:
        print(f"Error: Could not find {csv_file}")
        return None


def create_scatter_plot(
    df: pd.DataFrame, output_file: str = "performance_scatter_plot.png"
):
    """Create scatter plot similar to the reference image."""

    # Set up the plot style
    plt.style.use("default")
    fig, ax = plt.subplots(figsize=(8, 8))

    # Extract data
    incremental_cycles = df["incremental_cycles"].values
    fromscratch_cycles = df["fromscratch_cycles"].values

    # Create log-log scatter plot
    ax.loglog(
        fromscratch_cycles,
        incremental_cycles,
        "o",
        alpha=0.6,
        markersize=4,
        color="steelblue",
        markeredgewidth=0.5,
        markeredgecolor="darkblue",
    )

    # Calculate plot range
    min_cycles = min(min(incremental_cycles), min(fromscratch_cycles))
    max_cycles = max(max(incremental_cycles), max(fromscratch_cycles))

    # Add diagonal line (x = y, equal performance)
    diagonal_range = np.logspace(
        np.log10(min_cycles * 0.8), np.log10(max_cycles * 1.2), 100
    )
    ax.loglog(
        diagonal_range,
        diagonal_range,
        "k-",
        linewidth=2,
        alpha=0.8,
        label="Equal Performance (x = y)",
    )

    # Set labels and title
    ax.set_xlabel("Cycles for From-Scratch Traversal", fontsize=12, fontweight="bold")
    ax.set_ylabel("Cycles for Incremental Traversal", fontsize=12, fontweight="bold")
    ax.set_title(
        f"CSS Processing Performance Comparison\n({len(df)} benchmark runs)",
        fontsize=14,
        fontweight="bold",
        pad=20,
    )

    # Add grid
    ax.grid(True, alpha=0.3, which="both")

    # Set axis limits with some padding
    margin = 0.1
    min_limit = min_cycles * (1 - margin)
    max_limit = max_cycles * (1 + margin)
    ax.set_xlim(min_limit, max_limit)
    ax.set_ylim(min_limit, max_limit)

    # Add annotations about performance regions
    ax.text(
        0.02,
        0.98,
        "Incremental Faster\n(points below line)",
        transform=ax.transAxes,
        fontsize=10,
        verticalalignment="top",
        bbox={"boxstyle": "round", "facecolor": "lightgreen", "alpha": 0.7},
    )

    ax.text(
        0.98,
        0.02,
        "From-Scratch Faster\n(points above line)",
        transform=ax.transAxes,
        fontsize=10,
        verticalalignment="bottom",
        horizontalalignment="right",
        bbox={"boxstyle": "round", "facecolor": "lightcoral", "alpha": 0.7},
    )

    # Add performance statistics
    speedups = cast(npt.NDArray[np.float64], df["speedup"].values)
    avg_speedup = np.mean(speedups)
    faster_incremental = np.sum(speedups > 1.0)
    faster_fromscratch = np.sum(speedups < 1.0)

    stats_text = f"""Performance Statistics:
• Average Speedup: {avg_speedup:.2f}x
• Incremental Faster: {faster_incremental} cases
• From-Scratch Faster: {faster_fromscratch} cases
• Equal Performance: {len(df) - faster_incremental - faster_fromscratch} cases"""

    ax.text(
        0.02,
        0.5,
        stats_text,
        transform=ax.transAxes,
        fontsize=9,
        verticalalignment="center",
        bbox={"boxstyle": "round", "facecolor": "lightyellow", "alpha": 0.8},
    )

    # Make it look professional
    ax.tick_params(axis="both", which="major", labelsize=10)
    ax.tick_params(axis="both", which="minor", labelsize=8)

    # Tight layout and save
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches="tight", facecolor="white")
    print(f"Scatter plot saved to {output_file}")

    return fig, ax


def create_detailed_analysis(df: pd.DataFrame):
    """Create detailed analysis of the performance data."""
    print("\n" + "=" * 60)
    print("DETAILED PERFORMANCE ANALYSIS")
    print("=" * 60)

    # Basic statistics
    speedups = cast(npt.NDArray[np.float64], df["speedup"].values)
    print(f"Total benchmark runs: {len(df)}")
    print(f"Average speedup: {np.mean(speedups):.3f}x")
    print(f"Median speedup: {np.median(speedups):.3f}x")
    print(f"Min speedup: {np.min(speedups):.3f}x")
    print(f"Max speedup: {np.max(speedups):.3f}x")
    print(f"Std deviation: {np.std(speedups):.3f}")

    # Performance categories
    much_faster_inc = np.sum(speedups > 1.2)
    faster_inc = np.sum((speedups > 1.05) & (speedups <= 1.2))
    equal_perf = np.sum((speedups >= 0.95) & (speedups <= 1.05))
    faster_fs = np.sum((speedups >= 0.8) & (speedups < 0.95))
    much_faster_fs = np.sum(speedups < 0.8)

    print("\nPerformance Categories:")
    print(
        f"• Much faster incremental (>1.2x): {much_faster_inc} cases ({much_faster_inc / len(df) * 100:.1f}%)"
    )
    print(
        f"• Faster incremental (1.05-1.2x): {faster_inc} cases ({faster_inc / len(df) * 100:.1f}%)"
    )
    print(
        f"• Similar performance (0.95-1.05x): {equal_perf} cases ({equal_perf / len(df) * 100:.1f}%)"
    )
    print(
        f"• Faster from-scratch (0.8-0.95x): {faster_fs} cases ({faster_fs / len(df) * 100:.1f}%)"
    )
    print(
        f"• Much faster from-scratch (<0.8x): {much_faster_fs} cases ({much_faster_fs / len(df) * 100:.1f}%)"
    )

    # Test type analysis
    print("\nPerformance by Test Type:")
    for test_type in [
        "scale",
        "no_change",
        "single_leaf",
        "small_subtree",
        "realistic",
    ]:
        mask = df["test_name"].str.contains(test_type)
        if mask.any():
            subset_speedups = df[mask]["speedup"].values
            print(
                f"• {test_type}: {len(subset_speedups)} tests, avg speedup: {np.mean(subset_speedups):.3f}x"
            )

    # Node count analysis
    unique_node_counts = df["nodes_count"].unique()
    print("\nNode Count Analysis:")
    for node_count in sorted(unique_node_counts):
        mask = df["nodes_count"] == node_count
        subset_speedups = df[mask]["speedup"].values
        print(
            f"• {node_count} nodes: {len(subset_speedups)} tests, avg speedup: {np.mean(subset_speedups):.3f}x"
        )


def main():
    """Main function to generate scatter plot and analysis."""
    csv_file = "performance_benchmark.csv"

    # Load data
    df = load_benchmark_data(csv_file)
    if df is None:
        return

    # Create scatter plot
    fig, ax = create_scatter_plot(df)

    # Show detailed analysis
    create_detailed_analysis(df)

    # Display the plot
    plt.show()


if __name__ == "__main__":
    main()
