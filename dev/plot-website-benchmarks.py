# /// script
# dependencies = ["matplotlib>=3.9,<4"]
# ///
"""Check the website measurements. Write the report and figures."""
from pathlib import Path
import hashlib
import json
import math
import statistics

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.colors import TwoSlopeNorm
import numpy as np

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"
PLOTS = DOCS / "benchmark-plots"
ENGINES = ["bit", "tri", "rec_tri", "quad", "stylo_blitz"]
LABELS = ["bit", "tri", "rec_tri", "quad", "Stylo / Blitz"]
SITES = ["amazon", "bilibili", "bing", "bootstrap", "google", "netflix", "tiktok", "whatsapp", "wikipedia", "yahoo", "youtube"]
report = json.loads((DOCS / "website-results.json").read_text())
assert report["protocol"] == "website-incremental-rgba-v1"
assert report["samples"] == 7 and report["warmups"] == 1
assert not report["validation_only"]
sites = report["results"]
assert [s["name"] for s in sites] == SITES
for hashes in [report["source_hashes"], *(s["input_hashes"] for s in sites)]:
    for line in hashes.splitlines():
        digest, path = line.split(maxsplit=1)
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, path
for site in sites:
    assert [m["engine"] for m in site["measurements"]] == ENGINES
    assert len(site["execution_order"]) == 8
    assert sum(site["command_counts"].values()) == site["checked_frames_per_run"]
    for measurement in site["measurements"]:
        samples = measurement["samples"]
        assert len(samples) == 7
        assert all(s["update_ns"] + s["output_ns"] == s["total_ns"] for s in samples)
        for phase in ["update", "output", "total"]:
            values = [s[f"{phase}_ns"] for s in samples]
            assert measurement[phase] == {"median_ns": statistics.median(values), "min_ns": min(values), "max_ns": max(values)}


def times(phase, field="median_ns"):
    return np.array([[m[phase][field] / 1e6 for m in s["measurements"]] for s in sites])


def save(fig, name):
    for suffix in ["svg", "png"]:
        fig.savefig(PLOTS / f"{name}.{suffix}", bbox_inches="tight", dpi=160)
    plt.close(fig)


plt.rcParams.update({"svg.fonttype": "none", "font.size": 10, "axes.spines.top": False, "axes.spines.right": False})
PLOTS.mkdir(exist_ok=True)
colours = ["#2468b4", "#47a5cc", "#6b54a3", "#d58b22", "#258354"]
total = times("total")
fig, ax = plt.subplots(figsize=(11, 9), layout="constrained")
for col, label in enumerate(LABELS):
    y = np.arange(len(sites)) + (col - 2) * .15
    values = total[:, col]
    ax.barh(y, values, height=.13, label=label, color=colours[col], xerr=np.array([values - times("total", "min_ns")[:, col], times("total", "max_ns")[:, col] - values]), error_kw={"elinewidth":.7,"capsize":1.5})
ax.set(yticks=np.arange(len(sites)), yticklabels=SITES, xscale="log", xlabel="Full trace after initialization: update + all-node RGBA output (ms, log scale)", title="Captured website data: five native implementations\nMedian of seven samples; error bars show min–max; lower is faster")
ax.invert_yaxis()
ax.grid(axis="x", alpha=.2)
ax.set_axisbelow(True)
ax.legend(ncols=5, loc="lower center", bbox_to_anchor=(.5, 1.07))
save(fig, "website-times")

fig, axes = plt.subplots(1, 2, figsize=(11, 7), layout="constrained")
all_ratios = [times(phase)[:, 4, None] / times(phase)[:, :4] for phase in ["update", "total"]]
limit = max(1, math.ceil(max(np.abs(np.log2(r)).max() for r in all_ratios)))
for ax, ratios, title in zip(axes, all_ratios, ["Update only", "Update + all-node RGBA output"]):
    im = ax.imshow(np.log2(ratios), cmap="RdBu", norm=TwoSlopeNorm(vmin=-limit, vcenter=0, vmax=limit), aspect="auto")
    ax.set(xticks=range(4), xticklabels=ENGINES[:4], yticks=range(len(sites)), yticklabels=SITES, title=title)
    for row in range(len(sites)):
        for col in range(4):
            ax.text(col, row, f"{ratios[row,col]:.2f}×", ha="center", va="center", color="white" if abs(math.log2(ratios[row,col])) > limit*.55 else "black")
fig.suptitle("Stylo / Blitz time ÷ project time\nAbove 1: project is faster. Below 1: Stylo / Blitz is faster.")
bar = fig.colorbar(im, ax=axes, shrink=.65, label="Time ratio (log scale)")
bar.set_ticks(range(-limit,limit+1), [f"{2.0**i:g}×" for i in range(-limit,limit+1)])
save(fig, "website-ratios")


