# /// script
# dependencies = ["matplotlib>=3.9,<4"]
# ///
"""Render measured results. Run with: uv run dev/plot-benchmarks.py."""
from pathlib import Path
import hashlib
import json
import statistics
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.colors import TwoSlopeNorm
import numpy as np

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"
PLOTS = DOCS / "benchmark-plots"
PLOTS.mkdir(exist_ok=True)
plt.rcParams.update({"svg.fonttype": "none", "font.size": 10, "axes.spines.top": False,
                     "axes.spines.right": False, "savefig.dpi": 160})
KINDS = ["bit", "tri", "rec_tri", "quad"]
SKIP_HISTORICAL = "--skip-historical" in sys.argv[1:]


def save(fig, name):
    if SKIP_HISTORICAL and name == "browser-total":
        plt.close(fig)
        return
    for suffix in ["svg", "png"]:
        path = PLOTS / f"{name}.{suffix}"
        fig.savefig(path, bbox_inches="tight")
        if suffix == "svg":
            path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
    plt.close(fig)


def measurements(site, workload):
    scenario = next(x for x in site["scenarios"] if x["workload"] == workload)
    return {x["engine"]: x for x in scenario["measurements"]}


def check_sources(report):
    for line in report["source_hashes"].splitlines():
        expected, path = line.split(maxsplit=1)
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == expected, path


corpus = json.loads((DOCS / "benchmark-results.json").read_text())
assert len(corpus["results"]) == 13, "corpus benchmark is incomplete"
assert corpus["samples"] == 7 and corpus["warmups"] == 1
check_sources(corpus)
for site in corpus["results"]:
    assert {s["workload"] for s in site["scenarios"]} == {"initial", "trace"}
    for scenario in site["scenarios"]:
        assert {m["engine"] for m in scenario["measurements"]} == set(KINDS + ["parcel_selectors"])
        for item in scenario["measurements"]:
            samples = item["samples_ns"]
            assert len(samples) == corpus["samples"]
            assert (item["min_ns"], item["median_ns"], item["max_ns"]) == (min(samples), statistics.median(samples), max(samples))
sites = [x for x in corpus["results"] if x["selectors"]]
matcher_ratios = {
    workload: np.array([[measurements(site, workload)["parcel_selectors"]["median_ns"] /
                         measurements(site, workload)[engine]["median_ns"] for engine in KINDS] for site in sites])
    for workload in ["initial", "trace"]
}
ratio_min = min(-1, int(np.floor(min(np.log2(r).min() for r in matcher_ratios.values()))))
ratio_max = max(1, int(np.ceil(max(np.log2(r).max() for r in matcher_ratios.values()))))
fig, axes = plt.subplots(1, 2, figsize=(11, 7), layout="constrained")
for ax, workload, title in zip(axes, ["initial", "trace"], ["Initial construction + matching", "Full mutation trace (includes initial)"]):
    ratios = matcher_ratios[workload]
    image = ax.imshow(np.log2(ratios), cmap="RdBu", norm=TwoSlopeNorm(vmin=ratio_min, vcenter=0, vmax=ratio_max), aspect="auto")
    ax.set(xticks=range(4), xticklabels=KINDS, yticks=range(len(sites)),
           yticklabels=[x["site"] for x in sites], title=title)
    for row in range(len(sites)):
        for col in range(4):
            ax.text(col, row, f"{ratios[row, col]:.2f}x", ha="center", va="center",
                    color="white" if np.log2(ratios[row, col]) < ratio_min * .6 or np.log2(ratios[row, col]) > ratio_max * .6 else "black")
ticks = range(ratio_min, ratio_max + 1)
bar = fig.colorbar(image, ax=axes, shrink=.75)
bar.set_ticks(ticks, labels=[f"{2.0 ** power:g}x" for power in ticks])
bar.set_label("parcel_selectors median / project median; >1 favors project")
fig.suptitle("Released matcher comparison: 7 samples per cell\nBaseline rematches every frame; zero-selector fixture excluded", fontsize=12)
save(fig, "matcher-speedup")

