# Neovim（coc + rust-analyzer）下 Rust 常用操作速查

本文针对当前环境：**Neovim + coc.nvim + coc-rust-analyzer + Vimspector**（及 `init.vim` 中的按键）。未单独设置时，**`<leader>` 默认为 `\`**，**`<space>` 前缀**为 coc 列表快捷键（见下文）。

更细的**调试**说明见同目录 **[nvim-coc-rust-debug.md](./nvim-coc-rust-debug.md)**。

---

## 1. 符号、调用关系与跳转

### 1.1 LSP 跳转（最常用）

在 Rust 缓冲区内，光标放在标识符上：

| 按键 / 命令 | 作用 |
|-------------|------|
| **gd** | 跳转到**定义**（`textDocument/definition`） |
| **gy** | 跳转到**类型定义**（如推断出的类型、trait 等） |
| **gi** | 跳转到**实现**（如 trait 方法的实现） |
| **gr** | 列出**引用**（谁在用这个符号），在列表中选择跳转 |

配合 **`:help jumpto`** 类习惯：可用 **`<C-o>`** / **`<C-i>`**（或 `:prev` / `:tnext` 等）在跳转历史中来回。

### 1.2 悬停文档与当前符号

| 按键 / 行为 | 作用 |
|-------------|------|
| **K** | 显示 **hover**（类型、文档注释）；若无 LSP 则退回 Vim 原生的 `K` |
| **光标停留**（`CursorHold`） | 高亮当前词及其**引用**（`coc` 的 `highlight`） |

### 1.3 当前文件结构 / 工作区符号（函数名、模块一览）

| 按键 | 作用 |
|------|------|
| **\<space>o** | **Outline**：当前缓冲区的符号树（函数、结构体、impl 等），可搜索、跳转 |
| **\<space>s** | **工作区符号**（跨文件模糊搜符号名） |

适合快速「这个文件有哪些函数」「全仓库找一个函数名」。

### 1.4 调用关系怎么查？

rust-analyzer **没有**像「一键调用图」那样固定在单个键上，常用组合是：

- **gr**：看谁**调用/使用**该函数或类型（引用列表）。
- **gd → 再 gr**：从定义处看引用，或从调用处跳定义再展开。
- **\<space>o**：先看本文件结构，再 **gd / gr** 跟进。

需要 **调用层次（Call Hierarchy）** 时：执行 **`:CocList commands`** 或命令行 **`:CocCommand `** 后按 Tab 补全，查找名称里带 **call hierarchy** / **rust-analyzer** 的命令（随 coc-rust-analyzer 版本可能略有差异）。

### 1.5 Cargo 与工程

| 方式 | 作用 |
|------|------|
| **`:CocCommand rust-analyzer.openCargoToml`** | 打开当前相关 **`Cargo.toml`** |
| **`:CocCommand rust-analyzer.reloadWorkspace`** | 工程结构大变（增删 crate、改 workspace）后**重载**分析 |

---

## 2. 运行与调试

### 2.1 运行（不挂调试器）

| 按键 / 命令 | 作用 |
|-------------|------|
| **\<leader>rr** | **`:CocCommand rust-analyzer.run`**：列出当前文件可用的 runnable（二进制、测试等），选择后通常在终端执行 |
| **终端里手动** | **`:terminal`**（你映射了 `te`）后执行 `cargo run` / `cargo test` 等 |

### 2.2 调试

| 按键 / 命令 | 作用 |
|-------------|------|
| **`:CocCommand rust-analyzer.debug`** | 选择 runnable，用 **Vimspector + CodeLLDB** 启动调试（需已按 [nvim-coc-rust-debug.md](./nvim-coc-rust-debug.md) 配置） |
| **\<leader>dd** | **`:call vimspector#Launch()`**：按 **`.vimspector.json`** 手动选配置启动（不经过 rust-analyzer 列表） |
| **\<leader>dx** | **`:VimspectorReset`**：结束调试 |

单步、断点、F 键等见 **nvim-coc-rust-debug.md**（HUMAN 映射：F5/F9/F10/F11/F12 等）。

### 2.3 与「只运行」相关的其它 coc 能力

- CodeLens（运行测试等）：见 **§4.3 \<leader>cl**。
- 构建/检查：rust-analyzer 通常在保存时做 **`cargo check`**（诊断），不必每次手敲，除非你要完整 **`cargo build --release`** 再在终端执行。

---

## 3. 代码快捷操作（格式化、重构、补全）

### 3.1 补全与插入模式

| 按键 | 作用 |
|------|------|
| **Tab / Shift+Tab** | 补全菜单**下一项 / 上一项**（菜单打开时）；否则 Tab 仍缩进 |
| **Enter** | 补全打开时**确认**所选项 |
| **Ctrl+Space** | **手动触发**补全刷新 |

### 3.2 格式化与导入

| 按键 / 命令 | 作用 |
|-------------|------|
| **\<leader>f**（可视模式或配合选中） | **格式化选中**（`coc-format-selected`） |
| **`:Format`** | **格式化整个缓冲区**（异步） |
| **`:OR`** | **Organize Imports**（整理 `use`，依赖 rust-analyzer） |

