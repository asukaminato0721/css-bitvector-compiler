#!/usr/bin/env python3
from contextlib import redirect_stdout
import re, sys
from pathlib import Path

def main(base_dir:str="css-gen-op"):
    base = Path(base_dir).resolve()
    rows:list[tuple[str,str,str]] = []
    for d in sorted(p for p in base.iterdir() if p.is_dir()):
        f = d / "bit_tmp.txt"
        f1 = d/"tri_tmp.txt"
        value = ""
        text = f.read_text(encoding="utf-8", errors="ignore")
        m = re.findall(r"MISS_CNT\s*\}\s*=\s*(\d+)", text)
        value = m[-1]  # take the last occurrence
        text1 = f1.read_text(encoding="utf-8", errors="ignore")
        value1 = ""
        m1 = re.findall(r"MISS_CNT\s*\}\s*=\s*(\d+)", text1)
        value1 = m1[-1]  # take the last occurrence
        rows.append((d.name, value, value1))
    f = Path("./misscnt.md")
    with redirect_stdout(f.open("w")):
        print("| Folder | MISS_CNT | NEW MISS_CNT")
        print("|---|---:|---:|")
        for name, v,v1 in rows:
            print(f"| {name} | {v} | {v1}")

if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "css-gen-op")