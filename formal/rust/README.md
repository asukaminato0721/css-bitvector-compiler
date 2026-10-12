# Rust source proofs

This package checks Lean definitions translated from the production Cargo crate.
It uses Lean 4.31.0 and the Aeneas Lean backend.
Charon and Aeneas translate the program. Lean checks the proofs.
The project does not need a third prover such as Verus.

## Coverage

| File | Checked property | Preconditions |
|---|---|---|
| `Refinement.lean` | Dirty merge and requirement operations match the algorithm model | None |
| `Refinement.lean` | Quad projection, OR specialization, and concrete acceptance match the model | Slice entries represent model inputs. Projection indexes are valid |
| `GuardRefinement.lean` | `requirements_accept` returns true exactly when lengths agree and all requirements accept | None beyond valid Rust slice lengths |
| `MarkRefinement.lean` | `Dom::mark` terminates and updates the ancestor chain exactly | Each stored parent ID is smaller than its child ID |
| `MarkRefinement.lean` | Marking preserves links, facts, caches, the trace-ID map, allocation counter, and parent-order invariant | Same parent-order condition |
| `ArenaRefinement.lean` | Allocation uses a fresh ID or reports counter exhaustion. It preserves freshness and parent order | All live IDs are below the counter. The new parent is absent or live. Existing parent order holds |
| `ArenaRefinement.lean` | Clearing removes all nodes and trace bindings but retains the counter | None |
| `RemoveRefinement.lean` | Recursive deletion terminates, removes the target, and preserves surviving records and ID invariants | Every child slot has a larger ID than its owner. Preserved invariants hold initially |
| `RemoveRefinement.lean` | Recursive deletion retains each trace binding whose target survives | Same child-ID order condition |
| `SiblingRefinement.lean` | `previous_sibling` implements a scan for the preceding live slot and ignores obsolete slots | None beyond valid Rust vector lengths |

The guard proof covers arbitrary slice lengths, early rejection, index bounds,
integer addition, and loop termination. It rejects unequal lengths.
`input_rep_of_arrays` discharges the quad input relation from equal lengths and matching entries.

`CssMark/Types.lean` and `CssMark/Funs.lean` are generated files.
Do not edit them by hand. `generate.py` selects the production entry points.
No separate Rust copy serves as the verification target.

## Build

From the repository root, run:

```sh
lake -d formal build
python3 formal/rust/generate.py --check-snapshot
lake -d formal/rust build
```

`SourceAudit.lean` checks all source-proof declarations and their axiom dependencies.
It permits only `propext`, `Quot.sound`, and `Classical.choice`.
It rejects `sorryAx`, additional axioms, and native evaluation proof axioms.
Warnings fail the build.

The CI source job checks the snapshot and builds these proofs.
The snapshot check detects changed Rust or generated files. It does not execute the translators.

## Regenerate from Rust

Use these pinned tools:

- Charon: `798016509139eb51f0d764645f76d068d34ba5f6`.
- Aeneas: `d119a474f852781f8ae88a85424f2f8ba8407b82`.
- Charon Rust toolchain: `nightly-2026-09-17`, with `rustc-dev` and `rust-src`.

The Aeneas revision is also fixed in `lakefile.lean` and `lake-manifest.json`.
Build the translator binaries from those revisions using their repository instructions.
Set the binary paths, then generate and check the definitions:

```sh
export CHARON=/absolute/path/to/charon
export AENEAS=/absolute/path/to/aeneas
python3 formal/rust/generate.py
lake -d formal/rust build
python3 formal/rust/generate.py --check
```

`--check` runs both translators and compares their output with the checked-in definitions.
The generator rejects version mismatches, translation errors, and partial output.
It copies only generated types and functions. It does not copy templates containing opaque declarations.
`source-manifest.json` records source and generated-file hashes.
The manifest detects drift. Hashes do not establish semantic correctness.
The `-loops-to-rec` option translates loops to recursive Lean functions.
The proofs establish termination for the selected entry points.

To test translation of the scheduler alone, run:

```sh
python3 formal/rust/generate.py --check-scheduler-translation
```

This probe treats `Engine::evaluate` as opaque. It checks translator acceptance only.
It does not check a scheduler proof or copy its generated files into this package.

## Trust boundary and remaining work

Lean checks the translated semantics. The connection to Rust depends on Charon and Aeneas.
The proof also depends on the Aeneas standard-library models.

`TypesExternal.lean` models a HashMap as a function from keys to optional values.
`FunsExternal.lean` supplies lookup and mutation through that interface.
Hash state is unobservable in this model. Allocation, hash collisions, and standard-library internals are outside the proof.
The map model supports the homogeneous keys used by the selected entry points.
Unsupported heterogeneous lookup returns failure.
These definitions contain no proof axioms, but their agreement with the Rust library is an explicit trust boundary.

Marking, allocation, clearing, and deletion preserve their stated ID invariants.
Allocation requires all live IDs to be below the next counter value.
Deletion requires increasing child IDs, including obsolete slots.
These source proofs do not yet establish that the full constructors maintain all arena invariants.
They do not establish node ownership or agreement between parent links and child lists.
Deletion preserves the root field. The caller must update that field when it removes the root.

The forest mutation and traversal proofs are in `../CssProofs/Incremental.lean`.
They do not yet establish refinement of `Engine::recompute_node` or every actual mutation.
The source proof now covers `previous_sibling`.
Scheduler translation succeeds with an opaque evaluator, but full evaluator translation remains unsupported.
The project also needs evaluator-loop, cache-provenance, ownership, and pseudo-update proofs.
No theorem in this package claims end-to-end correctness of the Rust engine.
