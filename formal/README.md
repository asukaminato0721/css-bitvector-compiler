# 算法的 Lean 4 证明

本目录证明 CSS 状态递推、tri 缓存复用、quad 专门化、修改失效和 dirty 遍历的数学性质。
Lean 内核检查证明项。证明不限制状态数、表达式长度或有限上下文的深度。
`CssProofs/` 包含算法模型证明。
[rust/](rust/README.md) 包含生产 Rust 经 Charon/Aeneas 转换后的局部源码证明。
完整 Rust 引擎的端到端验证尚未完成。

## 选择 Lean 4

三个证明器都能表达本项目所需的归纳定义和正确性定理。
本项目选择 Lean 4，主要考虑归纳证明、交互式证明和独立构建的需要。
算法模型只使用 Lean 自带的 `Std`。源码证明另依赖 Aeneas 后端及其 Mathlib 依赖。
本项目使用 Lean 4 作为主证明器。保留已有 Kani 检查，不引入 Verus。

| 证明器 | 本项目的适用方向 | 选择依据 |
|---|---|---|
| Lean 4 | 算法语义、缓存条件、递归结构 | 本次选择。可用归纳定义组织模型，并用 Lake 构建证明 |
| Rocq | 程序验证，以及后续提取已验证程序 | 若计划以提取程序替换实现，应重新评估 |
| Agda | 用依赖类型构造算法及其规格 | 若计划把算法本身写成依赖类型程序，应重新评估 |

