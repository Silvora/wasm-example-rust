#!/bin/bash
set -e

# 清空旧构建
rm -rf www/pkg/*
mkdir -p www/pkg

# 单独构建模块
build_module() {
    local name=$1
    echo "Building $name..."
    cd "crates/$name"
    wasm-pack build --target web --out-dir ../../www/pkg/$name --release
    cd ..
}

# 按需构建
if [ "$1" = "all" ]; then
    build_module "core"
else
    build_module "$1"
fi

echo ""
echo "🎉🎉🎉 [$(date +'%Y-%m-%d %H:%M:%S')] 构建完成！输出目录: www/pkg/"