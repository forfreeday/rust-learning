# Neovim + coc.nvim + Rust 调试说明

本文汇总在本仓库使用 **Neovim、coc.nvim、rust-analyzer、Vimspector** 进行 Rust 调试时的插件关系、配置方式、常见问题与常用操作。

---

## 1. 涉及插件与分工

| 组件 | 作用 |
|------|------|
| **coc.nvim** | LSP 客户端，连接语言服务器、提供补全、命令、CodeLens 等。 |
| **coc-rust-analyzer** | 在 coc 中集成 **rust-analyzer**，提供 `rust-analyzer.run`、`rust-analyzer.debug` 等命令；调试时会解析 Cargo 产物路径并调用你选择的「调试后端」。 |
| **rust-analyzer** | Rust 语言服务器（由 coc-rust-analyzer 下载或指定路径），不负责图形化调试 UI，只提供 runnable / 可执行文件信息。 |
| **Vimspector** | 调试 UI 与会话管理，通过 **Debug Adapter Protocol (DAP)** 连接调试适配器。 |
| **CodeLLDB**（Vimspector gadget） | LLDB 的 DAP 适配器，用于调试 Rust 等原生程序；需在 Vimspector 中安装（如 `:VimspectorInstall` 或配置 `g:vimspector_install_gadgets`）。 |

关系简述：**coc-rust-analyzer** 决定「调哪个二进制」；**Vimspector + CodeLLDB** 负责「启动调试器、断点、单步」等。

---

## 2. 正确的调试配置

### 2.1 coc 设置（`~/.config/nvim/coc-settings.json`）

必须将 rust-analyzer 的调试运行时从默认的 `termdebug` 改为 `vimspector`（见下文「原因」），并指定与 `.vimspector.json` 里一致的配置名：

```json
"rust-analyzer.debug.runtime": "vimspector",
"rust-analyzer.debug.vimspector.configuration.name": "launch"
```

修改后建议执行 `:CocRestart` 或重启 Neovim。

### 2.2 Vimspector 项目配置（本仓库 `.vimspector.json`）

`rust-analyzer.debug` 会通过 `vimspector#LaunchWithSettings` 传入变量 **`${Executable}`**（以及需要时的 **`${Args}`**）。因此需要存在名为 `launch` 的配置（与上面的 `configuration.name` 一致），并使用 CodeLLDB 示例：

```json
"launch": {
  "adapter": "CodeLLDB",
  "configuration": {
    "request": "launch",
    "program": "${Executable}",
    "cwd": "${workspaceRoot}",
    "expressions": "native"
  }
}
```

另可根据需要保留固定路径的配置（例如本仓库中的 `Rust - Test`），用于 `:call vimspector#Launch()` 手动选择。

### 2.3 Neovim 侧（`~/.config/nvim/init.vim`）

