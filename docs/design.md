# CSS 增量匹配器设计与验证

本文描述当前实现中的四种增量变体：`bit`、`tri`、`recursive-tri` 和
`quad`。`naive` 不属于优化变体；它是独立解释执行的正确性 oracle。

实现入口：

- 选择器编译：[compiler.rs](../src/clean/compiler.rs)
- DOM、失效传播和四种 engine：[engine.rs](../src/clean/engine.rs)
- tri/quad 的纯逻辑和 Kani proof harness：[logic.rs](../src/clean/logic.rs)
- trace 格式：[trace.rs](../src/clean/trace.rs)
- 普通与语料回归：[tests.rs](../src/clean/tests.rs)

## 1. 共享语义

### 1.1 选择器状态机

选择器：

```css
.a .b > .c
```

被编译为三个状态 $q_0$、$q_1$、$q_2$。令 $p_c(n)$ 表示节点 $n$
满足 compound predicate $c$，则 local transition 为：

$$
\begin{aligned}
q_0(n) &= p_{\mathtt{.a}}(n), \\
q_1(n) &= P_n[q_0] \land p_{\mathtt{.b}}(n), \\
q_2(n) &= P_n[q_1] \land p_{\mathtt{.c}}(n),
\qquad q_2 \in Q_{\mathrm{accept}}.
\end{aligned}
$$

descendant combinator 还为中间状态加入传播语义：

$$
O_n[q_0] = q_0(n) \lor P_n[q_0].
$$

child combinator 不传播；adjacent sibling 从前一个 sibling 的输出通道读取。
每个 selector 使用独立状态区间，因此 accept state 可以直接映射回原 selector。

### 1.2 节点输入与输出

每个 DOM 节点 $n$ 有两个输入通道和一个物化输出：

$$
P_n, S_n, O_n \in \mathbb{B}^{|Q|}, \qquad \mathbb{B}=\{0,1\},
$$

其中 $P_n$ 是 parent output，$S_n$ 是 previous-sibling output，$O_n$
是当前节点的输出。

`O` 同时包含中间状态和 accept state。accept state 只表示“该 selector
在当前节点匹配”，不能把祖先的 accept bit 当成当前节点匹配。

### 1.3 Dirty 不变量

节点状态按强度排序：

$$
\mathsf{Clean} \;<\; \mathsf{InputChanged} \;<\; \mathsf{NodeChanged}.
$$

- `InputChanged`：节点事实未变，只有 `P` 或 `S` 变化。
- `NodeChanged`：tag/class/id/attribute/pseudo/兄弟位置发生变化。
- 高强度状态不能被低强度状态覆盖。
- `subtree_dirty = true` 表示该子树中至少有一个待处理节点。

父输出变化会使 children 的 parent input 失效；节点输出变化会使下一个 sibling
的 sibling input 失效。插入和删除还会使后续 sibling 的 `nth-*` 位置失效。

## 2. Bit：物化 bitvector 基线

### 2.1 状态

Bit 只缓存物化输出：

$$
\mathsf{cache}_{\mathrm{bit}}(n) = O_n.
$$

### 2.2 重算规则

$$
\mathsf{step}_{\mathrm{bit}}(n)=
\begin{cases}
F_n(P_n,S_n), & d_n \in \{\mathsf{NodeChanged},\mathsf{InputChanged}\},\\
O_n, & d_n=\mathsf{Clean}.
\end{cases}
$$

如果新旧 `O` 相同，不向 children/sibling 传播失效；否则标记其输入变化。

### 2.3 特点

- 控制流简单，是增量算法基线。
- 能利用“输出没变”剪枝。
- 不知道程序实际读取了哪些输入，因此 input bit 的无关变化也会重算。

## 3. Tri：输入依赖抽象

### 3.1 抽象域

Tri 使用三点抽象域：

$$
\mathcal{R}=\{\bot,0,1\},
$$

其中 $\bot$ 对应 `Unused`，$0$ 对应 `Zero`，$1$ 对应 `One`。

含义：

