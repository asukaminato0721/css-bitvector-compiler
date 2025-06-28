#!/usr/bin/env python3
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

def main():
    # Read the data
    df = pd.read_csv('web_layout_trace_benchmark.csv')
    
    # The 'speedup' column in the CSV is a ratio (incremental/full).
    # We will recalculate it to be a more intuitive speedup factor (full/incremental).
    # A small epsilon is added to incremental_cycles to prevent division by zero,
    # treating cases where incremental is zero as having a very high speedup.
    epsilon = 1e-9
    df['speedup'] = df['full_layout_cycles'] / (df['incremental_cycles'] + epsilon)
    # For cases where both are 0, speedup should be 1, not 0.
    df.loc[(df['full_layout_cycles'] == 0) & (df['incremental_cycles'] == 0), 'speedup'] = 1.0

    # Create the performance comparison plot
    plt.figure(figsize=(10, 10))
    
    # Color code by modification type
    colors = {
        'TreeInitialization': '#d62728',  # Red
        'LayoutRecalculation': '#1f77b4', # Blue
        'Insertion': '#2ca02c',           # Green
        'AttributeChange': '#ff7f0e'      # Orange
    }
    
    # Plot each modification type
    for mod_type in df['modification_type'].unique():
        mask = df['modification_type'] == mod_type
        plt.scatter(df[mask]['full_layout_cycles'], df[mask]['incremental_cycles'], 
                    c=colors.get(mod_type, 'gray'), 
                    label=mod_type, alpha=0.7, s=80, edgecolors='white', linewidth=0.5)
    
    # Get the range for the diagonal line
    min_cycles = min(df['full_layout_cycles'].min(), df['incremental_cycles'].min())
    max_cycles = max(df['full_layout_cycles'].max(), df['incremental_cycles'].max())
    
    # Add diagonal line (equal performance)
    diagonal_range = np.logspace(np.log10(min_cycles * 0.8), np.log10(max_cycles * 1.2), 100)
    plt.plot(diagonal_range, diagonal_range, 'k-', alpha=0.8, linewidth=2, label='Equal Performance')
    
    # Set log scale for both axes
    plt.xscale('log')
    plt.yscale('log')
    
    # Calculate geometric mean for display
    positive_speedups = df[df['speedup'] > 0]['speedup']
    geometric_mean_speedup = np.exp(np.log(positive_speedups).mean()) if len(positive_speedups) > 0 else 1.0
    
    # Set labels and title
    plt.xlabel('Cycles for Full Layout', fontsize=14, fontweight='bold')
    plt.ylabel('Cycles for Incremental Layout', fontsize=14, fontweight='bold')
    plt.title(f'Layout Engine Performance Comparison\n(Corrected Recalculate-based Benchmark, Geomean Speedup: {geometric_mean_speedup:.3f}x)', 
              fontsize=16, fontweight='bold', pad=20)
    
    # Add grid
    plt.grid(True, alpha=0.3, which='both')
    
    # Set axis limits with some padding
    plt.xlim(min_cycles * 0.8, max_cycles * 1.2)
    plt.ylim(min_cycles * 0.8, max_cycles * 1.2)
    
    # Add legend
    plt.legend(loc='upper left', frameon=True, fancybox=True, shadow=True)
    
    # Improve layout
    plt.tight_layout()
    
    # Save the plot
    plt.savefig('performance_comparison_scatter.png', dpi=300, bbox_inches='tight', 
                facecolor='white', edgecolor='none')
    print('✅ Performance comparison scatter plot saved to performance_comparison_scatter.png')
    
    # Print analysis
    print('\n' + '📊' + '='*60 + '📊')
    print('     LAYOUT ENGINE PERFORMANCE COMPARISON ANALYSIS')
    print('📊' + '='*60 + '📊')
    
    # Calculate statistics
    total_points = len(df)
    points_below_diagonal = len(df[df['incremental_cycles'] < df['full_layout_cycles']])
    points_above_diagonal = len(df[df['incremental_cycles'] > df['full_layout_cycles']])
    points_on_diagonal = total_points - points_below_diagonal - points_above_diagonal
    
    print(f'📈 Total data points: {total_points}')
    print(f'🟢 Incremental faster (below diagonal): {points_below_diagonal} ({100*points_below_diagonal/total_points:.1f}%)')
    print(f'🔴 Incremental slower (above diagonal): {points_above_diagonal} ({100*points_above_diagonal/total_points:.1f}%)')
    print(f'⚪ Equal performance (on diagonal): {points_on_diagonal} ({100*points_on_diagonal/total_points:.1f}%)')
    
    # Performance ratio analysis
    avg_speedup = df['speedup'].mean()
    median_speedup = df['speedup'].median()
    
    # Calculate geometric mean
    positive_speedups = df[df['speedup'] > 0]['speedup']
    geometric_mean_speedup = np.exp(np.log(positive_speedups).mean()) if len(positive_speedups) > 0 else 1.0
    
    print(f'\n⚡ Average speedup: {avg_speedup:.3f}x')
    print(f'📊 Median speedup: {median_speedup:.3f}x')
    print(f'📈 Geometric mean speedup: {geometric_mean_speedup:.3f}x')
    with open("./geometric_mean_speedup.txt", "a") as f:
        f.write(f'Geometric mean speedup: {geometric_mean_speedup:.3f}x\n')

    
    # Range analysis
    min_full = df['full_layout_cycles'].min()
    max_full = df['full_layout_cycles'].max()
    min_inc = df['incremental_cycles'].min()
    max_inc = df['incremental_cycles'].max()
    
    print(f'\n📏 Full layout cycles range: {min_full:,} - {max_full:,}')
    print(f'📏 Incremental layout cycles range: {min_inc:,} - {max_inc:,}')
    
    # Type-specific analysis
    print(f'\n🎯 Performance by Operation Type:')
    for mod_type in df['modification_type'].unique():
        subset = df[df['modification_type'] == mod_type]
        avg_ratio = subset['speedup'].mean()
        count = len(subset)
        faster_count = len(subset[subset['speedup'] > 1.0])
        print(f'  • {mod_type}: {count} ops, avg speedup {avg_ratio:.3f}x, faster in {faster_count}/{count} cases')
    
    print('\n📈 Methodology Notes:')
    print('  • Each point represents one layout operation frame')
    print('  • Points below diagonal = incremental layout is faster')
    print('  • Points above diagonal = full layout is faster') 
    print('  • Logarithmic scale shows performance across different workload sizes')
    print('  • Similar to academic browser engine performance analysis')
    
    print('\n' + '📊' + '='*60 + '📊')

if __name__ == '__main__':
    main() 