lines = [
    "# 与 parcel_selectors 的性能对比", "",
    "本次对照使用已发布的 [parcel_selectors 0.28.2](https://docs.rs/parcel_selectors/0.28.2/parcel_selectors/)。",
    "两侧使用同一 DOM、同一修改序列和本项目支持的同一选择器子集。",
    "对照库在每个修改帧后完整匹配。本项目使用增量更新。此结果不代表与浏览器完整样式引擎的比较。", "",
    "本轮四个算法共用 Vec 节点表，其余运行时映射使用 FxHash。parcel_selectors 适配器也使用这份 DOM，因此两侧都获得存储优化。",
    "结果均来自本轮重测。旧测量的 CPU 状态不同，不用于计算本次改动的加速倍数。", "",
    "## 测试方法", "",
    "- release 构建，单线程，固定 CPU 4。",
    "- 每个实现预热一次，测量七次。每轮轮换运行顺序。",
    "- 使用 std::time::Instant。下表显示中位数。原始 JSON 保留全部样本、最小值和最大值。",
    "- 计时包含 DOM 构造或修改、派生伪类刷新、匹配和运行时状态维护。",
    "- CSS 解析、trace 读取、program 复制、结果集合导出和结果校验不计时。",
    "- initial 只执行首个初始化帧。trace 执行完整修改序列，包含初始化。",
    "- 所有数据集都先逐帧检查匹配集合。对照同时检查 naive、bit、tri、rec_tri、quad。",
    "- 对照适配器为每个 DOM 版本重建兄弟索引，并启用该版本内的 nth 缓存。索引构建计入耗时。",
    "- 没有启用祖先 Bloom filter、样式规则索引、浏览器失效管理、级联或布局。",
    "- 两侧共享 DOM 修改和派生伪类处理。因此，该检查独立比较匹配器，不独立验证 DOM 修改语义。", "",
    "## 环境", "",
    f"- 时间：`{corpus['captured_at_utc']}`。",
    f"- CPU：{corpus['cpu'].split(':', 1)[-1].strip()}。CPU 4 调速模式：`{corpus.get('cpu4_governor', '未记录')}`。",
    f"- 系统：`{corpus['kernel']}`。",
    f"- Rust：`{corpus['rustc'].splitlines()[0]}`。",
    f"- 基础提交：`{corpus['revision']}`。包含本次未提交改动，源码指纹保存在原始 JSON。", "",
    "## 汇总", "",
    "共有 13 个数据集。a_to_b 有零个参与匹配的选择器，保留原始数据，但不计入以下 12 个数据集的汇总。",
    "表中数值是各数据集“对照耗时 ÷ 本项目耗时”的几何平均。大于 1 表示本项目较快。", "",
    "| 工作负载 | bit | tri | rec_tri | quad |", "|---|---:|---:|---:|---:|",
]
for workload, title in [("initial", "首次构造与匹配"), ("trace", "完整修改序列")]:
    ratios = [statistics.geometric_mean(measurements(s, workload)["parcel_selectors"]["median_ns"] /
              measurements(s, workload)[e]["median_ns"] for s in sites) for e in KINDS]
    lines.append(f"| {title} | " + " | ".join(f"{v:.2f}×" for v in ratios) + " |")
lines += ["", "![初始化和完整修改序列的耗时倍率](benchmark-plots/matcher-speedup.svg)", "",
          "增量引擎测试见[原生 Stylo／Blitz 对比](stylo-benchmark.md)和[真实网站数据对比](website-benchmark.md)。",
          "存储优化的隔离实验见[性能分析](performance-analysis.md)。另保留[早期 Blink 测量](browser-benchmark.md)，供检查跨接口测量的限制。"]
for workload, title in [("initial", "首次构造与匹配"), ("trace", "完整修改序列")]:
    lines += ["", f"## {title}", "", "中位耗时，单位为毫秒。数值越小越快。", "",
              "| 数据集 | 选择器 | 帧数 | parcel_selectors | bit | tri | rec_tri | quad |",
              "|---|---:|---:|---:|---:|---:|---:|---:|"]
    for site in corpus["results"]:
        row = measurements(site, workload)
        frames = next(s["frames"] for s in site["scenarios"] if s["workload"] == workload)
        values = " | ".join(f"{row[e]['median_ns'] / 1e6:.3f}" for e in ["parcel_selectors"] + KINDS)
        lines.append(f"| {site['site']} | {site['selectors']} | {frames} | {values} |")
