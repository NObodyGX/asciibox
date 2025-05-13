#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cdir=$(realpath "$pwd/../..")
pfile="$cdir/po/POTFILES.in"

# 定义要排除的目录或文件模式
exclude_pattern="target|node_modules"

cd "$cdir" || exit
echo "# DO NOT EDIT MANUALLY，GENERATE BY gen_potfiles.sh" > "$pfile"
# 查找所有的 .ui 文件，不包括匹配排除模式的项
find "data/ui" -name '*.ui' | grep -Ev "$exclude_pattern" >> "$pfile"
# 查找所有的 .rs 文件，不包括匹配排除模式的项
find "src" -name '*.rs' | grep -Ev "$exclude_pattern" >> "$pfile"

echo "POTFILES.in has been generated."
cd - || exit
