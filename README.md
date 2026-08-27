# CSS Bitvector Compiler

This repository evaluates incremental CSS selector matching over captured DOM
traces. CSS is compiled into typed state transitions once, then replayed with
several cache policies over one shared DOM implementation.

## Engines

- `naive`: an independent selector-chain oracle used for correctness checks.
- `bit`: cached output bitvectors with dirty-subtree traversal.
- `tri`: bitvectors plus zero/one/unused input requirements.
- `rec_tri`: the recursive-tri experiment, using the same typed runtime API.
- `quad`: retained as an experimental legacy target and excluded from defaults.

All active engines support descendant (` `), child (`>`), adjacent sibling
(`+`), equality attributes, `:hover`, `:focus`, `:focus-within`, `:first-child`,
`:first-of-type`, `:nth-child`, and `:nth-of-type`. Last/nth-last selectors,
`:has`, selector-list pseudos, general sibling (`~`), and pseudo-elements are
reported consistently as unsupported.

## Running

Run an engine against a checked-in site capture:

```sh
WEBSITE_NAME=google cargo run -r --bin bit
WEBSITE_NAME=google cargo run -r --bin tri
WEBSITE_NAME=google cargo run -r --bin rec_tri
WEBSITE_NAME=google cargo run -r --bin naive
```

The repository ships a typed development driver through Cargo:

```sh
cargo xtask check
cargo xtask corpus
cargo xtask run --site google
cargo xtask benchmark --site google
cargo xtask report
```

`cargo xtask run` compares the oracle, bit, tri, and recursive-tri outputs in
memory. Pass `--all` to check every captured site. With `--update`, it writes
one consolidated `results.json` per site; no per-engine logs or DOT copies are
created. Run `cargo xtask report` afterward to regenerate the single
`misscnt.html` summary.

Run repeated median-cycle benchmarks. Parsing, trace decoding, logging, DOT
generation, and report writing are outside the measured region.

```sh
cargo xtask benchmark --site google bit,tri,rec_tri
```

Inspect selector coverage for a CSS file:

```sh
cargo run --bin main -- path/to/input.css
```

Set `TRI_LOG_MATCH_DELTAS=1` for per-frame miss and match-change diagnostics.

## Validation

```sh
cargo xtask check
cargo xtask corpus
cargo xtask all
```

The ignored corpus test replays every checked-in trace and requires exact
final-match parity between the oracle, bit, tri, and recursive-tri engines.

The only remaining Python utilities are `css-gen-op/generate.py` and
`css-gen-op/common.py`, which convert raw browser captures into trace commands.