lines += ["", "## 解释范围", "",
          "本次结果区分首次计算成本和增量更新收益。不能把完整修改序列的倍率解释为单次选择器匹配的倍率。",
          "输入仅覆盖项目支持的选择器。简单单段选择器和不支持的语法按现有编译规则排除。排除数量保存在 JSON。",
          "短至数微秒的测试易受计时和分配开销影响。七次样本不能用于推断其他机器、浏览器或页面的性能。", "",
          "## 复现", "", "```sh", "cargo build --release --locked -p xtask",
          "taskset -c 4 target/release/xtask compare --all --samples 7 --output docs/benchmark-results.json",
          "uv run dev/plot-benchmarks.py --skip-historical", "```", "",
          "绘图还需要当前源码对应的受控 Stylo 基准 JSON。若 CPU 4 不可用，请选择可用 CPU。",
          "[原始结果](benchmark-results.json)包含全部耗时、环境、版本、源码指纹和测试覆盖数量。", ""]
(DOCS / "benchmark.md").write_text("\n".join(lines))
print("Wrote matcher report and figures.")

native = json.loads((DOCS / "browser-native-results.json").read_text())
browser = json.loads((DOCS / "browser-results.json").read_text())
assert len(native["results"]) == len(browser["results"]) == 8, "browser benchmark is incomplete"
browser_sites = {x["name"]: x for x in browser["results"]}
labels = [x["name"] for x in native["results"]]
advantages = []
for site in native["results"]:
    best = min(x["median_ns"] for x in measurements(site, "trace").values())
    blink = next(x for x in browser_sites[site["name"]]["scenarios"] if x["workload"] == "trace")
    advantages.append(best / blink["median_ns"])
colours = ["#4477AA", "#66CCEE", "#228833", "#CCBB44", "#AA3377"]
fig, ax = plt.subplots(figsize=(11, 10), layout="constrained")
for index, engine in enumerate(KINDS + ["Blink"]):
    medians, lower, upper = [], [], []
    for site in native["results"]:
        if engine == "Blink":
            item = next(x for x in browser_sites[site["name"]]["scenarios"] if x["workload"] == "trace")
            samples = [x["elapsed_ns"] for x in item["samples"]]
        else:
            item = measurements(site, "trace")[engine]
            samples = item["samples_ns"]
        median = item["median_ns"]
        medians.append(median / 1e6)
        lower.append((median - min(samples)) / 1e6)
        upper.append((max(samples) - median) / 1e6)
    ax.barh(np.arange(8) + (index - 2) * .15, medians, height=.135,
            xerr=[lower, upper], label=engine, color=colours[index], capsize=1.5, error_kw={"elinewidth": .7})
ax.set(yticks=range(8), yticklabels=labels, xscale="log", xlabel="Total milliseconds (log scale); lower is faster",
       title="Historical cross-boundary measurement: unequal output contracts\nInitial DOM + 100 mutations; 32 rules; median and min–max of 7 samples")
ax.invert_yaxis()
ax.grid(axis="x", alpha=.2)
ax.set_axisbelow(True)
ax.legend(ncols=5, loc="upper right")
save(fig, "browser-total")

