# Rust 工程化与社区实践概览

本文从**范式、目录、规范、社区约定**四方面归纳 Rust 在商业与开源项目中常见做法，便于搭仓库与团队协作时对齐预期。细节以官方文档与具体团队规范为准。

---

## 1. 商业与工程化中常见的编程范式

Rust 不强制单一范式，实际项目常**混合使用**，由 **所有权、类型系统、trait** 统一约束。

### 1.1 以所有权与借用为核心的命令式编程

- 默认写法：可变状态 + 显式生命周期/借用，由编译器保证内存与数据竞争安全。
- 适合：系统服务、游戏引擎内核、嵌入式、大部分业务逻辑。

### 1.2 类型驱动与「使非法状态无法表示」

- 用 **`enum`**、**新类型**、**私有字段 + 构造函数** 把不变量放进类型，而不是到处 `if` 校验。
- 适合：领域建模、协议状态机、解析结果（`Option` / `Result` 链）。

### 1.3 函数式风格（不改变范式本质）

- **`Iterator`**、**`map` / `and_then` / `?`** 链式处理，减少中间可变集合。
- 常与命令式混用：热点路径可保留循环，外围用迭代器表达数据流。

### 1.4 基于 trait 的多态与扩展点

- **静态分发**（泛型 + trait bound）：性能敏感路径。
- **动态分发**（`dyn Trait`）：插件、回调、对象数量在运行时变化。
- **与 OOP 的差异**：无继承，用 **组合 + trait + 默认实现** 复用行为。

### 1.5 错误处理范式：库 vs 应用

- **库（library crate）**：倾向 **`Result<T, E>`** 与**具体错误类型**（或 `thiserror`），不把策略性强塞给调用方。
- **应用（binary）**：可在边界用 **`anyhow::Error`** 等聚合错误，在 `main` 或顶层统一汇报/退出码。
- 商业项目常在规范里写明：**何时新建错误枚举、是否允许 `unwrap`（通常仅测试/已知不变量）**。

### 1.6 并发与异步范式

- **并发**：`std::thread`、消息传递、`Arc<Mutex<_>>` 等；强调 Send/Sync 边界。
- **异步**：I/O 密集常用 **`async`/`.await` + 运行时（如 Tokio）**；与同步代码分界要清晰（避免在异步里阻塞线程池）。
- 商业项目需约定：**是否全栈 async、阻塞调用放哪里、超时与取消策略**。

### 1.7 元编程与 DSL

- **声明宏 `macro_rules!`**、**过程宏**（derive 等）用于减少样板代码；团队规范里常限制「谁有权写过程宏、审查流程」。

### 1.8 `unsafe` 的使用边界

- 原则：**默认不用**；仅在 FFI、底层抽象或可证明安全的不变量封装中使用，并 **文档说明不变量 + 通常配套 `SAFETY` 注释**。
- 商业化项目多要求 **code review 对 unsafe 单独标红**。

---

## 2. 目录与仓库结构如何定

### 2.1 单 crate 库（library）

典型布局（Cargo 约定）：

```text
my-lib/
  Cargo.toml
  README.md
  LICENSE
  src/
    lib.rs          # 库根，常放 re-export 与模块树
    foo.rs          # 或 mod foo; 对应子模块
    foo/
      mod.rs
  tests/            # 集成测试，每个文件独立 crate
    integration.rs
  examples/         # 示例，cargo run --example xxx
  benches/          # 基准测试（常配合 criterion 等）
```

要点：**对外 API 在 `lib.rs` 控制可见性**（`pub` / `pub(crate)`）。

### 2.2 单 crate 可执行程序（binary）

```text
my-app/
  Cargo.toml
  src/
    main.rs
```

若 CLI 较复杂，常见 **`src/main.rs` 很薄，逻辑在 `src/lib.rs` 或 `src/cli.rs` 等**，便于集成测试与复用。

### 2.3 多 crate：**Workspace**（中大型与商业项目极常见）

根目录 `Cargo.toml`：

```toml
[workspace]
members = ["crates/foo", "crates/bar", "apps/cli"]
resolver = "2"   # 建议显式（依赖解析行为）
```

常见拆分：

| crate 类型 | 职责 |
|------------|------|
| `*-core` / `*-domain` | 领域模型、与 IO 无关的逻辑 |
| `*-infra` / `*-adapter` | 数据库、HTTP 客户端、消息队列 |
| `*-api` / `server` | HTTP/gRPC 入口 |
| `cli` | 命令行入口 |

原则：**依赖方向清晰**（避免环依赖），**二进制薄、库可测**。

### 2.4 与本学习仓库的对比

本仓库根 `Cargo.toml` 使用 **`[workspace]` + 多 `members`**，同时根上还有 `[package]`，属于**「根目录既参与 workspace 又是一个包」**的混合形态；练习项目可接受。纯商业 monorepo 更常见的是：

- 根目录**仅 workspace**，**不带** `[package]`；各 crate 放在 `crates/` 或 `apps/` 下。

### 2.5 其它常见目录（约定俗成）

| 路径 | 用途 |
|------|------|
| `.github/workflows/` | CI（`cargo test`、`clippy`、`fmt --check`） |
| `docs/` | 设计文档、ADR |
| `benches/`、`examples/` | 性能与示例（与 Cargo 约定一致） |
| `build.rs` | 构建脚本（FFI、代码生成） |

