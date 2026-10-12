# CSS Bitvector Compiler

This repository evaluates incremental CSS selector matching over captured DOM
traces. CSS is compiled into typed state transitions once, then replayed with
several cache policies over one shared DOM implementation.

Detailed algorithm, Hoare-logic, and Kani verification notes are in
[docs/design.md](docs/design.md). A typeset copy is available as
[docs/design.pdf](docs/design.pdf) and can be rebuilt with `cargo xtask docs`.

[Lean 4 proofs](formal/README.md) cover selector recurrence, cache reuse,
mutation invalidation, and dirty traversal in a finite forest model.
The algorithm theorem compares initialization and every subsequent edit with full recomputation.
Algorithm verification is the proof target. Complete Rust implementation verification is outside that target.
[Source proofs](formal/rust/README.md) verify selected production Rust functions
translated by Charon and Aeneas. Coverage includes marking, allocation, deletion,
live sibling lookup, input guards, and local quad operations.
Full arena and scheduler refinement remains incomplete.
Lean is the main prover. Existing Kani checks remain. This project does not use Verus.

Build both proof packages from the repository root:

```sh
lake -d formal build
python3 formal/rust/generate.py --check-snapshot
lake -d formal/rust build
```

The snapshot check detects source changes. It does not run the Rust translators.

## Engines

- `naive`: an independent selector-chain oracle used for correctness checks.
- `bit`: cached output bitvectors with dirty-subtree traversal.
- `tri`: bitvectors plus zero/one/unused input requirements.
- `rec_tri`: the recursive-tri experiment, using the same typed runtime API.
- `quad`: compositional `0 / 1 / FromInput` outputs that avoid recomputing
  pure propagation nodes.

All active engines support descendant (` `), child (`>`), adjacent sibling
(`+`), equality attributes, `:hover`, `:focus`, `:focus-within`, `:first-child`,
`:first-of-type`, `:nth-child`, and `:nth-of-type`. Last/nth-last selectors,
`:has`, selector-list pseudos, general sibling (`~`), and pseudo-elements are
reported consistently as unsupported.

## Running

The repository ships a typed development driver through Cargo:

```sh
cargo xtask check
cargo xtask corpus
cargo xtask run --site google
cargo xtask benchmark --site google
cargo xtask stats --site google
cargo xtask report
cargo xtask verify
```

`cargo xtask run` compares the oracle, bit, tri, and recursive-tri outputs in
memory, including quad. Pass `--all` to check every captured site. With `--update`, it writes
one consolidated `results.json` per site; no per-engine logs or DOT copies are
created. Run `cargo xtask report` afterward to regenerate the single
`misscnt.html` summary.

Run repeated median-cycle benchmarks. Parsing, trace decoding, result
serialization, and report writing are outside the measured region.

```sh
cargo xtask benchmark --site google bit,tri,rec_tri,quad
```

Compare with the released `parcel_selectors` matcher on the same DOM and selector subset:

```sh
cargo xtask compare --all --samples 7 --output docs/benchmark-results.json
```

The comparison checks exact match sets after every frame before timing.
It reports initialization and trace replay separately. See [benchmark results](docs/benchmark.md) for the method and limits.
The [website data comparison](docs/website-benchmark.md) replays 11 captured sites through bit, tri, rec_tri, quad, and Stylo/Blitz. It uses normalized trees and synthetic colour declarations.
The [native Stylo comparison](docs/stylo-benchmark.md) measures incremental updates with the same timer and per-node output contract. The [earlier Blink measurements](docs/browser-benchmark.md) use different output contracts and do not establish an algorithm speed ratio.

Inspect selector coverage for a site or CSS file:

```sh
cargo xtask stats --site google
cargo xtask stats --css path/to/input.css
```

## Validation

```sh
cargo xtask check
cargo xtask corpus
cargo xtask all
```

The ignored corpus test replays every checked-in trace and requires exact
final-match parity between the oracle, bit, tri, recursive-tri, and quad engines.

The only remaining Python utilities are `css-gen-op/generate.py` and
`css-gen-op/common.py`, which convert raw browser captures into trace commands.