### 3.3 重命名与代码动作

| 按键 | 作用 |
|------|------|
| **\<leader>rn** | **重命名**符号（全工程一致重命名） |
| **\<leader>a**（选区） / **\<leader>a**（普通模式按提示选范围） | 对**选中范围**应用 **Code Action** |
| **\<leader>ac** | 光标处的 **Code Action**（快速修复、生成 impl 等常在这里） |
| **\<leader>as** | 作用于**整个缓冲区**的 Code Action |
| **\<leader>qf** | 对当前行诊断应用**首选 QuickFix** |
| **\<leader>re** / **\<leader>r**（选区） | **重构**类 Code Action |

Rust 里 **\<leader>ac**、**\<leader>qf** 使用频率很高（未使用变量、类型错误提示的自动修复等）。

### 3.4 诊断跳转

| 按键 | 作用 |
|------|------|
| **[g** / **]g** | 上一条 / 下一条 **诊断**（错误、警告） |
| **\<space>a** | **`:CocList diagnostics`**：当前工作区诊断列表 |

### 3.5 基于语义的「文本对象」（函数 / 类级选区）

可视模式或 operator 待定时：

| 按键 | 作用 |
|------|------|
| **if / af** | **函数** inner / around（选函数体或含签名整段） |
| **ic / ac** | **类/结构体**等 inner / around |

配合 **d**、**y**、**\<leader>f** 等使用，例如 **`daf`** 删除整个函数。

### 3.6 折叠

| 命令 | 作用 |
|------|------|
| **`:Fold`** | 按 LSP 语义**折叠**当前缓冲区 |

---

## 4. 其它高频操作

### 4.1 Coc 列表与命令面板

| 按键 | 作用 |
|------|------|
| **\<space>c** | **命令列表**（含各类 `CocCommand`，可搜 **rust-analyzer**） |
| **\<space>e** | **扩展管理**（coc 插件） |
| **\<space>j** / **\<space>k** | 在打开的 **coc 列表**里下一项 / 上一项的默认动作 |
| **\<space>p** | **恢复上一次**的 coc 列表 |

### 4.2 CodeLens（行内「Run Test」等）

若 rust-analyzer 在行上方显示 **Run | Debug** 等 Lens：

| 按键 | 作用 |
|------|------|
| **\<leader>cl** | 执行**当前行**的 CodeLens 动作 |

### 4.3 浮动窗口滚动

文档/悬浮窗打开时：

| 按键 | 作用 |
|------|------|
| **Ctrl+f** / **Ctrl+b** | 在 **coc 浮窗**内上下滚动（有浮窗时；否则保持原编辑器行为） |

### 4.4 状态栏

你的配置里 **statusline** 含 **`coc#status()`** 与 **`b:coc_current_function`**，可在底部看到 **LSP 状态**与**当前函数名**（依赖语言服务器支持）。

### 4.5 侧栏文件树（非 coc 专属）

| 按键 | 作用 |
|------|------|
| **\<leader>t**（或 Neovide 下 **tt**） | **Tagbar** / **NERDTree** 等（你配置里 **Tagbar** 为 `\<leader>t`；**NERDTree** 在 Neovide 下为 `tt`） |

Rust 的「符号大纲」仍以 **\<space>o**（coc outline）为主；Tagbar 对 Rust 需自行配 **ctags/Universal Ctags**，不如 outline 准。

### 4.6 升级与分析工具链

| 命令 | 作用 |
|------|------|
| **`:CocCommand rust-analyzer.upgrade`** | 升级 **rust-analyzer**（行为以扩展说明为准） |

### 4.7 重载 coc

改 **`coc-settings.json`** 或扩展后：

| 命令 | 作用 |
|------|------|
| **`:CocRestart`** | 重启 coc 服务 |

---

## 5. 配置与文档索引

| 路径 | 内容 |
|------|------|
| `~/.config/nvim/init.vim` | 上述映射、插件列表 |
| `~/.config/nvim/coc-settings.json` | `rust-analyzer.*`、debug 等 |
| 本仓库 **`.vimspector.json`** | CodeLLDB / `launch` 等 |
| **docs/nvim-coc-rust-debug.md** | 调试配置、F 键、汇编视图 FAQ |

---

## 6. 速记卡片（Rust 日常）

| 目的 | 优先试试 |
|------|-----------|
| 跳到定义 | **gd** |
| 谁在用 | **gr** |
| 看类型/文档 | **K** |
| 本文件函数列表 | **\<space>o** |
| 报错下一条 | **]g** |
| 小灯泡修复 | **\<leader>ac** 或 **\<leader>qf** |
| 重命名 | **\<leader>rn** |
| 整理 import | **:OR** |
| 跑当前文件 runnable | **\<leader>rr** |
| 调试 | **:CocCommand rust-analyzer.debug** |
| 打开 Cargo.toml | **:CocCommand rust-analyzer.openCargoToml** |

若某键与本地其它插件冲突，以 **`:verbose nmap <键>`** 查归属后再改 **init.vim**。
