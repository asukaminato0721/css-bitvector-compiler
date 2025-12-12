#!/usr/bin/env python3
"""Aggregate miss count data into markdown and HTML reports."""

from __future__ import annotations

from contextlib import redirect_stdout
from dataclasses import dataclass, asdict
from datetime import datetime, timezone
import html
import json
import re
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple


MISS_RE = re.compile(r"MISS_CNT\s*\}\s*=\s*(\d+)")
STATUS_CLASS = {
    "OK": "status-ok",
    "DIFF": "status-diff",
    "MISSING": "status-missing",
}


@dataclass
class ReportRow:
    """Normalized row for reporting output."""

    folder: str
    miss_cnt_bit: str
    miss_cnt_tri: str
    miss_cnt_rec: str
    bit_state: str
    tri_state: str
    rec_state: str

    def as_tuple(self) -> Tuple[str, ...]:
        return (
            self.folder,
            self.miss_cnt_bit,
            self.miss_cnt_tri,
            self.miss_cnt_rec,
            self.bit_state,
            self.tri_state,
            self.rec_state,
        )


def parse_miss_count(value: str) -> Optional[int]:
    """Convert stored miss count values into integers when possible."""

    value = value.strip()
    if not value or not value.isdigit():
        return None
    try:
        return int(value)
    except ValueError:
        return None


def summarize_states(rows: List[ReportRow], attr: str) -> Dict[str, int]:
    """Count occurrences of status values for a given attribute name."""

    totals: Dict[str, int] = {}
    for row in rows:
        state = getattr(row, attr)
        totals[state] = totals.get(state, 0) + 1
    return totals


def load_sorted_rules(path: Path) -> Optional[Tuple[str, ...]]:
    try:
        lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
    except FileNotFoundError:
        return None
    collecting = False
    body: list[str] = []
    saw_begin = False
    for line in lines:
        if "BEGIN" in line:
            collecting = True
            saw_begin = True
            continue
        if "END" in line:
            collecting = False
            continue
        if collecting:
            body.append(line.rstrip("\n"))
    if not saw_begin:
        return None
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


def collect_rows(base: Path) -> List[ReportRow]:
    """Gather per-site statistics from the trace folders."""

    if not base.exists():
        raise SystemExit(f"Base directory {base} does not exist")

    rows: List[ReportRow] = []
    for d in sorted(p for p in base.iterdir() if p.is_dir()):
        if d.name == "reddit":
            continue
        bit_log = d / "bit_tmp.txt"
        tri_log = d / "tri_tmp.txt"
        rec_log = d / "rec_tri_tmp.txt"
        baseline_log = d / "tmp.txt"

        bit_value = miss_cnt_from_log(bit_log)
        tri_value = miss_cnt_from_log(tri_log)
        rec_value = miss_cnt_from_log(rec_log)

        baseline = load_sorted_rules(baseline_log)
        bit_rules = load_sorted_rules(bit_log)
        tri_rules = load_sorted_rules(tri_log)
        rec_rules = load_sorted_rules(rec_log)

        rows.append(
            ReportRow(
                folder=d.name,
                miss_cnt_bit=bit_value,
                miss_cnt_tri=tri_value,
                miss_cnt_rec=rec_value,
                bit_state=diff_status(baseline, bit_rules),
                tri_state=diff_status(baseline, tri_rules),
                rec_state=diff_status(baseline, rec_rules),
            )
        )
    return rows


def write_markdown(rows: List[ReportRow], output_path: Path = Path("./misscnt.md")) -> None:
    """Emit the original markdown summary without modifying its format."""

    with redirect_stdout(output_path.open("w")):
        print(
            r"| Folder | MISS\_CNT | TRI MISS\_CNT | REC\_TRI MISS\_CNT | bit vs tmp | tri vs tmp | rec_tri vs tmp |"
        )
        print("|---|---:|---:|---:|:---:|:---:|:---:|")
        for row in rows:
            print(
                f"| {row.folder} | {row.miss_cnt_bit} | {row.miss_cnt_tri} | {row.miss_cnt_rec} | {row.bit_state} | {row.tri_state} | {row.rec_state} |"
            )