- 已安装 **coc.nvim**、**coc-rust-analyzer**（如通过 `g:coc_global_extensions`）。
- 已安装 **vimspector**，并设置 `let g:vimspector_install_gadgets = [ ..., 'CodeLLDB' ]`（或已手动安装 CodeLLDB gadget）。
- `let g:vimspector_enable_mappings = 'HUMAN'` 会加载 **4.3** 节的 HUMAN 快捷键（`leader` 未设置时默认为 `\`）；也可改用 **4.4** 的 `VISUAL_STUDIO` 或 **4.5** 自订映射。

### 2.4 启动调试的推荐流程

1. `cargo build`（或让 rust-analyzer / 运行命令先产出 debug 二进制）。
2. 在源码行上打好断点。
3. 执行 `:CocCommand rust-analyzer.debug`，选择要调试的 runnable（如某测试或二进制）。

---

## 3. 问题原因与解决方式（`TermdebugCommand` / E492）

### 现象

执行 `CocCommand rust-analyzer.debug` 时出现类似：

```text
[coc.nvim]: Error on notification "runCommand": Vim:E492: Not an editor command: TermdebugCommand
```

### 原因

- **coc-rust-analyzer** 默认 **`rust-analyzer.debug.runtime` 为 `termdebug`**。
- 该模式会执行 Vim 命令 **`TermdebugCommand`**，依赖 Vim 自带的 **termdebug**（与 GDB 配合的终端调试界面）。
- **Neovim 不提供 `TermdebugCommand`**，因此报错 `E492: Not an editor command`。

### 解决方式

在 **Neovim** 下不要使用默认的 `termdebug`，改为：

- **`vimspector`**（本仓库已采用）：在 `coc-settings.json` 中设置 `rust-analyzer.debug.runtime` 为 `vimspector`，并配置 `.vimspector.json` 中的 `launch` + CodeLLDB；或
- **`nvim-dap`**：需单独安装配置 `nvim-dap` 与相应适配器，并在 coc 中把 runtime 设为 `nvim-dap`。

---

## 4. 调试常用操作与快捷键

### 4.0 先分清几件事：单步 ≠ 跳到下一个断点

调试里常见动作可以这样理解（与 IDE 里名称一致）：

| 概念（英文） | 白话 |
|-------------|------|
| **Continue（继续）** | 全速运行，直到**命中某个断点**、程序结束或再次暂停。**不是**「只执行一行」。 |
| **StepOver（单步跳过）** | 执行**当前这一行**；若该行调用了函数，**不进入**函数内部，把整个调用当成一步。 |
| **StepInto（单步进入）** | 执行当前行；若停在函数调用上，**跟入**被调函数内部。 |
| **StepOut（单步跳出）** | 从当前函数**一直执行到返回**（回到上层调用栈），中间不停在每一行。 |
| **Run to cursor（运行到光标）** | 从当前暂停位置继续跑，直到执行到**光标所在行**（相当于临时断点）。 |
| **Jump to next breakpoint（跳到下一断点）** | 光标/视图跳到**下一个断点所在位置**（浏览断点用），**不会**执行代码；与 Continue 不同。 |

**重要：** 在 Vimspector 里，**「跳到下一个 / 上一个断点」**对应 `<Plug>VimspectorJumpToNextBreakpoint` / `JumpToPreviousBreakpoint`。  
- **`VISUAL_STUDIO` 预设**里给它们绑了 **F8 / Shift+F8**。  
- **`HUMAN` 预设里没有绑定这两个功能**；此时 **F8** 被用作「函数断点」，**不是**「跳到下一断点」。若你需要该功能，见下文 **4.5** 自行映射。

### 4.1 Vimspector 的几种按键方案：HUMAN 是什么？

Vimspector 通过 **`g:vimspector_enable_mappings`** 选择**内置的一套预设**，不是单独某一个键：

| 取值 | 含义 |
|------|------|
| **`'HUMAN'`** | 一套偏「人类可读」的 F 键布局（见下表）；**没有**为「跳到下一断点」分配 F8。 |
| **`'VISUAL_STUDIO'`** | 对齐 Visual Studio：F5 继续、F9 断点、F10/F11 单步、**F8 跳到下一断点**、Shift+F11 跳出等；**F12 在 VS 预设里未用于 StepOut**（StepOut 是 Shift+F11）。 |
| 不设或设为空 | 不自动映射 F 键，可只用 `:command` 或自己 `nmap` 到 `<Plug>Vimspector…`。 |

因此：**HUMAN 就是一种可选的整套方案**；若你更习惯「F8 = 下一断点」且能接受 VS 风格，可改为 `let g:vimspector_enable_mappings = 'VISUAL_STUDIO'`（并通读其 F 键表，避免和 Mac 系统键冲突）。

### 4.2 Mac 外接键盘：F11 / F12 冲突与 F7 是否占用

**系统与终端层：**

- macOS 常把 **F11**（显示桌面）、**F12**（部分键盘/系统功能）留给系统或「标准功能键」关闭时的媒体键；外接键盘也可能把顶行发给系统，导致 Neovim **收不到 F11/F12**，或先触发桌面/调度中心。
- **处理思路：** 在 **系统设置 → 键盘** 中开启 **「将 F1、F2 等键用作标准功能键」**（或仅对外接键盘），减少被系统拦截；或在终端/iTerm2 里关闭/调整把 F11/F12 交给系统的选项。

**Vimspector 是否占用 F7：**

- 在 `vimspector/plugin/vimspector.vim` 的 **`HUMAN` / `VISUAL_STUDIO` 预设里都没有绑定 F7**。  
- 即 **F7 在 Vimspector 默认映射中是空闲的**，可把单步类操作改绑到 F7 等键（见 **4.5**）。若仍无效，再查终端或 macOS 是否拦截 F7。

**与 Vimspector 的叠加冲突：**

- 你在 `init.vim` 里还有 **`<LocalLeader>F11` / `<LocalLeader>F12`** 映射到**调用栈上一帧 / 下一帧**，同样依赖 F11/F12，在 Mac 上容易与系统或「标准功能键」行为撞车，可考虑一并改绑（例如用 `<leader>` + 字母）。

### 4.3 Vimspector「HUMAN」映射（含白话说明）

以下以 **`g:vimspector_enable_mappings = 'HUMAN'`** 为准。`<leader>` 未改时默认为 **`\`**；`<LocalLeader>` 未改时默认也常为 **`\`**。

| 按键 | 插件中的名称 | 白话（调试时在干什么） |
|------|----------------|------------------------|
| **F5** | Continue | **继续跑**：直到下一个断点、结束或暂停；不是逐行。 |
| **\<leader>F5** | Launch | 弹出/走**启动流程**（选配置启动调试）。 |
| **F3** | Stop | **停掉**调试会话。 |
| **F4** | Restart | **重新启动**当前调试（再跑一遍）。 |
| **F6** | Pause | **暂停**正在运行的被调试程序（若适配器支持）。 |
| **F9** | ToggleBreakpoint | 在当前行**加/消普通断点**。 |
| **\<leader>F9** | ToggleConditionalBreakpoint | **条件断点**（满足表达式才停）。 |
| **F8** | AddFunctionBreakpoint | **按函数名加断点**（不是「跳到代码里下一个断点位置」）。 |
| **\<leader>F8** | RunToCursor | **运行到光标行**（临时当作断点）。 |
| **F10** | StepOver | **单步跳过**：执行本行，**不进**子函数。 |
| **F11** | StepInto | **单步进入**：若本行有调用，**进入**被调函数。 |
| **F12** | StepOut | **单步跳出**：执行完当前函数并回到上层。 |

**对照记忆（HUMAN）：**  
- 想**一行一行跟逻辑、但不钻进函数** → **F10**。  
- 想**跟进函数内部** → **F11**（Mac 上若被系统抢走，见 **4.5**）。  
- 想**快速跑出当前函数** → **F12**。  
- 想**一直跑到下一个断点** → **F5（Continue）**，**不是** F10。  
- 想**光标跳到列表里的下一个断点位置**（不执行）→ HUMAN **默认没绑键**，用 **4.5** 或改用 **VISUAL_STUDIO** 的 F8。

### 4.4 可选：`VISUAL_STUDIO` 预设（含「跳到下一断点」）

若改为 `let g:vimspector_enable_mappings = 'VISUAL_STUDIO'`，与调试相关的 F 键大致为：

| 按键 | 名称 | 白话 |
|------|------|------|
| F5 | Continue | 继续直到断点 |
| Shift+F5 | Stop | 停止 |
| Ctrl+Shift+F5 | Restart | 重启 |
| F9 | ToggleBreakpoint | 切换断点 |
| **F8** | **JumpToNextBreakpoint** | **光标跳到下一个断点**（不执行代码） |
| **Shift+F8** | **JumpToPreviousBreakpoint** | 光标跳到上一个断点 |
| F10 | StepOver | 单步跳过 |
| F11 | StepInto | 单步进入 |
| Shift+F11 | StepOut | 单步跳出 |
| Ctrl+F10 | RunToCursor | 运行到光标 |

注意：VS 预设下 **F12 不用于 StepOut**，且 **F8 是跳断点浏览**，与 HUMAN 里 F8 的「函数断点」完全不同；换预设前请整体适应一遍。

### 4.5 仍用 HUMAN，但避开 F11 / F12：自定义映射示例

思路：**保留** `g:vimspector_enable_mappings = 'HUMAN'`，再用 **`nmap` 覆盖**或与 `<Plug>` 并列绑定到你顺手的键。Vimspector 未占用 **F7**，可用来做「单步进入 / 单步跳出」等（若终端未拦截）。

在 `~/.config/nvim/init.vim` 中、`vim-plug` 加载 vimspector **之后** 加入类似配置（按喜好修改键位）：

```vim
" 例：用 F7 / Shift+F7 代替 F11 / F12（单步进入 / 单步跳出）
nmap <F7>   <Plug>VimspectorStepInto
nmap <S-F7> <Plug>VimspectorStepOut