---

## 3. 团队内还应定的开发规范（工程化清单）

下列项建议在 **README / CONTRIBUTING / 内部 Wiki** 写清，避免每人一套习惯。

### 3.1 工具链版本

- **Rust 版本**：固定 **`rust-toolchain.toml`**（或文档写明 `stable` + 最低支持版本 **MSRV**）。
- **`edition`**：`2021`（新项目默认）；升级 edition 走团队节奏。

### 3.2 格式化与静态检查

- **`rustfmt`**：默认配置或提交 `.rustfmt.toml`；CI 中 **`cargo fmt --all -- --check`**。
- **`clippy`**：CI 中 **`cargo clippy -- -D warnings`**（或分阶段引入）；可维护 **`clippy.toml`** 放宽个别 lint。

### 3.3 依赖与安全

- **语义化版本**：库对外 API 遵守 **SemVer**；`Cargo.lock` **二进制提交、库 crate 常不提交**（团队可统一规定）。
- **供应链**：可选 **`cargo deny`**、**`cargo audit`**（或 GitHub Dependabot）纳入 CI。

### 3.4 测试策略

- **单元测试**：同文件 `mod tests` 或 `src/*_test.rs`。
- **集成测试**：`tests/` 目录。
- **文档测试**：`///` 中的代码块由 `cargo test` 执行。
- 约定：**覆盖率门槛、是否要求模糊测试（如 `cargo fuzz`）**。

### 3.5 文档与 API

- 公共 API 使用 **`///` 文档注释**，复杂行为写 **Examples**。
- 可选 **`cargo doc --open`** 作为发布前检查。

### 3.6 日志、配置、可观测性

- 约定：**`tracing` / `log` 其一**、日志级别、结构化字段。
- **配置来源**（环境变量、配置文件）及**密钥不落库**。

### 3.7 性能与发布

- **Release 配置**：`Cargo.toml` 中 `[profile.release]`（`lto`、`codegen-units` 等）是否统一。
- **功能开关（feature flags）**：可选依赖与平台差异用 `Cargo features` 表达，并在文档列出。

---

## 4. 社区约定俗成与参考标准

### 4.1 官方与半官方参考

- **[The Rust Book](https://doc.rust-lang.org/book/)**：语言与所有权基础。
- **[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)**：库作者面向用户的约定（命名、错误类型、`Debug`/`Display`、错误消息、稳定性等）。
- **[Cargo Book](https://doc.rust-lang.org/cargo/)**：工作区、特性、发布流程。
- **[Rust RFC 仓库](https://github.com/rust-lang/rfcs)**：语言与标准库演进的讨论范式（团队学习「为什么这样设计」很有用）。

### 4.2 命名与可见性（高频约定）

- **crate 名**：`snake_case`（crates.io 惯例）。
- **类型 / trait**：`PascalCase`；**函数、变量、模块**：`snake_case`。
- **常量**：`SCREAMING_SNAKE_CASE`。
- **`pub use` 重导出**：谨慎使用，作为**稳定 API 表面**的一部分来设计。

### 4.3 错误与类型生态中的「默认选择」

- 库：**`thiserror`** 派生错误类型较常见。
- 应用：**`anyhow`** 或自定义顶层错误较常见。
- 避免在库 API 中**泛用 `String` 作为错误类型**（不利于调用方匹配）。

### 4.4 异步生态约定

- 运行时：**Tokio** 在服务端生态占主导；选型后避免混用多个不兼容的运行时。
- **`async fn` in trait** 已随语言演进，新项目按当前 stable 能力与团队 MSRV 选择模式。

### 4.5 开源协作惯例

- **`LICENSE`**：明确许可证（MIT/Apache-2.0 双许可在 Rust 生态很常见）。
- **`CHANGELOG.md`**：遵循 [Keep a Changelog](https://keepachangelog.com/) 等格式。
- **`CONTRIBUTING.md`**：如何跑测试、提交 PR、commit 风格。
- **Issue / PR 模板**：分类 bug、feature、安全披露渠道（`SECURITY.md`）。

### 4.6 Clippy 与「惯用法」

- 社区倾向 **开启 clippy** 并逐步消除 warn；许多惯用法（如 **`cloned()`、`ok_or`** 等）来自 clippy 与书评。

---

## 5. 小结表

| 维度 | 建议抓手 |
|------|-----------|
| 范式 | 所有权 + 类型驱动 + trait 抽象；库/应用分层处理错误；async 与阻塞边界写清 |
| 目录 | 单 crate 用 `src/`、`tests/`；变大用 **workspace** 拆 crate，根目录尽量只当 workspace |
| 规范 | `rustfmt` + `clippy` + CI；`rust-toolchain` / MSRV；SemVer 与 `Cargo.lock` 策略 |
| 社区 | API Guidelines、Cargo Book、许可证与 CHANGELOG、内部 CONTRIBUTING |

若团队从零定规范，可先做 **最小集**：**`fmt` + `clippy -D warnings` + `cargo test` + `rust-toolchain.toml` + 根 README 说明 workspace 结构**，再按需加安全扫描与覆盖率。