def table(phase):
    lines = ["| 网站 | bit | tri | rec_tri | quad | Stylo／Blitz |", "|---|---:|---:|---:|---:|---:|"]
    for site, row in zip(sites,times(phase)):
        minimum = min(row)
        values = [f"**{v:.3f}**" if v == minimum else f"{v:.3f}" for v in row]
        lines.append("| " + site["name"] + " | " + " | ".join(values) + " |")
    return "\n".join(lines)


frames = sum(s["checked_frames_per_run"] for s in sites)
updates = sum(s["updates"] for s in sites)
node_frames = sum(s["node_frames"] for s in sites)
best = total[:, :4].min(axis=1)
stylo_wins = sum(total[:,4] < best)
project_wins = len(sites) - stylo_wins
geo_total = [statistics.geometric_mean(all_ratios[1][:,i]) for i in range(4)]
geo_update = [statistics.geometric_mean(all_ratios[0][:,i]) for i in range(4)]
size_lines = ["| 网站 | 选择器 | 跳过简单项 | 不支持项 | 初始／峰值节点 | 更新帧 | 有重叠的节点帧 |", "|---|---:|---:|---:|---:|---:|---:|"]
for s in sites:
    size_lines.append(f"| {s['name']} | {s['selectors']} | {s['skipped_simple']} | {s['unsupported']} | {s['initial_nodes']}／{s['peak_nodes']} | {s['updates']} | {s['overlapping_node_frames']:,} |")