这是针对本项目的工程判断，不是证明能力排名。
参考官方资料：[Lean 归纳与递归](https://docs.lean-lang.org/theorem_proving_in_lean4/induction_and_recursion.html)、
[Rocq 程序提取](https://docs.rocq-prover.org/master/refman/addendum/extraction.html)、
[Agda 安全模式](https://agda.readthedocs.io/en/latest/language/safe-agda.html)。

## 构建

先按 [Lean 官方安装说明](https://lean-lang.org/install/manual/) 安装 Lean 工具链管理器 `elan`。
然后从项目根目录执行：

```sh
cd formal
lake build
```

`lean-toolchain` 固定 Lean 版本为 `v4.31.0`。
`lake build` 检查全部证明，并运行公理检查。
CI 的 `lean` job 执行同一构建。

项目把 Lean 警告视为错误。
[Audit.lean](CssProofs/Audit.lean) 检查所有已导入的 `CssProofs` 声明及其公理依赖。
检查只允许 Lean 的 `propext`、`Quot.sound` 和 `Classical.choice`。
`sorryAx`、额外公理和依赖本机计算的证明公理均不在允许范围内。
新增证明模块时，必须在 `Audit.lean` 中导入该模块。

## 定理与实现的对应关系

| 文件 | 主要定理 | Rust 对应位置 |
|---|---|---|
| [Requirements.lean](CssProofs/Requirements.lean) | `compatible_record_iff`、`record_cons` | `Requirement`、`record_decision`、`requirements_accept` |
| [Tri.lean](CssProofs/Tri.lean) | `tri_skip_sound`、`tri_program_reuse` | `evaluate_bits`、`recompute_node` 的 tri 分支 |
| [Tri.lean](CssProofs/Tri.lean) | `recursive_tri_admission` | `propagate_output_change` 的 recursive-tri 分支 |
| [Quad.lean](CssProofs/Quad.lean) | `specialized_or_sound`、`concrete_accept_sound` | `specialize_or`、`specialize_concrete` |
| [Quad.lean](CssProofs/Quad.lean) | `quad_step_sound`、`quad_program_reuse`、`quad_initial_correct` | `evaluate_quad`、`materialize_quad` |
| [Selectors.lean](CssProofs/Selectors.lean) | `compiled_correct`、`accept_correct` | 状态递推、`matches_selector_part` |
| [Incremental.lean](CssProofs/Incremental.lean) | `sweep_correct`、`trace_then_sweep` | 修改失效与前序 dirty 遍历的模型 |

Rust 文件位于 [logic.rs](../src/clean/logic.rs) 和 [engine.rs](../src/clean/engine.rs)。
此表描述算法模型与 Rust 的对应关系，不代表所有函数已有源码精化证明。
源码证明的具体覆盖范围见 [rust/README.md](rust/README.md)。

## 证明内容

### 输入条件

`Address n = Channel × Fin n` 表示通道与状态索引。
`Channel` 区分 parent 与 previous sibling。
`Fin n` 保证索引有效，`n` 可以是任意自然数，包括零。

`record reads old` 对读取过的地址记录旧值，对其他地址记录 `unused`。
`compatible_record_iff` 证明以下等价关系：

```text
新输入满足记录的条件 ↔ 每个已读取地址的新值等于旧值
```

`record_cons` 证明重复读取对应覆盖同一数组位置。
所有读取都来自同一次求值的输入，因此重复写入不会产生相互冲突的条件。
模型保留读取地址列表，再生成条件函数。Rust 直接更新条件数组。

### Tri 与 recursive-tri

`Expr` 包含常量、输入读取、短路 AND 和记录两侧读取的 OR。
节点谓词先变成常量。它们在输入变化期间必须保持不变。
OR 的读取模型对应 Rust 先计算 `raw` 和 `carried`，再计算 `raw || carried`。

对表达式结构归纳，`tri_skip_sound` 证明：

```text
Compatible(record(reads(e, old), old), new)
  → eval(e, new) = eval(e, old)
```

AND 左侧为 false 时，证明不要求右侧输入保持不变。
`tri_program_reuse` 把此结果推广到任意有限表达式列表。

`recursive_tri_admission` 只检查发生变化的通道。
该定理明确要求另一通道保持不变。
它证明当前节点的输出不变，因此当前事件不需要使下游输入失效。
它不允许清除后代节点已有的 dirty 标记。

### Quad

`Quad.fromInput (parent, i)` 对应 `FromParent(i)`。
`Quad.fromInput (sibling, i)` 对应 `FromSibling(i)`。
常量对应 `Zero` 与 `One`。

`specialized_or_sound` 证明，在记录的分支条件成立时：

```text
materialize(specializeOr(raw, carry, old), new)
  = materialize(raw, new) OR materialize(carry, new)
```

分支条件是充分条件，不一定是必要条件。
例如 `raw = carry` 时，两条分支相同，不需要限制新输入。
`identical_or_needs_no_guard` 证明这个例子。

`concrete_accept_sound` 证明接受状态可安全保存为常量。
`Step` 组合节点谓词、前驱输入、descendant 传播和接受状态。
`quad_step_sound` 证明一个状态的专门化正确。
`quad_program_reuse` 证明任意有限状态列表的组合正确。
`quad_initial_correct` 证明首次专门化也与布尔求值一致。

quad 复用时必须用新输入物化投影。
该定理不声称整个输出向量保持不变。
有效选择器的接受状态没有向外的 descendant 边。
`Step.expr` 和 `Step.specialize` 均让接受状态覆盖传播设置，匹配 Rust 的分支顺序。

### 选择器状态递推

`Selector` 包含 compound 谓词及 descendant、child、adjacent sibling 三种组合关系。
compound 谓词的内部实现是参数。
`oracle` 直接解释选择器，descendant 通过祖先搜索求值。
`compiled` 使用 parent 与 sibling 的状态求值。

模型给每个选择器前缀保存一对结果：本地匹配位和包含当前节点的祖先匹配位。
Rust 根据向外的组合关系只保存其中一个结果。
descendant 读取祖先匹配位，child 与 adjacent sibling 读取本地匹配位。
接受状态读取本地匹配位。

`Context` 是 parent 与 previous-sibling 链接的有限展开。
共享节点可在展开中重复出现。模型不包含节点 ID 或可变 arena。
对 `Context` 归纳，`compiled_correct` 同时证明两个结果与直接解释器一致。
`accept_correct` 排除把祖先匹配误当作当前节点匹配的情况。

## 修改失效与 dirty 遍历

[Incremental.lean](CssProofs/Incremental.lean) 使用 first-child/next-sibling 有限森林。
`Ready` 表示缓存与当前缓存输入一致，或者节点已有 dirty 标记。
`subtreeDirty = false` 还要求后代没有待处理修改。

- `edit_facts_preserves_ready` 证明节点谓词修改会标记该节点及祖先。
- `invalidate_suffix_ready` 证明后续兄弟可以同时改变谓词和 sibling 输入。
- 插入、删除和任意有限修改序列都保持 `Ready`。
- `sweep_correct` 证明一次前序遍历终止，并使所有缓存输出正确。
- `trace_then_sweep` 组合混合修改序列与遍历证明。

遍历只在输出改变时传播输入变化。节点输出不变时，遍历仍处理已有 dirty 后代。
模型把待传播变化作为递归参数。Rust 会写入节点标记。
两种表示之间的完整源码精化尚未完成。

## 证明边界

已完成的定理没有未证明占位项。两套构建均检查公理依赖。
完整 Rust 引擎仍需以下连接证明：

1. CSS parser、索引分配和节点谓词与选择器模型一致。
2. 实际状态数组及缓存条件来自同一次有效求值，并满足复用定理的前提。
3. Rust arena 能展开为有限森林，且实际构造和修改维护节点所有权及父子关系。
4. `build_subtree`、`add`、`remove`、属性及 pseudo 更新维护模型不变量。
5. 实际 `recompute_node`、传播逻辑与已证明的森林遍历一致。

源码证明已覆盖输入条件检查、quad 局部转换和 `Dom::mark`。
`requirements_accept` 现在显式拒绝长度不等的数组。
其源码证明覆盖任意合法 slice 长度、循环终止、索引安全和整数加法安全。

`Dom::mark` 的证明要求父节点内部 ID 小于子节点 ID。
证明还使用 HashMap 的函数式接口模型。它不验证标准库的哈希表实现。
构造和修改代码维护上述 ID 前提的源码证明仍待完成。

Kani 保留固定长度局部检查。差分测试检查 mutation trace 与 naive oracle 的输出。
测试不能替代上述剩余源码证明。