def render_status_cell(state: str) -> str:
    css_class = STATUS_CLASS.get(state, "status-unknown")
    return f'<span class="status-pill {css_class}">{html.escape(state)}</span>'


def aggregate_numeric(rows: List[ReportRow], attr: str) -> Optional[Tuple[int, int]]:
    values = [
        parse_miss_count(getattr(row, attr))
        for row in rows
        if getattr(row, attr, "-") not in {"-", ""}
    ]
    filtered = [v for v in values if v is not None]
    if not filtered:
        return None
    return sum(filtered), len(filtered)


def render_summary_section(rows: List[ReportRow]) -> str:
    """Build the HTML summary grid."""

    if not rows:
        return "<p>No data rows found.</p>"

    status_blocks = []
    for label, attr in (
        ("bit vs tmp", "bit_state"),
        ("tri vs tmp", "tri_state"),
        ("rec_tri vs tmp", "rec_state"),
    ):
        totals = summarize_states(rows, attr)
        lines = "".join(
            f'<div class="summary-line"><span>{html.escape(state)}</span><strong>{count}</strong></div>'
            for state, count in sorted(totals.items())
        )
        status_blocks.append(
            f'<div class="summary-card"><div class="summary-title">{html.escape(label)}</div>{lines}</div>'
        )

    count_cards = []
    for label, attr in (
        ("bit miss count", "miss_cnt_bit"),
        ("tri miss count", "miss_cnt_tri"),
        ("rec_tri miss count", "miss_cnt_rec"),
    ):
        aggregated = aggregate_numeric(rows, attr)
        if not aggregated:
            continue
        total, count = aggregated
        count_cards.append(
            (
                '<div class="summary-card total-card">'
                f'<div class="summary-title">{html.escape(label)}</div>'
                f'<div class="summary-total">{total:,}</div>'
                f'<div class="summary-note">across {count} site(s)</div>'
                "</div>"
            )
        )

    summary_html = "".join(status_blocks)
    totals_html = "".join(count_cards)
    return (
        f'<div class="summary-grid">{summary_html}</div>'
        + (f'<div class="summary-grid totals">{totals_html}</div>' if totals_html else "")
    )


def dataset_json_payload(rows: List[ReportRow]) -> str:
    """Serialize rows into a JSON string safe for embedding in HTML."""

    payload = json.dumps([asdict(r) for r in rows], indent=2)
    # Prevent closing the surrounding script tag accidentally.
    return payload.replace("</", "<\\/")


STYLE = """
:root {
  color-scheme: light dark;
}
* {
  box-sizing: border-box;
}
body {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  margin: 0;
  padding: 2rem;
  background: #f8fafc;
  color: #0f172a;
}
header h1 {
  margin: 0;
  font-size: 1.75rem;
}
header p {
  margin: 0.25rem 0 0;
  color: #475569;
}
.summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 1rem;
  margin-top: 1.5rem;
}
.summary-card {
  background: #ffffff;
  border-radius: 12px;
  padding: 1rem;
  box-shadow: 0 8px 20px rgba(15, 23, 42, 0.08);
  border: 1px solid #e2e8f0;
}
.summary-card.total-card {
  background: #0f172a;
  color: #f8fafc;
}
.summary-title {
  font-size: 0.95rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: #64748b;
  margin-bottom: 0.5rem;
}
.total-card .summary-title {
  color: #cbd5f5;
}
.summary-line {
  display: flex;
  justify-content: space-between;
  font-weight: 600;
  padding: 0.15rem 0;
  color: #0f172a;
}
.total-card .summary-line {
  color: inherit;
}
.summary-total {
  font-size: 1.75rem;
  font-weight: 700;
}
.summary-note {
  font-size: 0.85rem;
  color: #94a3b8;
}
.table-wrapper {
  margin-top: 2rem;
  overflow-x: auto;
}

.scatter-section {
  margin-top: 2rem;
  padding: 1.5rem;
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 10px 24px rgba(15, 23, 42, 0.08);
  border: 1px solid #e2e8f0;
}
.scatter-section h2 {
  margin-top: 0;
  font-size: 1.25rem;
}
.scatter-section figure {
  margin: 0;
  text-align: center;
}
.scatter-section img {
  width: 100%;
  max-height: 460px;
  object-fit: contain;
  border-radius: 8px;
  border: 1px solid #e2e8f0;
  background: #f8fafc;
}
.scatter-section figcaption {
  margin-top: 0.5rem;
  color: #64748b;
  font-size: 0.9rem;
}
table {
  width: 100%;
  border-collapse: collapse;
  background: #ffffff;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 12px 30px rgba(15, 23, 42, 0.1);
}
thead th {
  background: #0f172a;
  color: #f8fafc;
  text-align: left;
  padding: 0.75rem 1rem;
  font-size: 0.85rem;
  letter-spacing: 0.05em;
  text-transform: uppercase;
}
tbody td {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid #e2e8f0;
}
tbody tr:nth-child(even) {
  background: #f8fafc;
}
tbody tr:hover {
  background: #eef2ff;
}
td.num {
  text-align: right;
  font-variant-numeric: tabular-nums;
}
.status-pill {
  display: inline-flex;
  align-items: center;
  padding: 0.1rem 0.6rem;
  border-radius: 999px;
  font-weight: 600;
  font-size: 0.85rem;
}
.status-ok {
  background: #dcfce7;
  color: #166534;
}
.status-diff {
  background: #fee2e2;
  color: #b91c1c;
}
.status-missing {
  background: #fefce8;
  color: #854d0e;
}
.status-unknown {
  background: #e2e8f0;
  color: #0f172a;
}
td.empty {
  text-align: center;
  padding: 2rem;
  color: #94a3b8;
  font-style: italic;
}
"""


