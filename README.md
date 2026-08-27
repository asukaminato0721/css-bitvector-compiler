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

Run the default correctness/report pipeline:

```sh
./run.sh
```

Run repeated median-cycle benchmarks. Parsing, trace decoding, logging, DOT
generation, and report writing are outside the measured region.

```sh
cargo run -r --bin benchmark -- --site google bit,tri,rec_tri
```

Inspect selector coverage for a CSS file:

```sh
cargo run --bin main -- path/to/input.css
```

Set `TRI_LOG_MATCH_DELTAS=1` for per-frame miss and match-change diagnostics.
Set `CSS_BV_NO_DOT=1` to suppress DOT output during ad-hoc validation.

## Validation

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test -r clean::tests::checked_in_corpus_has_engine_parity -- --ignored
```

The ignored corpus test replays every checked-in trace and requires exact
final-match parity between the oracle, bit, tri, and recursive-tri engines.
