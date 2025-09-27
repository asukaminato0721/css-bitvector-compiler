#!/usr/bin/env python3
import sys
from pathlib import Path
import difflib
from typing import List, Dict

SUBDIRS = ["amazon", "google", "testcase", "youtube"]
FILES = ["tmp.txt", "tri_tmp.txt", "bit_tmp.txt", "quad_tmp.txt"]

def extract_between_markers(text: str):
    """Extract lines between the first BEGIN and the next END (exclusive)."""
    i = text.find("BEGIN")
    i += len("BEGIN")
    j = text.find("END", i)
    block = text[i:j].strip()
    # Ensure each line ends with a newline for difflib
    return sorted(line.strip() + ("\n" if not line.endswith(("\n", "\r")) else "") for line in block.splitlines())

def process_dir(dir_path: Path) -> bool:
    """Return True if any diff found in this dir."""
    paths = {name: dir_path / name for name in FILES}

    contents: Dict[str, List[str]] = {}
    for name, p in paths.items():
        try:
            text = p.read_text(encoding="utf-8", errors="replace")
            contents[name] = extract_between_markers(text)
        except Exception as e:
            sys.stderr.write(f"[{dir_path.name}] {name}: {e}\n")
            return True

    base_name = "tmp.txt"
    baseline = contents[base_name]
    any_diff = False
    for name in ["tri_tmp.txt", "bit_tmp.txt", "quad_tmp.txt"]:
        target = contents[name]
        diff = list(difflib.unified_diff(
            baseline, target,
            fromfile=str(dir_path / base_name),
            tofile=str(dir_path / name),
            lineterm=""
        ))
        print(f"-- {base_name} vs {name} --")
        if diff:
            any_diff = True
            print("\n".join(diff))
        else:
            print("(no differences)")
        print()
    return any_diff

def main():
    # Run from repo root, inspect css-gen-op/{amazon,google,testcase,youtube}
    repo_root = Path.cwd()
    css_root = repo_root / "css-gen-op"

    any_diff_all = False
    for sub in SUBDIRS:
        dir_path = css_root / sub
        if not dir_path.is_dir():
            sys.stderr.write(f"Directory not found: {dir_path}\n")
            any_diff_all = True
            continue
        if process_dir(dir_path):
            any_diff_all = True

    sys.exit(1 if any_diff_all else 0)

if __name__ == "__main__":
    main()
