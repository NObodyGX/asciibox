#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cdir=$(realpath "$pwd/../..")
name=$(grep 'name =' "$cdir/Cargo.toml" | awk -F'"' '{print $2}')
odir=$(grep 'LOCALE_DIR' "$cdir/src/config.rs" | awk -F '"' '{print $2}')
linguas="$cdir/po/LINGUAS"
source "$cdir/data/scripts/env.sh"

# check all potfiles is exist
gen_mo() {
  while read -r lang; do
    if [[ -n $lang && ${lang::1} != '#' ]]; then
      if [[ -f "$cdir/po/${lang}.po" ]]; then
        log_info "start msgfmt $lang"
        msgfmt -o "${name}.mo" "$cdir/po/${lang}.po"
        sudo_run "mv ${name}.mo $odir/$lang/LC_MESSAGES/${name}.mo"
      fi
    fi
  done <"$linguas"
  log_succ "gen mo PASSED"
}

main() {
  gen_mo
}

main "$@"
