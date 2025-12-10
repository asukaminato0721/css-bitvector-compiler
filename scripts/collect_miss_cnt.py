#!/usr/bin/env python3
from contextlib import redirect_stdout
import re, sys
from pathlib import Path
from typing import Iterable, Optional, Tuple


def load_sorted_rules(path: Path) -> Optional[Tuple[str, ...]]:
    try:
        lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
    except FileNotFoundError:
        return None
    collecting = False
    body: list[str] = []
    for line in lines:
        if "BEGIN" in line:
            collecting = True
            continue
        if "END" in line:
            collecting = False
            continue
        if collecting:
            body.append(line.rstrip("\n"))
    return tuple(sorted(body))


def diff_status(reference: Optional[Tuple[str, ...]], target: Optional[Tuple[str, ...]]) -> str:
    if reference is None or target is None:
        return "MISSING"
    return "OK" if reference == target else "DIFF"


def main(base_dir: str = "css-gen-op"):
    base = Path(base_dir).resolve()
    rows: list[tuple[str, str, str, str, str, str, str]] = []
    for d in sorted(p for p in base.iterdir() if p.is_dir()):
        if d.name == "reddit":
            continue
        f = d / "bit_tmp.txt"
        f1 = d / "tri_tmp.txt"
        quad = d / "quad_tmp.txt"
        naive = d / "tmp.txt"

        value = (
            f.read_text(encoding="utf-8", errors="ignore")
            .strip()
            .splitlines()[-1]
            .split(" = ")[-1]
        )
        value1 = (
            f1.read_text(encoding="utf-8", errors="ignore")
            .strip()
            .splitlines()[-3]
            .split(" = ")[-1]
        )
        value2 = (
            quad.read_text(encoding="utf-8", errors="ignore")
            .strip()
            .splitlines()[-3]
            .split(" = ")[-1]
        )

        baseline = load_sorted_rules(naive)
        bit_rules = load_sorted_rules(f)
        tri_rules = load_sorted_rules(f1)
        quad_rules = load_sorted_rules(quad)
        rows.append(
            (
                d.name,
                value,
                value1,
                value2,
                diff_status(baseline, bit_rules),
                diff_status(baseline, tri_rules),
                diff_status(baseline, quad_rules),
            )
        )
    with redirect_stdout(Path("./misscnt.md").open("w")):
        print(
            r"| Folder | MISS\_CNT | TRI MISS\_CNT | QUAD MISS\_CNT | bit vs tmp | tri vs tmp | quad vs tmp |"
        )
        print("|---|---:|---:|---:|:---:|:---:|:---:|")
        for name, v, v1, v2, bit_state, tri_state, quad_state in rows:
            print(f"| {name} | {v} | {v1} | {v2} | {bit_state} | {tri_state} | {quad_state} |")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "css-gen-op")