" 例：用 leader + 字母，彻底避开顶行功能键（勿用 <leader>do，易与 ShowOutput 冲突）
nmap <leader>di <Plug>VimspectorStepInto
nmap <leader>dO <Plug>VimspectorStepOut

" 仍用 HUMAN 时，若需要「跳到下一/上一断点」，可补上（与 VISUAL_STUDIO 的 F8 行为一致）
nmap <leader>dn <Plug>VimspectorJumpToNextBreakpoint
nmap <leader>dp <Plug>VimspectorJumpToPreviousBreakpoint

" 调用栈上下帧：避免 <LocalLeader>F11/F12 与系统冲突，可改为：
nmap <leader>du <Plug>VimspectorUpFrame
nmap <leader>dD <Plug>VimspectorDownFrame
```

说明：`nmap` 后加载会**覆盖**同键上 HUMAN 的映射；未重绑的键（如 F10、F5）仍保持 HUMAN 默认行为。

### 4.6 你在 `init.vim` 中的额外映射（摘录）

| 映射 | 作用 |
|------|------|
| **\<leader>dd** | `:call vimspector#Launch()`，手动按 `.vimspector.json` 选择配置启动（不经过 `rust-analyzer.debug`） |
| **\<leader>dx** | `:VimspectorReset`，结束调试并清理会话 |
| **\<leader>de** | `:VimspectorEval`，求值表达式 |
| **\<leader>dw** | `:VimspectorWatch`，添加监视 |
| **\<leader>do** | `:VimspectorShowOutput`，查看调试输出 |
| **\<LocalLeader>F11** | 上一调用栈帧（UpFrame） |
| **\<LocalLeader>F12** | 下一调用栈帧（DownFrame） |
| **\<LocalLeader>B** | 断点列表（Breakpoints） |
| **\<LocalLeader>D** | 反汇编窗口（Disassemble） |

