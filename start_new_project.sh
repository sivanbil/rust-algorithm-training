#!/bin/bash

# 检查必需的参数是否提供
if [ $# -lt 2 ]; then
  echo "Usage: $0 <project_name> <description> [date]"
  exit 1
fi

# 获取参数
project_name=$1
description=$2

# 如果提供了日期参数，则使用指定的日期；否则，使用当前日期
if [ $# -eq 3 ]; then
  date=$3
else
  date=$(date +"%Y-%m-%d")
fi

# 使用 cargo 创建新项目
cargo new $project_name

# 进入项目目录
cd $project_name

# 创建 title.md 文件并写入内容
echo "$date $description" > title.md

# 创建solution.rs
echo "pub struct Solution;" > ./src/solution.rs

echo "Project $project_name created successfully with title.md"