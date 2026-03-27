#!/bin/bash

echo "===== 强制重置 rust-analyzer ====="

# 1. 清除 rust-analyzer 缓存
echo "1. 清除 rust-analyzer 缓存..."
rm -rf ~/.cache/rust-analyzer 2>/dev/null
rm -rf ~/Library/Caches/rust-analyzer 2>/dev/null
echo "   完成"

# 2. 清除项目的 target 目录
echo "2. 清除项目 target 目录..."
cd /Users/liukai/workspaces/rust/my-project/rust-learning
find . -type d -name "target" -exec rm -rf {} + 2>/dev/null
echo "   完成"

# 3. 重新构建项目
echo "3. 重新构建项目..."
cd /Users/liukai/workspaces/rust/my-project/rust-learning/add_test/adder
cargo clean
cargo check
echo "   完成"

echo ""
echo "===== 重置完成 ====="
echo "现在请："
echo "1. 在 Cursor 中按 Cmd+Shift+P"
echo "2. 输入: rust-analyzer: Restart server"
echo "3. 等待 10 秒"
echo "4. 打开 main.rs 文件查看是否有 Run 按钮"
