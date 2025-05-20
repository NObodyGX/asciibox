#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cdir=$(realpath "$pwd/../..")
tfile="$cdir/data/html/mermaid.min.js"
source "$cdir/data/scripts/env.sh"

ver=$(grep -o 'name:"mermaid",version:"[^"]*' "$tfile" | cut -d '"' -f4)

echo "$ver"
