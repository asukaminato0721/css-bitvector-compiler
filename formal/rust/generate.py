#!/usr/bin/env python3
"""Regenerate Lean definitions from the production Rust crate."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "formal/rust"
CHARON_REV = "798016509139eb51f0d764645f76d068d34ba5f6"
AENEAS_REV = "d119a474f852781f8ae88a85424f2f8ba8407b82"
ENTRY_POINTS = [
    "clean::engine::Dom::mark",
    "clean::engine::Dom::insert_instance",
    "clean::engine::Dom::remove_subtree",
    "clean::engine::Dom::clear",
    "clean::engine::Dom::previous_sibling",
    "clean::logic::Requirement::accepts",
    "clean::logic::Requirement::from_bit",
    "clean::logic::specialize_or",
    "clean::logic::specialize_concrete",
    "clean::logic::requirements_accept",
]
GENERATED = ["CssMark/Types.lean", "CssMark/Funs.lean"]
# Container operations use an extensional map contract. The concrete Vec layout
# is outside these local algorithm proofs, just as HashMap buckets were before.
OPAQUE_CONTAINERS = ["crate::clean::engine::arena"]


def run(args):
    result = subprocess.run(args, cwd=ROOT, text=True, capture_output=True)
    output = re.sub(r"\x1b\[[0-?]*[ -/]*[@-~]", "", result.stdout + result.stderr)
    if result.returncode or "[Error" in output or "Generated the partial file" in output:
        raise SystemExit(output)
    return output


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def snapshot(generated):
    sources = [ROOT / "Cargo.toml", ROOT / "Cargo.lock", *sorted((ROOT / "src").rglob("*.rs"))]
    return {
        "charon": CHARON_REV,
        "aeneas": AENEAS_REV,
        "entry_points": ENTRY_POINTS,
        "aeneas_options": ["-split-files", "-loops-to-rec"],
        "opaque_containers": OPAQUE_CONTAINERS,
        "sources": {str(p.relative_to(ROOT)): digest(p) for p in sources},
        "generated": {name: digest(generated / name) for name in GENERATED},
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true", help="Regenerate and compare all outputs.")
    mode.add_argument("--check-snapshot", action="store_true", help="Check hashes without running the translators.")
    mode.add_argument("--check-scheduler-translation", action="store_true",
                      help="Test scheduler translation with an opaque evaluator. This does not check a proof.")
    args = parser.parse_args()
    manifest = OUT / "source-manifest.json"
    if args.check_snapshot:
        if json.loads(manifest.read_text()) != snapshot(OUT):
            raise SystemExit("Rust sources or generated definitions changed. Run generate.py with the pinned translators.")
        print("Source snapshot matches. This check does not run the translators.")
        return

    charon = os.environ.get("CHARON", "charon")
    aeneas = os.environ.get("AENEAS", "aeneas")
    if CHARON_REV not in run([charon, "version"]):
        raise SystemExit(f"Use Charon {CHARON_REV}.")
    if AENEAS_REV[:8] not in run([aeneas, "-version"]):
        raise SystemExit(f"Use Aeneas {AENEAS_REV}.")
    with tempfile.TemporaryDirectory(prefix="css-proof-") as directory:
        temp = Path(directory)
        llbc = temp / "css_mark.llbc"
        command = [charon, "cargo", "--preset=aeneas", "--sysroot", "default"]
        for container in OPAQUE_CONTAINERS:
            command += ["--opaque", container]
        entries = ["clean::engine::Engine::recompute_node"] if args.check_scheduler_translation else ENTRY_POINTS
        for entry in entries:
            command += ["--start-from", f"css_bitvector_compiler::{entry}"]
        if args.check_scheduler_translation:
            command += ["--opaque", "crate::clean::engine::_::evaluate"]
        run(command + ["--dest-file", str(llbc), "--", "--lib"])
        run([aeneas, "-backend", "lean", str(llbc), "-dest", str(temp / "CssMark"),
             "-namespace", "CssDom", "-split-files", "-loops-to-rec"])
        if args.check_scheduler_translation:
            print("Scheduler translation succeeds with an opaque evaluator. No scheduler proof was checked.")
            return
        new_manifest = snapshot(temp)
        if args.check:
            for name in GENERATED:
                if (OUT / name).read_bytes() != (temp / name).read_bytes():
                    raise SystemExit(f"Generated definition differs: {name}")
            if json.loads(manifest.read_text()) != new_manifest:
                raise SystemExit("Source manifest differs. Run generate.py.")
            print("Production Rust extraction matches the checked-in Lean definitions.")
        else:
            for name in GENERATED:
                (OUT / name).write_bytes((temp / name).read_bytes())
            manifest.write_text(json.dumps(new_manifest, indent=2) + "\n")
            print("Generated Lean definitions. Run lake build to check the proofs.")


if __name__ == "__main__":
    main()