若你为 `StepOut` 等操作自定义的键与上表中的 **\<leader>do**、**\<leader>dw** 等重复，请为「调试单步」与「ShowOutput / Watch」择不同前缀或键位。

### 4.7 与 Rust / coc 相关

| 操作 | 方式 |
|------|------|
| 用 rust-analyzer 选择 runnable 并调试 | `:CocCommand rust-analyzer.debug` |
| 仅运行（不挂调试器） | `:CocCommand rust-analyzer.run`（若已映射如 `\<leader>rr` 则可用映射） |
| 重载 coc / 语言服务 | `:CocRestart` 等 |

### 4.8 常用命令行（无快捷键时）

- `:VimspectorReset`：停止调试  
- `:call vimspector#Launch()`：选择配置启动  
- `:CocCommand rust-analyzer.debug`：从 rust-analyzer 列表启动（需 coc 与 `.vimspector.json` 配置正确）

---

## 5. 参考路径小结

| 文件 | 说明 |
|------|------|
| `~/.config/nvim/coc-settings.json` | `rust-analyzer.debug.*` |
| `~/.config/nvim/init.vim` | coc、Vimspector 插件与自定义映射 |
| 本仓库 `.vimspector.json` | `launch`（对接 coc）与其它配置 |

若子工程不在仓库根目录，可能需要在对应目录增加 `.vimspector.json`，或使用全局 `~/.vimspector.json` 统一维护。

---

## 6. FAQ：按 F10 单步到最后为何出现汇编视图？

### 原因

1. **`main` 返回之后**还有 **Rust 运行时 / 标准库收尾**（如 `lang_start` 相关路径、`atexit`、与 libc 的交接等）。这些代码往往**没有对应你工程里的 `.rs` 源码**，或调试信息不完整，调试器只能显示 **反汇编**；Vimspector 会打开类似 `_vimspector_tmp/...` 的缓冲区，并出现 `; Source: unknown`、`; Symbol: no symbol info` 等提示。
2. **`println!` 是宏**，有时单步会进入展开后的标准库实现；若再连续 **StepOver**，有可能逐步走到**无源码映射**的帧，同样会落到汇编视图。
3. 这是 **LLDB + DAP 的正常行为**，不是配置坏了；表示「当前停在没有源映射的机器指令上」。

### 如何避免「为了退出而一路 F10 走进汇编」

| 做法 | 说明 |
|------|------|
| **在业务代码最后一两步改用 F5（Continue）** | 当你已经确认逻辑正确、只剩 `main` 收尾时，用 **继续运行** 让进程**一口气跑到结束**，而不是再用 **F10** 逐行「走出」`main`。这样通常**不会**再手动经过一长串汇编单步。 |
| **直接结束调试** | 若已看到预期输出、无需跟到进程真正退出，可按 **F3（Stop）** 或 `:VimspectorReset`，不必跟完运行时。 |
| **少用 F10 顶穿 `main` 的右花括号** | `main` 执行完后的下一条「用户语义上的下一步」往往已进入运行时；若坚持用 F10，就很容易进入无源码帧 → 汇编视图。 |

**注意**：无法在保证「始终单步、又始终只看 Rust 源码」的前提下走完**整个**进程退出路径——因为退出路径本身就没有你的 `.rs` 源。实际调试里一般以 **Continue** 或 **Stop** 收尾即可。

### 若已误入汇编窗口

- 切回 **调用栈**里带 `main.rs` 的帧（若仍存在），或关闭该临时缓冲，用 **F5** 让程序跑完 / **F3** 停止调试。
- 不要随意按 **\<LocalLeader>D（反汇编）** 除非你想看汇编；正常源码调试不必开它。