text = f'''# 真实网站数据上的增量算法对比

本次使用仓库保存的 11 个网站捕获数据。比较 bit、tri、rec_tri、quad，以及 Stylo 0.22.0／Blitz DOM 0.3.0-beta.3。
输入保留网站的树结构、选择器条件和 DOM 修改序列。声明统一为可校验的背景颜色。
因此，结果衡量网站数据在共同算法模型中的回放成本。结果不代表原始网页渲染、Firefox 或 Blink 的性能。

按“更新加输出”的中位数，Stylo／Blitz 在 {stylo_wins} 个网站快于全部四个项目实现。项目实现在 {project_wins} 个网站更快。
不同网站的工作负载差异较大。请按网站查看结果。
粗体只表示实测中位数最小，不表示统计显著。接近的结果需结合样本范围判断。

![五个实现的耗时](benchmark-plots/website-times.svg)

## 总耗时

单位为毫秒。每格是七次测量的中位数。粗体为每行最小值。
计时从初始化后的第一帧开始，包含每帧更新和全节点 RGBA 输出。

{table("total")}

![更新阶段及更新加输出的倍率](benchmark-plots/website-ratios.svg)

下表对每个网站等权，计算“Stylo／Blitz 耗时 ÷ 项目耗时”的几何平均。大于 1 表示项目实现较快。

| 阶段 | bit | tri | rec_tri | quad |
|---|---:|---:|---:|---:|
| 更新 | {" | ".join(f"{v:.2f}×" for v in geo_update)} |
| 更新加输出 | {" | ".join(f"{v:.2f}×" for v in geo_total)} |

## 数据范围

共 {frames:,} 个有效帧，其中 {updates:,} 个更新帧。`layout_*` 记录不参与测试。
原始 JSON 保存 CSS、trace 和实现源码的 SHA-256。
数据来自仓库中的 `css-gen-op/<网站>/`。这不是本次联网抓取，也不是当前线上网站的快照。
`a_to_b` 和 `testcase` 是小型测试用例，不计入网站汇总。`reddit` 缺少 CSS 文件，因此无法构造共同输入。

{"\n".join(size_lines)}

“跳过简单项”和“不支持项”使用项目编译器的报告。编译器只读取顶层样式规则。
嵌套规则、媒体条件、原始声明、原始级联层及 `!important` 不参与测试。
因此，这些计数不能表示完整网页 CSS 的覆盖率。
“有重叠的节点帧”表示某一帧中一个节点至少命中两个参与测试的选择器。一个节点可以在多帧中重复计数。

## 共同语义与校验

- 全部记录节点都参与算法树，包括文本、document 和 Shadow DOM 记录。两侧将它们映射为普通 HTML 元素。
- 标签名采用可逆编码。类型选择器同步转换。通配选择器保持通配含义。
- 除 class、id 和 trace 状态标记外，属性名采用可逆编码。这会停用内联样式、hidden 等内建行为，并保留属性等值条件。
- 每个已接受的选择器保留独立条目和状态位置。相同条件的条目也不合并。
- 每条规则设置一个唯一的不透明 RGB 颜色。未命中的节点输出透明色。
- 优先级取转换后条件的 CSS specificity。相同优先级按原编译程序的选择器顺序决定胜者。该顺序不是原网页的声明顺序。
- hover 向子树传播。focus 位于标记节点。focus-within 向祖先传播。两侧每帧重新派生这些状态，成本计入更新。
- 此版 Blitz 的 `FocusWithin` 匹配固定为 false。适配器用空闲的 `:active`／ACTIVE 状态承载相同布尔条件。specificity 不变。此项不测试 Blitz 原生 focus-within 支持。
- 项目根节点没有兄弟位置。原生 nth 条件增加零 specificity 的 `:where(:not(:root))`，以保持这个约定。
- 节点移动保留逻辑旧位置。原生 DOM 只保存活节点。后续删除旧位置不会删除移动后的新实例。

计时前，bit 逐帧检查原始输入与转换后输入的全部选择器匹配位，证明本次数据转换未改变匹配结果。
这里是有限数据上的校验，不是形式化证明。
然后五个实现逐帧比较全部活节点的 `(trace ID, RGBA8)`。比较在计时区间外执行。
每轮共检查 {frames:,} 帧、{node_frames:,} 个节点帧。五个实现各运行八轮，共检查 {frames*40:,} 份帧输出。
颜色只标识最终获胜规则。规则重叠时，它不能证明每个未获胜规则的匹配位正确。
已有的 [parcel_selectors 对比](benchmark.md) 单独检查完整匹配集合。
新增的四个回归测试使用 naive 作为参考，检查规则重叠、移动、位置、动态状态和重复条件。

## 计时方法与限制

- 同一个 release 二进制，同一个 `std::time::Instant`，固定 CPU 4。Stylo 显式使用顺序遍历。
- 每个实现预热一次，再测量七次。每轮轮换五个实现的顺序。每次使用新引擎和新 DOM。
- 初始化、CSS 解析、输入转换、参考结果生成、输出比较和析构不计时。
- 更新计时包含 DOM 修改、状态维护、失效处理和样式／匹配计算。Stylo 调用 `resolve_stylist`，不调用布局或绘制。
- 输出计时包含生成按 trace ID 排序的全节点 RGBA 列表。项目实现从匹配位中选择获胜规则。Stylo 读取已计算的背景颜色。
- 每条非 layout 命令后都计算并输出，包括 `recalculate`。测试不模拟浏览器在多条 DOM 修改间合并样式计算。
- Stylo 通过 Blitz 的公开修改接口维护失效。其父节点／子树失效可能比 Firefox 更保守。
- 两侧保留各自数据结构。Stylo 另有 trace ID 和逻辑位置适配成本。结果是这些实现与适配器的比较，不能归因于单一缓存算法。
- Stylo 仍计算它需要的样式结构。项目引擎仍维护全部匹配位。统一输出不会消除两侧内部工作的差别。
- 样本只有一台机器、一份捕获语料和一种运行顺序轮换。图中的误差线为最小值至最大值，不是置信区间。

环境：`{report['cpu'].split(':',1)[-1].strip()}`，`{report['rustc'].splitlines()[0]}`。
运行开始时间：`{report['captured_at_utc']}`。CPU 亲和性：`{report['cpu_affinity']}`。

## 阶段耗时

更新耗时，单位为毫秒：

{table("update")}

输出耗时，单位为毫秒：

{table("output")}

各阶段分别取中位数。因此，阶段中位数之和不一定等于总耗时的中位数。

## 复现

```sh
cargo test --release --locked --manifest-path dev/stylo-benchmark/Cargo.toml --bin css-stylo-websites
cargo build --release --locked --manifest-path dev/stylo-benchmark/Cargo.toml --bin css-stylo-websites
taskset -c 4 dev/stylo-benchmark/target/release/css-stylo-websites --samples 7 --output docs/website-results.json
uv run dev/plot-website-benchmarks.py
```

使用 `--site bing --validate-only --output /tmp/bing-validation.json` 可只检查一个网站。
绘图脚本检查样本数量、中位数、源码指纹和输入指纹，再生成本报告和 SVG／PNG。

[原始样本](website-results.json) · [基准实现](../dev/stylo-benchmark/src/websites.rs) · [绘图脚本](../dev/plot-website-benchmarks.py) · [受控 Stylo 对比](stylo-benchmark.md)
'''
(DOCS / "website-benchmark.md").write_text(text)
print(f"Wrote website report and figures. Stylo wins: {stylo_wins}/{len(sites)}.")
