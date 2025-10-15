#!/usr/bin/env python3
from contextlib import redirect_stdout
import re, sys
from pathlib import Path


def main(base_dir: str = "css-gen-op"):
    base = Path(base_dir).resolve()
    rows: list[tuple[str, str, str, str]] = []
    for d in sorted(p for p in base.iterdir() if p.is_dir()):
        f = d / "bit_tmp.txt"
        f1 = d / "tri_tmp.txt"
        quad = d / "quad_tmp.txt"
        value = f.read_text(encoding="utf-8", errors="ignore").strip().splitlines()[-1].split(" = ")[-1]
        value1 = f1.read_text(encoding="utf-8", errors="ignore").strip().splitlines()[-3].split(" = ")[-1]
        value2 = quad.read_text(encoding="utf-8", errors="ignore").strip().splitlines()[-3].split(" = ")[-1]

        rows.append((d.name, value, value1, value2))
    with redirect_stdout(Path("./misscnt.md").open("w")):
        print(r"| Folder | MISS\_CNT | TRI MISS\_CNT | QUAD MISS\_CNT")
        print("|---|---:|---:|---:|")
        for name, v, v1, v2 in rows:
            print(f"| {name} | {v} | {v1} | {v2} |")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "css-gen-op")
