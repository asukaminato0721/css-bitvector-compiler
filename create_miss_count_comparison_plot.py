#!/usr/bin/env python3
from pathlib import Path
from typing import Optional
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import os
import re

MISS_CNT_RE = re.compile(r"MISS_CNT\s*}\s*=\s*(\d+)")


def extract_miss_count(path: Path) -> Optional[int]:
    if not path.exists():
        return None
    match = MISS_CNT_RE.search(path.read_text(encoding="utf-8", errors="ignore"))
    return int(match.group(1)) if match else None


def load_from_logs() -> pd.DataFrame:
    base = Path("css-gen-op")
    rows = []
    for case_dir in sorted(base.iterdir()):
        if not case_dir.is_dir():
            continue
        if case_dir.name == "reddit":
            continue
        bit = extract_miss_count(case_dir / "bit_tmp.txt")
        tri = extract_miss_count(case_dir / "tri_tmp.txt")
        if bit is None or tri is None:
            continue
        rows.append(
            {
                "case": case_dir.name,
                "bitvector_cache_misses": bit,
                "trivector_cache_misses": tri,
            }
        )   
    return pd.DataFrame(rows)


def main():
    # Read the data, falling back to css-gen-op logs if CSV missing
    df = load_from_logs()

    # Clean data - remove invalid values
    df.replace([np.inf, -np.inf], np.nan, inplace=True)
    df.dropna(subset=["bitvector_cache_misses", "trivector_cache_misses"], inplace=True)
    fig, ax = plt.subplots(figsize=(10, 10))

    # Create scatter plot
    ax.scatter(
        x=df["bitvector_cache_misses"],
        y=df["trivector_cache_misses"],
        c="#e74c3c",  # Red color for misses
        label="Cache Miss Comparison",
        alpha=0.7,
        s=80,
        edgecolors="white",
        linewidth=0.5,
    )

    # Get the range for the diagonal line
    # Add a small constant to handle cases where min is 0 for log scale
    min_val = min(
        df["trivector_cache_misses"].min(), df["bitvector_cache_misses"].min()
    )
    min_misses = max(min_val, 1)  # Ensure at least 1 for log scale
    max_misses = max(
        df["trivector_cache_misses"].max(), df["bitvector_cache_misses"].max()
    )

    # Add diagonal line (equal miss count)
    diagonal_range = np.logspace(
        np.log10(min_misses * 0.8), np.log10(max_misses * 1.2), 100
    )
    ax.plot(
        diagonal_range,
        diagonal_range,
        "k-",
        alpha=0.8,
        linewidth=2,
        label="Equal Miss Count",
    )

    # Set log scale for both axes if there's sufficient range
    if max_misses / min_misses > 10:
        ax.set_xscale("log")
        ax.set_yscale("log")

    # Calculate miss count ratio statistics
    df["miss_ratio"] = df["trivector_cache_misses"] / (
        df["bitvector_cache_misses"] + 1e-10
    )  # Avoid division by zero
    valid_ratios = df[df["miss_ratio"] > 0]["miss_ratio"]

    geometric_mean_miss_ratio = (
        np.exp(np.log(valid_ratios).mean()) if len(valid_ratios) > 0 else 1.0
    )

    # Set labels and title
    ax.set_xlabel("Cache Misses for BitVector Layout", fontsize=14, fontweight="bold")
    ax.set_ylabel(
        "Cache Misses for TriVector (IState) Layout", fontsize=14, fontweight="bold"
    )
    ax.set_title(
        f"BitVector vs TriVector Cache Miss Comparison\n(TriVector/BitVector Miss Ratio: {geometric_mean_miss_ratio:.3f}x)",
        fontsize=16,
        fontweight="bold",
        pad=20,
    )

    # Add grid
    ax.grid(True, alpha=0.3, which="both")

    # Set axis limits with some padding
    ax.set_xlim(min_misses * 0.8, max_misses * 1.2)
    ax.set_ylim(min_misses * 0.8, max_misses * 1.2)

    # Add legend
    fig.legend(loc="upper left", frameon=True, fancybox=True, shadow=True)

    # Improve layout
    fig.tight_layout()

    # Save the plot
    output_path = Path("css-gen-op") / "cache_miss_comparison_scatter.png"
    fig.savefig(
        output_path,
        dpi=300,
        bbox_inches="tight",
        facecolor="white",
        edgecolor="none",
    )
    print(f"Cache miss comparison scatter plot saved to {output_path}")

    # Calculate statistics
    total_points = len(df)
    points_below_diagonal = len(
        df[df["trivector_cache_misses"] < df["bitvector_cache_misses"]]
    )
    points_above_diagonal = len(
        df[df["trivector_cache_misses"] > df["bitvector_cache_misses"]]
    )
    points_on_diagonal = total_points - points_below_diagonal - points_above_diagonal

    print(f" Total data points: {total_points}")
    print(
        f" TriVector fewer misses (below diagonal): {points_below_diagonal} ({100 * points_below_diagonal / total_points:.1f}%)"
    )
    print(
        f" TriVector more misses (above diagonal): {points_above_diagonal} ({100 * points_above_diagonal / total_points:.1f}%)"
    )
    print(
        f" Equal misses (on diagonal): {points_on_diagonal} ({100 * points_on_diagonal / total_points:.1f}%)"
    )

    # Miss ratio analysis
    avg_miss_ratio = df["miss_ratio"].mean()
    median_miss_ratio = df["miss_ratio"].median()

    print(f"\n Average miss ratio (TriVector/BitVector): {avg_miss_ratio:.3f}x")
    print(f" Median miss ratio: {median_miss_ratio:.3f}x")
    print(f" Geometric mean miss ratio: {geometric_mean_miss_ratio:.3f}x")

    # Save geometric mean to file
    with open("./geometric_mean_miss_ratio.txt", "w") as f:
        f.write(f"Geometric mean miss ratio: {geometric_mean_miss_ratio:.3f}x\n")

    # Range analysis
    min_trivector_misses = df["trivector_cache_misses"].min()
    max_trivector_misses = df["trivector_cache_misses"].max()
    min_bitvector_misses = df["bitvector_cache_misses"].min()
    max_bitvector_misses = df["bitvector_cache_misses"].max()

    print(
        f"\n TriVector miss range: {min_trivector_misses:,} - {max_trivector_misses:,}"
    )
    print(f" BitVector miss range: {min_bitvector_misses:,} - {max_bitvector_misses:,}")

    # Total miss counts
    total_bitvector_misses = df["bitvector_cache_misses"].sum()
    total_trivector_misses = df["trivector_cache_misses"].sum()

    print(f"\nBitVector total misses: {total_bitvector_misses:,}")
    print(f"TriVector total misses: {total_trivector_misses:,}")

    if {"bitvector_cache_hits", "trivector_cache_hits"}.issubset(df.columns):
        total_bitvector_hits = df["bitvector_cache_hits"].sum()
        total_trivector_hits = df["trivector_cache_hits"].sum()

        bitvector_miss_rate = (
            total_bitvector_misses
            / (total_bitvector_misses + total_bitvector_hits)
            * 100
        )
        trivector_miss_rate = (
            total_trivector_misses
            / (total_trivector_misses + total_trivector_hits)
            * 100
        )

        print(
            f" BitVector miss rate: {bitvector_miss_rate:.1f}% (hits: {total_bitvector_hits:,})"
        )
        print(
            f" TriVector miss rate: {trivector_miss_rate:.1f}% (hits: {total_trivector_hits:,})"
        )

    print("\n" + "=" * 60)
    plt.close(fig)


if __name__ == "__main__":
    main()