lines = ["# 浏览器增量 CSS 对比", "",
"此页保留早期 Blink 测量。优先参阅[原生 Stylo／Blitz 对比](stylo-benchmark.md)。新基准统一计时器、输出要求和修改阶段。", "",
f"原测量中，Rust 与 Blink 的总耗时比为 {min(advantages):.1f}–{max(advantages):.1f}。此比值不能作为算法速度结论。",
"两侧的输出要求不同，计时包含初始化，且分别在原生程序与 JavaScript 中运行。",
"808 帧的结果校验通过，只能说明结果一致，不能消除这些测量差异。", "",
"| 实现 | 修改后的处理 | 复用内容 | 本次覆盖 |", "|---|---|---|---|",
"| 本项目 | 标记 dirty，沿父节点和前驱兄弟依赖传播 | 匹配状态、输入条件、quad 表达式 | 算法证明和性能实测 |",
"| Blink | 按选择器特征形成失效集合，定位需要重算的节点 | 已有计算样式及引擎内部缓存 | 真实浏览器性能实测 |",
"| Stylo | 按状态、属性和选择器依赖进行失效，再执行样式遍历 | 已有元素样式及引擎内部缓存 | 另见原生 Stylo／Blitz 实测 |", "",
"机制依据：[Blink 失效流程](https://chromium.googlesource.com/chromium/src/+/master/third_party/blink/renderer/core/css/style-invalidation.md)、"
"[Stylo 失效模块](https://doc.servo.org/style/invalidation/element/invalidator/index.html)、"
"[Firefox 对 Stylo 的说明](https://firefox-source-docs.mozilla.org/layout/StyleSystemOverview.html)。", "",
"## 实验条件", "",
f"- 浏览器：`{browser['version']['product']}`，通过 `{browser['executable']}` 启动。完整版本见原始 JSON。",
f"- CPU：{browser['cpu']}。两侧顺序执行，固定 CPU 4。",
"- 8 个确定性用例。每例包含 32 个选择器、初始化和 100 次修改。",
"- 每例有 256 或 1024 个子节点，外加一个根节点。位置修改最多增加一个子节点。",
"- local：修改单个子节点的 class。ancestor：切换根节点 class，影响后代。",
"- sibling：修改 class，影响相邻兄弟。position：交替在首位插入和删除，改变 nth-child 结果。",
"- 同一份输入分别交给 Rust 和浏览器。逐帧核对所有选择器的完整匹配集合。",
"- 浏览器还逐帧核对 background-color，确保真实样式计算与预期相同。校验不计时。",
"- 全节点样式校验的重算次数必须恰好等于帧数，确认校验没有补做计时路径遗漏的刷新。",
"- 每例预热一次，记录七次。Rust 内部轮换四个实现的执行顺序。浏览器在独立阶段运行。",
"- CSS 解析、输入读取、协议传输、结果导出和校验均不计入总耗时。",
"- 总耗时包含 DOM 构造、修改和同步刷新。浏览器使用真实 DOM、级联和样式重算；Rust 维护匹配状态。",
"- 每次修改后读取非布局属性，强制提交待处理样式。没有使用动画帧来合并多次修改。",
"- 刷新读取修改节点及其子节点或相邻兄弟。检查内部重算次数至少等于输入帧数，拒绝跳过刷新或合并计算的测量。",
"- 使用不继承的 background-color。规则分别命中不同 class，不使用网页原始声明。",
"- 该实验不测 hover/focus 事件、继承或 Shadow DOM。结果不能代表完整网页速度。",
"- 不主动请求布局或绘制。CDP 采集区间内的布局耗时另存 JSON，不用于修正总耗时。", "",
"Rust 的总耗时还包含每帧的派生伪类刷新，以及按需执行的兄弟位置查询。",
"当前实现分别遍历 DOM 和兄弟前缀。因此，这些数字不能单独衡量 tri/quad 缓存规则的成本。",
"实现见 [engine.rs](../src/clean/engine.rs) 中的 `refresh_pseudos` 和 `position`。", "",
"浏览器总耗时使用 `performance.now()`。内部重算耗时使用 CDP 的 `RecalcStyleDuration` 差值。",
"该指标来自样式重算探针，不能代表全部 DOM 修改和失效维护成本。",
"CDP 采集区间比 JavaScript 计时区间略宽，两个指标独立展示。",
"依据：[CDP Performance](https://chromedevtools.github.io/devtools-protocol/tot/Performance/)、"
"[Chromium 计时实现](https://github.com/chromium/chromium/blob/main/third_party/blink/renderer/core/inspector/inspector_performance_agent.cc)。", "",
"## 完整修改序列", "", "![浏览器和本项目的完整序列耗时](benchmark-plots/browser-total.svg)", "",
"中位耗时，单位为毫秒。数值越小越快。内部样式耗时是浏览器诊断项，不是与 Rust 等价的计时范围。", "",
"| 用例 | bit | tri | rec_tri | quad | Blink 总耗时 | Blink 内部样式 | 重算次数 |",
"|---|---:|---:|---:|---:|---:|---:|---:|"]
for site in native["results"]:
    row = measurements(site, "trace")
    blink = next(x for x in browser_sites[site["name"]]["scenarios"] if x["workload"] == "trace")
    values = [row[e]["median_ns"] / 1e6 for e in KINDS] + [blink["median_ns"] / 1e6, blink["median_style_ns"] / 1e6]
    lines.append(f"| {site['name']} | " + " | ".join(f"{x:.3f}" for x in values) + f" | {blink['median_style_count']} |")
