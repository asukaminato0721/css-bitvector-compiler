#!/usr/bin/env python3
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np


def main():
    # Read the data
    df = pd.read_csv("web_layout_trace_benchmark.csv")

    df.replace([np.inf, -np.inf], np.nan, inplace=True)
    df.dropna(subset=["speedup"], inplace=True)

    fig, ax = plt.subplots(figsize=(10, 10))

    ax.scatter(
        x=df["bitvector_cycles"],
        y=df["trivector_cycles"],
        c="#1f77b4",  # Blue
        label="Recalculate",
        alpha=0.7,
        s=80,
        edgecolors="white",
        linewidth=0.5,
    )

    # Get the range for the diagonal line
    # Add a small constant to handle cases where min is 0 for log scale
    min_val = min(df["trivector_cycles"].min(), df["bitvector_cycles"].min())
    min_cycles = min_val if min_val > 0 else 1
    max_cycles = max(df["trivector_cycles"].max(), df["bitvector_cycles"].max())

    # Add diagonal line (equal performance)
    diagonal_range = np.logspace(
        np.log10(min_cycles * 0.8), np.log10(max_cycles * 1.2), 100
    )
    ax.plot(
        diagonal_range,
        diagonal_range,
        "k-",
        alpha=0.8,
        linewidth=2,
        label="Equal Performance",
    )

    # Set log scale for both axes
    ax.set_xscale("log")
    ax.set_yscale("log")

    # Calculate geometric mean for display
    positive_speedups = df[df["speedup"] > 0]["speedup"]
    geometric_mean_speedup = (
        np.exp(np.log(positive_speedups).mean()) if len(positive_speedups) > 0 else 1.0
    )

    # Set labels and title
    ax.set_xlabel("Cycles for BitVector Layout", fontsize=14, fontweight="bold")
    ax.set_ylabel(
        "Cycles for TriVector (IState) Layout", fontsize=14, fontweight="bold"
    )
    ax.set_title(
        f"BitVector vs TriVector Performance Comparison\n(TriVector Geomean Speedup: {geometric_mean_speedup:.3f}x)",
        fontsize=16,
        fontweight="bold",
        pad=20,
    )

    # Add grid
    ax.grid(True, alpha=0.3, which="both")

    # Set axis limits with some padding
    ax.set_xlim(min_cycles * 0.8, max_cycles * 1.2)
    ax.set_ylim(min_cycles * 0.8, max_cycles * 1.2)

    # Add legend
    fig.legend(loc="upper left", frameon=True, fancybox=True, shadow=True)

    # Improve layout
    fig.tight_layout()

    # Save the plot
    fig.savefig(
        "performance_comparison_scatter.png",
        dpi=300,
        bbox_inches="tight",
        facecolor="white",
        edgecolor="none",
    )
    print(
        "✅ Performance comparison scatter plot saved to performance_comparison_scatter.png"
    )

    # Print analysis
    print("\n" + "" + "=" * 60 + "")
    print("     BITVECTOR vs TRIVECTOR PERFORMANCE COMPARISON")
    print("" + "=" * 60 + "")

    # Calculate statistics
    total_points = len(df)
    points_below_diagonal = len(df[df["trivector_cycles"] < df["bitvector_cycles"]])
    points_above_diagonal = len(df[df["trivector_cycles"] > df["bitvector_cycles"]])
    points_on_diagonal = total_points - points_below_diagonal - points_above_diagonal

    print(f"📈 Total data points: {total_points}")
    print(
        f"🟢 TriVector faster (below diagonal): {points_below_diagonal} ({100 * points_below_diagonal / total_points:.1f}%)"
    )
    print(
        f"🔴 TriVector slower (above diagonal): {points_above_diagonal} ({100 * points_above_diagonal / total_points:.1f}%)"
    )
    print(
        f"⚪ Equal performance (on diagonal): {points_on_diagonal} ({100 * points_on_diagonal / total_points:.1f}%)"
    )

    # Performance ratio analysis
    avg_speedup = df["speedup"].mean()
    median_speedup = df["speedup"].median()

    # Calculate geometric mean
    positive_speedups = df[df["speedup"] > 0]["speedup"]
    geometric_mean_speedup = (
        np.exp(np.log(positive_speedups).mean()) if len(positive_speedups) > 0 else 1.0
    )

    print(f"\n Average speedup: {avg_speedup:.3f}x")
    print(f" Median speedup: {median_speedup:.3f}x")
    print(f" Geometric mean speedup: {geometric_mean_speedup:.3f}x")
    with open("./geometric_mean_speedup.txt", "a") as f:
        f.write(f"Geometric mean speedup: {geometric_mean_speedup:.3f}x\n")

    # Range analysis
    min_trivector = df["trivector_cycles"].min()
    max_trivector = df["trivector_cycles"].max()
    min_bitvector = df["bitvector_cycles"].min()
    max_bitvector = df["bitvector_cycles"].max()

    print(f"\n📏 TriVector cycles range: {min_trivector:,} - {max_trivector:,}")
    print(f"📏 BitVector cycles range: {min_bitvector:,} - {max_bitvector:,}")

    print("\n" + "" + "=" * 60 + "")
    plt.close(fig)


if __name__ == "__main__":
    main()
