#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cdir=$(realpath "$pwd/../..")
potfile="$cdir/po/POTFILES.in"
source "$cdir/data/scripts/env.sh"

# check all potfiles is exist
check_potfiles() {
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