lines += ["", "## 首次构造与匹配", "", "中位耗时，单位为毫秒。", "",
          "| 用例 | bit | tri | rec_tri | quad | Blink 总耗时 |", "|---|---:|---:|---:|---:|---:|"]
for site in native["results"]:
    row = measurements(site, "initial")
    blink = next(x for x in browser_sites[site["name"]]["scenarios"] if x["workload"] == "initial")
    values = [row[e]["median_ns"] / 1e6 for e in KINDS] + [blink["median_ns"] / 1e6]
    lines.append(f"| {site['name']} | " + " | ".join(f"{x:.3f}" for x in values) + " |")
lines += ["", "## 复现", "", "```sh", "cargo build --release -p xtask",
          "node dev/browser-benchmark.mjs --generate /tmp/css-browser-fixtures.json",
          "taskset -c 4 target/release/xtask browser-native --input /tmp/css-browser-fixtures.json --output docs/browser-native-results.json --oracle /tmp/css-browser-oracle.json --samples 7",
          "taskset -c 4 node dev/browser-benchmark.mjs --oracle /tmp/css-browser-oracle.json --output docs/browser-results.json --samples 7 --browser /usr/bin/brave",
          "uv run dev/plot-benchmarks.py", "```", "",
          "Node.js 需提供内置 WebSocket。请按本机环境修改浏览器路径和 CPU 编号。绘图还需要完整的 matcher 基准 JSON。", "",
          "原始数据：[Rust](browser-native-results.json)、[浏览器](browser-results.json)。",
          "图形提供 SVG 和 PNG，位于 [benchmark-plots](benchmark-plots/)。", ""]
if not SKIP_HISTORICAL:
    (DOCS / "browser-benchmark.md").write_text("\n".join(lines))
    print("Wrote historical browser report and figures.")

