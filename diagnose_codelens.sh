#!/bin/bash

echo "===== Rust CodeLens 诊断脚本 ====="
echo ""

echo "1. 检查 rust-analyzer 进程状态："
ps aux | grep -i "rust-analyzer" | grep -v grep
echo ""

echo "2. 检查已安装的 Rust 相关扩展："
code --list-extensions | grep -i rust
echo ""

echo "3. 检查 Cursor 用户设置中的 CodeLens 配置："
if [ -f ~/Library/Application\ Support/Cursor/User/settings.json ]; then
    echo "找到 Cursor 用户设置文件"
    cat ~/Library/Application\ Support/Cursor/User/settings.json | grep -i "codelens\|rust-analyzer.lens" || echo "未找到相关配置"
else
    echo "未找到 Cursor 用户设置文件"
fi
echo ""

echo "4. 检查工作区设置："
if [ -f .vscode/settings.json ]; then
    echo "工作区设置内容："
    cat .vscode/settings.json
else
    echo "未找到工作区设置"
fi
echo ""

echo "===== 诊断完成 ====="
