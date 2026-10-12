"""Compare the isolated storage variants on captured website traces.

Build all four variants with analyze-performance.py first.
This driver validates all match bits against an unchanged reference engine.
Timed processes use Bit. Each process creates a new engine and DOM.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import platform
import statistics
import subprocess

ROOT = Path(__file__).resolve().parents[1]
OUT = Path('/tmp/css-performance-analysis')
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--sites', nargs='+', default=['bilibili', 'youtube', 'netflix'])
parser.add_argument('--samples', type=int, default=3)
parser.add_argument('--cpu', type=int, default=4)
parser.add_argument('--output', type=Path, default=ROOT/'docs/performance-results.json')
parser.add_argument('--storage', nargs='+', choices=['std', 'fx-internal', 'fx-all', 'vec-std', 'vec-fx'],
                    default=['std', 'fx-internal', 'fx-all', 'vec-fx'])
parser.add_argument('--variants', nargs='+', type=int, choices=range(5), default=[0,4])
args = parser.parse_args()
assert args.samples > 0 and args.samples % 2 == 1
storage = args.storage
manifests = {name: json.loads((OUT/name/'source-manifest.json').read_text()) for name in storage}
for name, manifest in manifests.items():
    assert hashlib.sha256((OUT/name/'profile').read_bytes()).hexdigest() == manifest['binary_sha256']
    for path, digest in manifest['sources'].items():
        content = (ROOT/path).read_bytes()
        if hashlib.sha256(content).hexdigest() != digest:
            content = subprocess.check_output(['git','show',f'{manifest["baseline_revision"]}:{path}'],cwd=ROOT)
        assert hashlib.sha256(content).hexdigest() == digest, path
report = dict(protocol='storage-ab-v1', captured_at_utc=datetime.now(timezone.utc).isoformat(),
              cpu_affinity=args.cpu, platform=platform.platform(),
              rustc=subprocess.check_output(['rustc','--version'], text=True).strip(),
              warmups=1, samples=args.samples, manifests=manifests,
              timed_engine='bit', validation_engines=['bit','tri','rec_tri','quad'],
              initial_frame_timed=False, results=[])
report['baseline_revision'] = manifests[storage[0]]['baseline_revision']
report['script_sources'] = {name:(ROOT/name).read_text() for name in ['dev/analyze-performance.py','dev/performance-arena.rs','dev/measure-performance.py']}
report['cpu'] = next(line.split(':',1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines() if line.startswith('model name'))
governor = Path(f'/sys/devices/system/cpu/cpu{args.cpu}/cpufreq/scaling_governor')
report['cpu_governor'] = governor.read_text().strip() if governor.exists() else None

def run(name, site, *options):
    command = ['taskset','-c',str(args.cpu),str(OUT/name/'profile'),'--site',site,*options]
    result = subprocess.run(command, cwd=ROOT, text=True, stdout=subprocess.PIPE, check=True)
    return json.loads(result.stdout)

def save():
    args.output.write_text(json.dumps(report, indent=2)+'\n')

for site in args.sites:
    data = dict(site=site, input_hashes={}, validation={}, execution_order=[], measurements=[])
    for path in [ROOT/f'css-gen-op/{site}/{site}.css', ROOT/f'css-gen-op/{site}/command.json']:
        data['input_hashes'][str(path.relative_to(ROOT))] = hashlib.sha256(path.read_bytes()).hexdigest()
    for name in storage:
        print(f'{site}: validate {name}', flush=True)
        data['validation'][name] = run(name, site, '--validate-only')
    cases = [(name, variant) for variant in args.variants for name in storage]
    values = {case: [] for case in cases}
    for round in range(args.samples + 1):
        offset = round % len(cases)
        order = cases[offset:] + cases[:offset]
        data['execution_order'].append(order)
        for name, variant in order:
            sample = run(name, site, '--profile-variant', str(variant))
            sample['total_ns'] = sample['update_ns'] + sample['output_ns']
            print(f'{site} {round}/{args.samples} {name}/{variant}: '
                  f'{sample["update_ns"]/1e6:.3f} + {sample["output_ns"]/1e6:.3f} ms', flush=True)
            if round:
                values[name,variant].append(sample)
    for (name,variant), samples in values.items():
        summary = {field: dict(median_ns=statistics.median(s[field] for s in samples),
                              min_ns=min(s[field] for s in samples),
                              max_ns=max(s[field] for s in samples))
                   for field in ['update_ns','output_ns','total_ns']}
        data['measurements'].append(dict(storage=name, variant=variant, summary=summary, samples=samples))
    report['results'].append(data)
    save()
