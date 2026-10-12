# 原生增量引擎对比

本次直接比较本项目与 Stylo／Blitz。两侧在同一个 Rust 程序内运行。
计时只覆盖初始化后的 100 次修改。两侧每次修改后都输出全节点颜色。

这比原来的 Blink 跨接口测量更可比。但它仍是实现层基准，不是纯算法成本测量。
Stylo 使用 Blitz 的 DOM 和失效接口。结果不能代表 Firefox，也不能替代 Blink 的原生引擎测试。

## 对齐的条件

| 条件 | 本项目 | Stylo／Blitz |
|---|---|---|
| 进程、构建、计时器 | 同一 release 程序，std::time::Instant | 相同 |
| CPU、线程 | CPU 4，单线程 | CPU 4，显式 Sequential 遍历 |
| 输入 | 相同 DOM、32 条规则、100 次修改 | 相同 |
| 刷新频率 | 每次修改后完成更新 | 相同 |
| 输出 | 按节点 ID 排序的完整 RGBA8 数组 | 相同 |
| 初始化、CSS 解析 | 不计时 | 不计时 |
| 布局、绘制、JS、IPC | 不执行 | 不执行 |
| 校验 | 每帧比较所有节点 ID 和 RGBA 通道 | 相同 |
| 执行顺序 | 五个实现同轮轮换 | 参与相同轮换 |

每例先预热一次，再记录 7 次。每轮新建 DOM，并完成首次更新后再计时。
计时包含 DOM 修改、失效维护和同步更新。全节点结果读取单独计时。
总耗时是两个计时区间之和。逐帧比较、结果数组释放和后端析构不计时。
使用逐帧计时器，不做空循环扣除。误差线表示最小值至最大值，不是置信区间。

## 输出为何可比

每条规则只设置一个不继承的 background-color。每条规则的颜色不同。
基准检查每帧、每个节点最多命中一条规则。因此，颜色可以表示该输入下的完整匹配结果。
本项目把匹配状态转换为颜色。Stylo 读取真实计算样式。两侧都返回相同的有序数组。
基准以 naive 全量计算作为逐帧参考。预热和全部计时样本都必须通过校验。

八组输入共 808 个不同帧位置。五个实现各运行 8 轮，共校验 32,320 份全节点结果。

## 仍然存在的差异

- Stylo 执行完整样式计算。本项目只匹配选择器，并按上述受限规则输出颜色。
- 两侧保留各自的 DOM 存储、规则索引和缓存。此基准没有统一这些内部实现。
- Blitz 的 set_attribute 会给修改节点及父节点添加子树重算标记。局部修改因此可能重算大量节点。
- 此处直接使用发布版 Blitz 接口，没有删除这些失效标记，也没有自行重写 Stylo 调度。
- 本项目仍执行现有的派生伪类刷新和兄弟位置查询。这些成本计入更新阶段。
- 输入都是浅层树。规则按 32 个 class 分组。未覆盖规则重叠、继承、深树或真实网页声明。

因此，局部用例中的排名不能推断 Stylo 核心、Firefox 或 Blink 的排名。
如果要隔离纯失效算法，需要统一 DOM 访问与匹配器。那将是另一项实验。

源码依据：[Stylo 项目](https://github.com/servo/stylo)、
[Blitz 0.3.0-beta.3 属性修改](https://docs.rs/crate/blitz-dom/0.3.0-beta.3/source/src/mutator.rs)、
[样式遍历接口](https://docs.rs/crate/blitz-dom/0.3.0-beta.3/source/src/stylo.rs)。

## 结果

![原生引擎更新和结果读取耗时](benchmark-plots/stylo-phases.svg)

下表为 100 次修改及每次完整结果读取的中位总耗时，单位为毫秒。越小越快。
每列使用自己的中位数。不选择每组最快的实现来计算统一倍率。

| 用例 | bit | tri | rec_tri | quad | Stylo／Blitz |
|---|---:|---:|---:|---:|---:|
| local-256 | 0.721 | 0.679 | 0.660 | 0.692 | 35.654 |
| ancestor-256 | 29.741 | 29.896 | 30.083 | 30.449 | 37.097 |
| sibling-256 | 1.473 | 1.414 | 1.407 | 1.440 | 38.665 |
| position-256 | 29.609 | 29.655 | 29.598 | 29.884 | 40.810 |
| local-1024 | 2.208 | 2.165 | 2.131 | 2.186 | 146.037 |
| ancestor-1024 | 278.392 | 279.765 | 278.255 | 280.356 | 147.829 |
| sibling-1024 | 4.908 | 4.831 | 4.777 | 4.812 | 165.400 |
| position-1024 | 274.190 | 275.856 | 275.077 | 276.937 | 171.121 |

仅修改与更新的中位耗时如下。结果读取不在此表内。单位仍为毫秒。

| 用例 | bit | tri | rec_tri | quad | Stylo／Blitz |
|---|---:|---:|---:|---:|---:|
| local-256 | 0.501 | 0.461 | 0.441 | 0.473 | 35.313 |
| ancestor-256 | 29.557 | 29.710 | 29.895 | 30.255 | 36.751 |
| sibling-256 | 1.255 | 1.198 | 1.186 | 1.220 | 38.330 |
| position-256 | 29.424 | 29.466 | 29.409 | 29.696 | 40.448 |
| local-1024 | 1.441 | 1.394 | 1.354 | 1.409 | 144.682 |
| ancestor-1024 | 277.380 | 278.853 | 277.336 | 279.368 | 146.457 |
| sibling-1024 | 4.126 | 4.042 | 3.981 | 4.015 | 164.014 |
| position-1024 | 273.233 | 274.877 | 274.044 | 275.916 | 169.712 |

local 修改单个子节点的 class。ancestor 切换根节点 class。
sibling 修改 class 并影响相邻兄弟。position 在首位交替插入和删除节点。
名称中的数字是子节点数量。每例还包含一个根节点。

## 版本与复现

- Stylo：0.22.0。Blitz DOM：0.3.0-beta.3。
- CPU：AMD Ryzen AI 7 H 350 w/ Radeon 860M。
- 编译器：rustc 1.99.0 (b940084d7 2026-09-28)。
- 使用独立 Cargo.lock 固定基准依赖。主引擎使用 rustc-hash 2.1.3。
- 四个算法共用 Vec 节点表，其余运行时映射使用 FxHash。求值和失效流程保持原实现。
- CPU 4 调速模式：performance。本轮同时重测四个算法与 Stylo／Blitz。

```sh
node dev/browser-benchmark.mjs --generate /tmp/css-browser-fixtures.json
cargo build --release --locked --manifest-path dev/stylo-benchmark/Cargo.toml
taskset -c 4 dev/stylo-benchmark/target/release/css-stylo-benchmark --input /tmp/css-browser-fixtures.json --output docs/stylo-results.json --samples 7
uv run dev/plot-benchmarks.py
```

Node.js 仅用于生成输入。计时程序不启动 Node.js 或浏览器。请按本机环境修改 CPU 编号。
绘图脚本同时读取已有 matcher 和 Blink 结果。

[基准代码](../dev/stylo-benchmark/src/main.rs) · [原始数据](stylo-results.json) · [真实网站数据对比](website-benchmark.md) · [早期 Blink 测量](browser-benchmark.md)
