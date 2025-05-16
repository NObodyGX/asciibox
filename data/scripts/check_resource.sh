#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cdir=$(realpath "$pwd/../..")
potfile="$cdir/po/POTFILES.in"
source "$cdir/data/scripts/env.sh"

# 检查所有在 POTFILES.in 里定义的文件是否存在
check_potfiles() {
    log_title "check potfiles"
    while read -r line; do
        if [[ -n $line && ${line::1} != '#' ]]; then
            if [[ ! -f $line ]]; then
                log_error "$line in ${potfile##*/} does not exists"
            fi
        fi
    done < "$potfile"
    log_succ "check ${potfile##*/} PASSED"
}

main() {
    check_potfiles
}

main "$@"