def write_html(rows: List[ReportRow], base_dir: Path, output_path: Path = Path("./misscnt.html")) -> None:
    """Render a richer HTML report alongside the markdown file."""

    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%SZ")
    table_rows = (
        "\n".join(
            (
                "<tr>"
                f"<td>{html.escape(row.folder)}</td>"
                f"<td class=\"num\">{html.escape(row.miss_cnt_bit)}</td>"
                f"<td class=\"num\">{html.escape(row.miss_cnt_tri)}</td>"
                f"<td class=\"num\">{html.escape(row.miss_cnt_rec)}</td>"
                f"<td>{render_status_cell(row.bit_state)}</td>"
                f"<td>{render_status_cell(row.tri_state)}</td>"
                f"<td>{render_status_cell(row.rec_state)}</td>"
                "</tr>"
            )
            for row in rows
        )
        if rows
        else '<tr><td class="empty" colspan="7">No data rows found.</td></tr>'
    )
    summary_html = render_summary_section(rows)
    dataset_json = dataset_json_payload(rows)

    html_doc = f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Miss Count Report</title>
  <style>{STYLE}</style>
</head>
<body>
  <header>
    <h1>Miss Count Report</h1>
    <p>Generated {timestamp} from {html.escape(str(base_dir))}</p>
  </header>
  {summary_html}
  <section class="scatter-section">
    <h2>Miss Count Comparison</h2>
    <figure>
      <img src="miss_count_comparison_scatter.png" alt="Scatter plot comparing miss count results across engines" loading="lazy" />
      <figcaption>miss_count_comparison_scatter.png generated from the latest run.</figcaption>
    </figure>
  </section>
  <div class="table-wrapper">
    <table>
      <thead>
        <tr>
          <th scope="col">Folder</th>
          <th scope="col">MISS_CNT</th>
          <th scope="col">TRI MISS_CNT</th>
          <th scope="col">REC_TRI MISS_CNT</th>
          <th scope="col">bit vs tmp</th>
          <th scope="col">tri vs tmp</th>
          <th scope="col">rec_tri vs tmp</th>
        </tr>
      </thead>
      <tbody>
        {table_rows}
      </tbody>
    </table>
  </div>
  <script type="application/json" id="misscnt-data">{dataset_json}</script>
</body>
</html>
"""
    output_path.write_text(html_doc, encoding="utf-8")


def main(base_dir: str = "css-gen-op"):
    base = Path(base_dir).resolve()
    rows = collect_rows(base)
    write_markdown(rows)
    write_html(rows, base)


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "css-gen-op")