- `Unused`：本次执行没有读取该 bit。
- `Zero`：读取了该 bit，且分支依赖其为 `0`。
- `One`：读取了该 bit，且分支依赖其为 `1`。

节点谓词采用短路语义。例如：

$$
p_{\mathtt{.b}}(n) \land P_n[q_0].
$$

当 `self(.b) == false` 时不会读取 `parent[q0]`，所以对应 requirement 是
`Unused`。

### 3.2 Skip 判定

令具体化关系 $\gamma:\mathcal{R}\to\mathcal{P}(\mathbb B)$ 为：

$$
\gamma(\bot)=\{0,1\},\qquad
\gamma(0)=\{0\},\qquad
\gamma(1)=\{1\}.
$$

对于 requirement vector $R$ 和新输入 $I'$：

$$
\operatorname{compatible}(R,I')
\iff \forall i,\ I'[i]\in\gamma(R[i]).
$$

所有 requirement 均 compatible 时复用缓存输出。

### 3.3 Hoare triple

令 $R$ 是上次执行得到的 requirements，$F_n$ 是节点状态机，
$O_n=F_n(I)$。Tri skip 的部分正确性规格为：

$$
\left\{
\begin{array}{l}
O_n=F_n(I) \\
\land\ R\ \text{记录了执行 }F_n(I)\text{ 时所有实际读取} \\
\land\ \operatorname{compatible}(R,I')
\end{array}
\right\}
\quad \mathsf{skip}\quad
\left\{F_n(I')=O_n\right\}.
$$

该三元组依赖一个关键前提：所有影响控制流或结果的 input read 必须被记录。
因此实现必须保持 Rust 短路顺序，不能在 `self(predicate)` 为 false 时提前读取
parent/sibling bit。

## 4. Recursive-tri：在进入子树前应用 tri

Tri 在节点被访问后才判断能否 skip。Recursive-tri 把判定提前到父节点传播阶段：

$$
P_c \neq P'_c
\Longrightarrow
\begin{cases}
\text{不调度 }c, & \operatorname{compatible}(R_c,P'_c),\\
d_c\gets\mathsf{InputChanged}, & \text{otherwise}.
\end{cases}
$$

同样的规则用于 next sibling。

### Hoare triple

$$
\left\{
d_c=\mathsf{Clean}\land\operatorname{compatible}(R_c,P'_c)
\right\}
\quad \mathsf{do\_not\_schedule}(c)\quad
\left\{
O'_c=O_c\land\neg\operatorname{invalidate}(\operatorname{desc}(c))
\right\}.
$$

当前实现采用逐层 admission check，而不是额外保存一个显式 folded
`recursive_tri_input` 数组。若 child 被跳过，它的输出不变，因此失效不会继续进入
更深层；效果上形成递归剪枝。显式的 subtree requirement fold 可作为后续优化，
但不是当前正确性的前提。

## 5. Quad：输出函数抽象

Tri 抽象输入使用情况；Quad 进一步抽象输出：

$$
\mathcal{Q}=\{\mathbf 0,\mathbf 1\}
\cup\{\pi^P_i\mid 0\le i<|Q|\}
\cup\{\pi^S_i\mid 0\le i<|Q|\}.
$$

这里 $\pi^P_i$ 对应 `FromParent(i)`，$\pi^S_i$ 对应
`FromSibling(i)`。

物化函数：

$$
\begin{aligned}
M(\mathbf 0,P,S)&=0, & M(\mathbf 1,P,S)&=1,\\
M(\pi^P_i,P,S)&=P[i], & M(\pi^S_i,P,S)&=S[i].
\end{aligned}
$$

因此纯传播节点保存的是“输出来自哪个输入”，而不是当时的布尔值。输入变化时，
只调用 `M` 即可得到新输出，不需要重新执行 selector predicates。

### 5.1 两阶段计算

non-propagate rule 必须先于 propagate rule，避免传播结果污染本地
transition。当前实现显式分两步：

1. 计算 local/predecessor transition，得到 `raw`。
2. 对 descendant self-propagation 计算
   $\mathit{raw}\lor\pi^P_{\mathit{state}}$。

### 5.2 OR 专门化

`QuadValue` 只能表示常量或单个 projection，不能直接表示任意 OR。因此根据当前
输入选择一条分支，并记录维持该选择所需的 decision：

$$
\operatorname{specializeOr}(r,c,I)=
\begin{cases}
(r,\ r=1), & M(r,I)=1,\\
(c,\ r=0), & M(r,I)=0.
\end{cases}
$$

对应 Hoare triple：

$$
\left\{
(q,d)=\operatorname{specializeOr}(r,c,I)
\land\operatorname{holds}(d,I')
\right\}
\quad x\gets M(q,I')\quad
\left\{
x=M(r,I')\lor M(c,I')
\right\}.
$$

accept state 是外部可观察匹配结果，必须具体化为 `Zero/One`：

$$
\left\{
(c,d)=\operatorname{specializeConcrete}(q,I)
\land\operatorname{holds}(d,I')
\right\}
\quad x\gets M(c,I')\quad
\left\{x=M(q,I')\right\}.
$$

若 decision 不再成立，节点回退到完整重算并生成新的 quad output。

## 6. Mutation、pseudo 与结构变化

- class/id/equality attribute 更新属于 `NodeChanged`，不能用 input requirement 跳过。
- 更新后会先做局部求值；若输出和匹配均不变，则不传播到子树。
- `is_hover_root` 与 `is_hovered_root` 在输入边界统一。
- `:hover` 向下派生，`:focus-within` 向上派生；派生值变化的节点按
  `NodeChanged` 处理。
- sibling 插入/删除会重算后续节点的 `nth-child`/`nth-of-type` 位置。

## 7. Kani 形式化验证

Kani harness 位于 [logic.rs](../src/clean/logic.rs) 的 `cfg(kani)` 模块中。
这些证明直接调用生产代码中的纯函数，而不是复制一份规格实现。
安装和命令格式参考 [Kani 官方文档](https://model-checking.github.io/kani/)。

| Harness | 证明目标 |
|---|---|
| `tri_requirement_reuse_is_sound` | `Zero/One` requirement compatible 时输入 bit 没变 |
| `quad_projection_materializes_current_input` | parent/sibling projection 精确读取对应 bit |
| `specialized_or_refines_boolean_or` | decision 成立时，专门化输出等价于布尔 OR |
| `concrete_accept_reuse_is_sound` | decision 成立时，具体 accept bit 可安全复用 |

运行：

```sh
cargo install --locked kani-verifier
cargo kani setup
cargo xtask verify
```

CI 使用官方 `model-checking/kani-github-action` 执行全部 proof harness。

Kani 的证明边界是固定长度 2 的 parent/sibling bitvector。状态 index 在该范围内
完全符号化，所有布尔输入和 quad tag 均被穷举。由于各状态的 transition 是逐 bit
独立组合的，这证明覆盖局部转换定律；它不等价于证明整个 CSS parser 或任意大小
DOM 的端到端正确性。

## 8. Differential validation

形式化局部定律之外，项目还使用独立 naive oracle 做端到端验证：

```sh
cargo xtask check
cargo xtask corpus
cargo xtask run --all
```

关键普通测试：

- `tri_and_recursive_tri_skip_unread_parent_input`
- `quad_composes_descendant_propagation_without_recompute`
- `engines_agree_on_sibling_and_nth_invalidation`
- `checked_in_corpus_has_engine_parity`

完整 corpus 要求 13 个站点上四种变体与 naive 的最终匹配完全一致，并额外断言
`quad.recomputed_nodes <= tri.recomputed_nodes`。

## 9. 当前非目标

- `:has`、general sibling、`:last-*`、`:nth-last-*` 和 selector-list pseudo。
- 对 CSS parser 的形式化证明。
- 对无界 DOM 大小的归纳证明；这更适合 Verus/Lean/Coq 中的递归数据结构模型。
- 并发 mutation；当前 runner 是单线程顺序 trace 语义。
