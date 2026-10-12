# /// script
# dependencies = ["matplotlib>=3.9,<4"]
# ///
"""Check storage experiment results and draw elapsed-time comparisons."""
import hashlib
import json
from pathlib import Path
import statistics
import subprocess

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parents[1]
report = json.loads((ROOT/'docs/performance-results.json').read_text())
assert report['protocol'] == 'storage-ab-v1'
for manifest in report['manifests'].values():
    for path, digest in manifest['sources'].items():
        if path in report['script_sources']:
            content = report['script_sources'][path].encode()
        else:
            content = subprocess.check_output(['git','show',f'{report["baseline_revision"]}:{path}'],cwd=ROOT)
        assert hashlib.sha256(content).hexdigest() == digest, path
sites = report['results']
assert [site['site'] for site in sites] == ['bilibili', 'youtube', 'netflix']
for site in sites:
    for path, digest in site['input_hashes'].items():
        assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest() == digest, path
    assert all(result['full_match_bits_validated'] for result in site['validation'].values())
    for result in site['measurements']:
        assert len(result['samples']) == report['samples']
        for field, summary in result['summary'].items():
            values = [sample[field] for sample in result['samples']]
            assert summary == dict(median_ns=statistics.median(values), min_ns=min(values), max_ns=max(values))

cases = [('std',0), ('fx-internal',0), ('fx-all',0), ('vec-fx',0), ('vec-fx',4)]
labels = ['Standard', 'Fx: IDs + pseudos', 'Fx: all runtime tables', 'Vec nodes + Fx', 'Vec + Fx + other changes']
fig, axes = plt.subplots(2,3,figsize=(13,6.8),layout='constrained')
for col, site in enumerate(sites):
    by_case = {(m['storage'],m['variant']):m for m in site['measurements']}
    for row, field in enumerate(['update_ns', 'total_ns']):
        ax = axes[row,col]
        values = [by_case[case]['summary'][field] for case in cases]
        medians = [v['median_ns']/1e6 for v in values]
        errors = [[(v['median_ns']-v['min_ns'])/1e6 for v in values],
                  [(v['max_ns']-v['median_ns'])/1e6 for v in values]]
        ax.barh(range(len(cases)), medians, xerr=errors, capsize=3,
                color=['#64748b','#38bdf8','#0284c7','#16a34a','#f59e0b'])
        ax.set_yticks(range(len(cases)), labels=labels if col == 0 else ['']*len(cases))
        ax.invert_yaxis()
        ax.set_xlim(0,max(v['max_ns']/1e6 for v in values)*1.25)
        ax.grid(axis='x',alpha=.2)
        ax.set_axisbelow(True)
        ax.set_xlabel('Update (ms)' if row == 0 else 'Update + all-node RGBA output (ms)')
        if row == 0:
            ax.set_title(site['site'])
        for i, median in enumerate(medians):
            ax.text(values[i]['max_ns']/1e6+ax.get_xlim()[1]*.015, i, f'{median:,.0f}', va='center',fontsize=9)
fig.suptitle('Storage experiments: Bit engine, 3 samples, median and min–max\n'
             'Other changes: conditional pseudo refresh, lazy nth position, fast no-match output',fontsize=12)
out = ROOT/'docs/benchmark-plots'
fig.savefig(out/'performance-storage.svg')
svg = out / 'performance-storage.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines()) + '\n')
fig.savefig(out/'performance-storage.png',dpi=180)

lines = ['| 网站 | 阶段 | 标准表 | Fx：内部表 | Fx：全部运行时表 | Vec 节点 + Fx | Vec + Fx + 三项优化 |',
         '|---|---|---:|---:|---:|---:|---:|']
for site in sites:
    by_case = {(m['storage'],m['variant']):m for m in site['measurements']}
    for field, label in [('update_ns','更新'),('total_ns','更新 + 输出')]:
        values = [by_case[case]['summary'][field]['median_ns']/1e6 for case in cases]
        lines.append(f'| {site["site"]} | {label} | '+' | '.join(f'{value:,.1f}' for value in values)+' |')
(ROOT/'docs/performance-table.md').write_text('\n'.join(lines)+'\n')