stylo = json.loads((DOCS / "stylo-results.json").read_text())
assert stylo["protocol"] == "native-incremental-rgba-v1"
assert len(stylo["results"]) == 8, "native Stylo benchmark is incomplete"
assert stylo["samples"] == 7 and stylo["warmups"] == 1
check_sources(stylo)
native_kinds = KINDS + ["stylo_blitz"]
engine_labels = KINDS + ["Stylo / Blitz"]
for case in stylo["results"]:
    assert case["updates"] == 100
    assert case["checked_frames_per_run"] == 101
    assert len(case["measurements"]) == 5
    for item in case["measurements"]:
        assert len(item["samples"]) == stylo["samples"]
        for sample in item["samples"]:
            assert sample["total_ns"] == sample["update_ns"] + sample["output_ns"]
        for phase in ["update", "output", "total"]:
            samples = sorted(s[phase + "_ns"] for s in item["samples"])
            assert item[phase]["median_ns"] == samples[len(samples) // 2]

fig, axes = plt.subplots(1, 2, figsize=(14, 10), layout="constrained", sharey=True)
for ax, phase, title in zip(axes, ["update", "output"], ["Mutation + incremental update", "Read all node IDs + RGBA values"]):
    for index, engine in enumerate(native_kinds):
        values, lo, hi = [], [], []
        for case in stylo["results"]:
            item = next(x for x in case["measurements"] if x["engine"] == engine)[phase]
            median = item["median_ns"]
            values.append(median / 1e6)
            lo.append((median - item["min_ns"]) / 1e6)
            hi.append((item["max_ns"] - median) / 1e6)
        ax.barh(np.arange(8) + (index - 2) * .15, values, height=.135,
                xerr=[lo, hi], label=engine_labels[index], color=colours[index],
                capsize=1.5, error_kw={"elinewidth": .7})
    ax.set(yticks=range(8), yticklabels=[x["name"] for x in stylo["results"]], xscale="log",
           xlabel="Milliseconds for 100 updates (log scale)", title=title)
    ax.grid(axis="x", alpha=.2)
    ax.set_axisbelow(True)
axes[0].invert_yaxis()
axes[0].legend(ncols=1, loc="upper right", fontsize=9)
fig.suptitle(f"Native incremental engines: common per-node output\nSingle thread; initialization excluded; median and min–max of {stylo['samples']} samples", fontsize=13)
save(fig, "stylo-phases")

lines = [
    "# 原生增量引擎对比", "",
    "本次直接比较本项目与 Stylo／Blitz。两侧在同一个 Rust 程序内运行。",
    "计时只覆盖初始化后的 100 次修改。两侧每次修改后都输出全节点颜色。", "",
    "这比原来的 Blink 跨接口测量更可比。但它仍是实现层基准，不是纯算法成本测量。",
    "Stylo 使用 Blitz 的 DOM 和失效接口。结果不能代表 Firefox，也不能替代 Blink 的原生引擎测试。", "",
    "## 对齐的条件", "",
    "| 条件 | 本项目 | Stylo／Blitz |",
    "|---|---|---|",
    "| 进程、构建、计时器 | 同一 release 程序，std::time::Instant | 相同 |",
    "| CPU、线程 | CPU 4，单线程 | CPU 4，显式 Sequential 遍历 |",
    "| 输入 | 相同 DOM、32 条规则、100 次修改 | 相同 |",
    "| 刷新频率 | 每次修改后完成更新 | 相同 |",
    "| 输出 | 按节点 ID 排序的完整 RGBA8 数组 | 相同 |",
    "| 初始化、CSS 解析 | 不计时 | 不计时 |",
    "| 布局、绘制、JS、IPC | 不执行 | 不执行 |",
    "| 校验 | 每帧比较所有节点 ID 和 RGBA 通道 | 相同 |",
    "| 执行顺序 | 五个实现同轮轮换 | 参与相同轮换 |", "",
    f"每例先预热一次，再记录 {stylo['samples']} 次。每轮新建 DOM，并完成首次更新后再计时。",
    "计时包含 DOM 修改、失效维护和同步更新。全节点结果读取单独计时。",
    "总耗时是两个计时区间之和。逐帧比较、结果数组释放和后端析构不计时。",
    "使用逐帧计时器，不做空循环扣除。误差线表示最小值至最大值，不是置信区间。", "",
    "## 输出为何可比", "",
    "每条规则只设置一个不继承的 background-color。每条规则的颜色不同。",
    "基准检查每帧、每个节点最多命中一条规则。因此，颜色可以表示该输入下的完整匹配结果。",
    "本项目把匹配状态转换为颜色。Stylo 读取真实计算样式。两侧都返回相同的有序数组。",
    "基准以 naive 全量计算作为逐帧参考。预热和全部计时样本都必须通过校验。", "",
    f"八组输入共 808 个不同帧位置。五个实现各运行 {stylo['samples'] + stylo['warmups']} 轮，共校验 {808 * 5 * (stylo['samples'] + stylo['warmups']):,} 份全节点结果。", "",
    "## 仍然存在的差异", "",
    "- Stylo 执行完整样式计算。本项目只匹配选择器，并按上述受限规则输出颜色。",
    "- 两侧保留各自的 DOM 存储、规则索引和缓存。此基准没有统一这些内部实现。",
    "- Blitz 的 set_attribute 会给修改节点及父节点添加子树重算标记。局部修改因此可能重算大量节点。",
    "- 此处直接使用发布版 Blitz 接口，没有删除这些失效标记，也没有自行重写 Stylo 调度。",
    "- 本项目仍执行现有的派生伪类刷新和兄弟位置查询。这些成本计入更新阶段。",
    "- 输入都是浅层树。规则按 32 个 class 分组。未覆盖规则重叠、继承、深树或真实网页声明。", "",
    "因此，局部用例中的排名不能推断 Stylo 核心、Firefox 或 Blink 的排名。",
    "如果要隔离纯失效算法，需要统一 DOM 访问与匹配器。那将是另一项实验。", "",
    "源码依据：[Stylo 项目](https://github.com/servo/stylo)、",
    "[Blitz 0.3.0-beta.3 属性修改](https://docs.rs/crate/blitz-dom/0.3.0-beta.3/source/src/mutator.rs)、",
    "[样式遍历接口](https://docs.rs/crate/blitz-dom/0.3.0-beta.3/source/src/stylo.rs)。", "",
    "## 结果", "",
    "![原生引擎更新和结果读取耗时](benchmark-plots/stylo-phases.svg)", "",
    "下表为 100 次修改及每次完整结果读取的中位总耗时，单位为毫秒。越小越快。",
    "每列使用自己的中位数。不选择每组最快的实现来计算统一倍率。", "",
    "| 用例 | bit | tri | rec_tri | quad | Stylo／Blitz |",
    "|---|---:|---:|---:|---:|---:|",
]
for case in stylo["results"]:
    row = {x["engine"]: x for x in case["measurements"]}
    lines.append(f"| {case['name']} | " + " | ".join(f"{row[e]['total']['median_ns'] / 1e6:.3f}" for e in native_kinds) + " |")
lines += ["", "仅修改与更新的中位耗时如下。结果读取不在此表内。单位仍为毫秒。", "",
          "| 用例 | bit | tri | rec_tri | quad | Stylo／Blitz |",
          "|---|---:|---:|---:|---:|---:|"]
for case in stylo["results"]:
    row = {x["engine"]: x for x in case["measurements"]}
    lines.append(f"| {case['name']} | " + " | ".join(f"{row[e]['update']['median_ns'] / 1e6:.3f}" for e in native_kinds) + " |")
lines += ["", "local 修改单个子节点的 class。ancestor 切换根节点 class。",
          "sibling 修改 class 并影响相邻兄弟。position 在首位交替插入和删除节点。",
          "名称中的数字是子节点数量。每例还包含一个根节点。", "",
          "## 版本与复现", "",
          f"- Stylo：{stylo['stylo']}。Blitz DOM：{stylo['blitz_dom']}。",
          f"- CPU：{stylo['cpu'].split(':', 1)[-1].strip()}。",
          f"- 编译器：{stylo['rustc'].splitlines()[0]}。",
          "- 使用独立 Cargo.lock 固定基准依赖。主引擎使用 rustc-hash 2.1.3。",
          "- 四个算法共用 Vec 节点表，其余运行时映射使用 FxHash。求值和失效流程保持原实现。",
          f"- CPU 4 调速模式：{stylo.get('cpu4_governor', '未记录')}。本轮同时重测四个算法与 Stylo／Blitz。", "",
          "```sh", "node dev/browser-benchmark.mjs --generate /tmp/css-browser-fixtures.json",
          "cargo build --release --locked --manifest-path dev/stylo-benchmark/Cargo.toml",
          "taskset -c 4 dev/stylo-benchmark/target/release/css-stylo-benchmark --input /tmp/css-browser-fixtures.json --output docs/stylo-results.json --samples 7",
          "uv run dev/plot-benchmarks.py", "```", "",
          "Node.js 仅用于生成输入。计时程序不启动 Node.js 或浏览器。请按本机环境修改 CPU 编号。",
          "绘图脚本同时读取已有 matcher 和 Blink 结果。", "",
          "[基准代码](../dev/stylo-benchmark/src/main.rs) · [原始数据](stylo-results.json) · [真实网站数据对比](website-benchmark.md) · [早期 Blink 测量](browser-benchmark.md)", ""]
(DOCS / "stylo-benchmark.md").write_text("\n".join(lines))
print("Wrote native Stylo report and phase plot (SVG and PNG).")
