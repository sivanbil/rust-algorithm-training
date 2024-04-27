#!/bin/bash

# 设置 Markdown 文件名
output_file="README.md"

# 删除已有的 Markdown 文件（如果存在）
if [ -f "$output_file" ]; then
  rm "$output_file"
fi

# 存储目录信息的临时文件
temp_file="temp_dirs.txt"
# 遍历当前目录及其子目录，排除以点号（.）开头的隐藏目录
process_directory() {
  local base_dir="$1"
  local indent="$2"

  for entry in "$base_dir"/*; do
    if [ -d "$entry" ]; then
      # 如果是目录且不是隐藏目录
      if [[ "$(basename "$entry")" != .* ]]; then
        local dir_name=$(basename "$entry")

        local title_file="$entry/title.md"

        # 检查目录下是否存在 title.md 文件
        if [ -f "$title_file" ]; then
          # 读取 title.md 文件的内容作为链接文本
          local link_text=$(cat "$title_file")

          # 提取标题中的日期
          local date=$(echo "$link_text" | grep -oE "^[0-9]{4}-[0-9]{2}-[0-9]{2}")

          # 将目录信息写入临时文件，格式为：日期 目录名 链接文本
          echo "$date|$dir_name|$link_text" >> "$temp_file"
        fi
      fi
    fi
  done
}

process_directory "." ""

# 根据日期对目录进行排序
sorted_dirs=$(sort "$temp_file")

# 将排序后的目录信息写入 Markdown 文件
echo "# 每日一题【from: 2024-04-23】" >> "$output_file"
while IFS='|' read -r date directory title; do
  markdown_links+="- [$title](./$directory)\n"
done <<< "$sorted_dirs"

# 输出结果
echo -e "$markdown_links" >> $output_file


## 删除临时文件
rm "$temp_file"

echo "Markdown file generated: $output_file"