# 浏览器增量 CSS 对比

此页保留早期 Blink 测量。优先参阅[原生 Stylo／Blitz 对比](stylo-benchmark.md)。新基准统一计时器、输出要求和修改阶段。

原测量中，Rust 与 Blink 的总耗时比为 5.9–22.3。此比值不能作为算法速度结论。
两侧的输出要求不同，计时包含初始化，且分别在原生程序与 JavaScript 中运行。
808 帧的结果校验通过，只能说明结果一致，不能消除这些测量差异。

| 实现 | 修改后的处理 | 复用内容 | 本次覆盖 |
|---|---|---|---|
| 本项目 | 标记 dirty，沿父节点和前驱兄弟依赖传播 | 匹配状态、输入条件、quad 表达式 | 算法证明和性能实测 |
| Blink | 按选择器特征形成失效集合，定位需要重算的节点 | 已有计算样式及引擎内部缓存 | 真实浏览器性能实测 |
| Stylo | 按状态、属性和选择器依赖进行失效，再执行样式遍历 | 已有元素样式及引擎内部缓存 | 另见原生 Stylo／Blitz 实测 |

机制依据：[Blink 失效流程](https://chromium.googlesource.com/chromium/src/+/master/third_party/blink/renderer/core/css/style-invalidation.md)、[Stylo 失效模块](https://doc.servo.org/style/invalidation/element/invalidator/index.html)、[Firefox 对 Stylo 的说明](https://firefox-source-docs.mozilla.org/layout/StyleSystemOverview.html)。

## 实验条件

- 浏览器：`Chrome/155.0.8059.40`，通过 `/usr/bin/brave` 启动。完整版本见原始 JSON。
- CPU：AMD Ryzen AI 7 H 350 w/ Radeon 860M。两侧顺序执行，固定 CPU 4。
- 8 个确定性用例。每例包含 32 个选择器、初始化和 100 次修改。
- 每例有 256 或 1024 个子节点，外加一个根节点。位置修改最多增加一个子节点。
- local：修改单个子节点的 class。ancestor：切换根节点 class，影响后代。
- sibling：修改 class，影响相邻兄弟。position：交替在首位插入和删除，改变 nth-child 结果。
- 同一份输入分别交给 Rust 和浏览器。逐帧核对所有选择器的完整匹配集合。
- 浏览器还逐帧核对 background-color，确保真实样式计算与预期相同。校验不计时。
- 全节点样式校验的重算次数必须恰好等于帧数，确认校验没有补做计时路径遗漏的刷新。
- 每例预热一次，记录七次。Rust 内部轮换四个实现的执行顺序。浏览器在独立阶段运行。
- CSS 解析、输入读取、协议传输、结果导出和校验均不计入总耗时。
- 总耗时包含 DOM 构造、修改和同步刷新。浏览器使用真实 DOM、级联和样式重算；Rust 维护匹配状态。
- 每次修改后读取非布局属性，强制提交待处理样式。没有使用动画帧来合并多次修改。
- 刷新读取修改节点及其子节点或相邻兄弟。检查内部重算次数至少等于输入帧数，拒绝跳过刷新或合并计算的测量。
- 使用不继承的 background-color。规则分别命中不同 class，不使用网页原始声明。
- 该实验不测 hover/focus 事件、继承或 Shadow DOM。结果不能代表完整网页速度。
- 不主动请求布局或绘制。CDP 采集区间内的布局耗时另存 JSON，不用于修正总耗时。

Rust 的总耗时还包含每帧的派生伪类刷新，以及按需执行的兄弟位置查询。
当前实现分别遍历 DOM 和兄弟前缀。因此，这些数字不能单独衡量 tri/quad 缓存规则的成本。
实现见 [engine.rs](../src/clean/engine.rs) 中的 `refresh_pseudos` 和 `position`。

浏览器总耗时使用 `performance.now()`。内部重算耗时使用 CDP 的 `RecalcStyleDuration` 差值。
该指标来自样式重算探针，不能代表全部 DOM 修改和失效维护成本。
CDP 采集区间比 JavaScript 计时区间略宽，两个指标独立展示。
依据：[CDP Performance](https://chromedevtools.github.io/devtools-protocol/tot/Performance/)、[Chromium 计时实现](https://github.com/chromium/chromium/blob/main/third_party/blink/renderer/core/inspector/inspector_performance_agent.cc)。

## 完整修改序列

![浏览器和本项目的完整序列耗时](benchmark-plots/browser-total.svg)

中位耗时，单位为毫秒。数值越小越快。内部样式耗时是浏览器诊断项，不是与 Rust 等价的计时范围。

| 用例 | bit | tri | rec_tri | quad | Blink 总耗时 | Blink 内部样式 | 重算次数 |
|---|---:|---:|---:|---:|---:|---:|---:|
| local-256 | 8.864 | 8.615 | 8.253 | 8.591 | 1.400 | 0.689 | 101 |
| ancestor-256 | 335.321 | 336.070 | 337.405 | 338.445 | 55.200 | 51.925 | 101 |
| sibling-256 | 32.165 | 30.325 | 30.143 | 31.001 | 1.800 | 0.899 | 101 |
| position-256 | 339.213 | 343.293 | 340.908 | 341.994 | 56.500 | 53.732 | 101 |
| local-1024 | 40.374 | 39.907 | 39.174 | 40.042 | 3.600 | 1.807 | 101 |
| ancestor-1024 | 4131.245 | 4164.931 | 4132.831 | 4153.332 | 196.400 | 193.470 | 101 |
| sibling-1024 | 98.396 | 96.488 | 95.720 | 96.936 | 4.300 | 2.583 | 101 |
| position-1024 | 4139.522 | 4200.306 | 4157.450 | 4139.832 | 239.300 | 229.742 | 101 |

## 首次构造与匹配

中位耗时，单位为毫秒。

| 用例 | bit | tri | rec_tri | quad | Blink 总耗时 |
|---|---:|---:|---:|---:|---:|
| local-256 | 1.844 | 1.832 | 1.842 | 1.873 | 1.000 |
| ancestor-256 | 2.689 | 2.673 | 2.694 | 2.708 | 0.700 |
| sibling-256 | 4.164 | 4.248 | 4.199 | 4.181 | 0.900 |
| position-256 | 3.240 | 3.219 | 3.232 | 3.274 | 1.000 |
| local-1024 | 15.231 | 15.308 | 15.341 | 15.729 | 2.700 |
| ancestor-1024 | 29.012 | 29.750 | 29.482 | 29.192 | 2.700 |
| sibling-1024 | 28.820 | 29.513 | 28.867 | 28.783 | 2.800 |
| position-1024 | 34.627 | 34.724 | 34.798 | 34.743 | 3.000 |

## 复现

```sh
cargo build --release -p xtask
node dev/browser-benchmark.mjs --generate /tmp/css-browser-fixtures.json
taskset -c 4 target/release/xtask browser-native --input /tmp/css-browser-fixtures.json --output docs/browser-native-results.json --oracle /tmp/css-browser-oracle.json --samples 7
taskset -c 4 node dev/browser-benchmark.mjs --oracle /tmp/css-browser-oracle.json --output docs/browser-results.json --samples 7 --browser /usr/bin/brave
uv run dev/plot-benchmarks.py
```

Node.js 需提供内置 WebSocket。请按本机环境修改浏览器路径和 CPU 编号。绘图还需要完整的 matcher 基准 JSON。

原始数据：[Rust](browser-native-results.json)、[浏览器](browser-results.json)。
图形提供 SVG 和 PNG，位于 [benchmark-plots](benchmark-plots/)。
