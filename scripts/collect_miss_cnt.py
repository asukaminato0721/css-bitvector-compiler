#!/usr/bin/env python3
from contextlib import redirect_stdout
import re, sys
from pathlib import Path
from typing import Iterable, Optional, Tuple


MISS_RE = re.compile(r"MISS_CNT\s*\}\s*=\s*(\d+)")


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


def miss_cnt_from_log(path: Path) -> str:
    try:
        text = path.read_text(encoding="utf-8", errors="ignore")
    except FileNotFoundError:
        return "-"
    matches = MISS_RE.findall(text)
    return matches[-1] if matches else "-"


def main(base_dir: str = "css-gen-op"):
    base = Path(base_dir).resolve()
    rows = []
    for d in sorted(p for p in base.iterdir() if p.is_dir()):
        if d.name == "reddit":
            continue
        f = d / "bit_tmp.txt"
        f1 = d / "tri_tmp.txt"
        rec_tri = d / "rec_tri_tmp.txt"
        naive = d / "tmp.txt"

        value = miss_cnt_from_log(f)
        value1 = miss_cnt_from_log(f1)
        value3 = miss_cnt_from_log(rec_tri)

        baseline = load_sorted_rules(naive)
        bit_rules = load_sorted_rules(f)
        tri_rules = load_sorted_rules(f1)
        rec_tri_rules = load_sorted_rules(rec_tri)
        rows.append(
            (
                d.name,
                value,
                value1,
                value3,
                diff_status(baseline, bit_rules),
                diff_status(baseline, tri_rules),
                diff_status(baseline, rec_tri_rules),
            )
        )
    with redirect_stdout(Path("./misscnt.md").open("w")):
        print(
            r"| Folder | MISS\_CNT | TRI MISS\_CNT | REC\_TRI MISS\_CNT | bit vs tmp | tri vs tmp | rec_tri vs tmp |"
        )
        print("|---|---:|---:|---:|---:|:---:|:---:|:---:|:---:|")
        for name, v, v1, v2, v3, bit_state, tri_state, rec_state in rows:
            print(
                f"| {name} | {v} | {v1} | {v2} | {v3} | {bit_state} | {tri_state} | {rec_state} |"
            )


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "css-gen-op")
