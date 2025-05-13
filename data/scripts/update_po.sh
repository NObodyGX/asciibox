#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cdir=$(realpath "$pwd/../..")
name=$(grep 'name =' "$cdir/Cargo.toml" | awk -F'"' '{print $2}')
linguas="$cdir/po/LINGUAS"
source "$cdir/data/scripts/env.sh"

update_po() {
  local lang pot po
  lang="$1"
  pot="$cdir/po/$name.pot"
  po="$cdir/po/${lang}.po"

  if [ -f "$po" ]; then
    msgmerge --update "$po" "$pot"
  else
    msginit --input="$pot" --locale="$lang" --output-file="$po" --no-translator
    sed -i 's/Last-Translator:\ Automatically\ generated/Last-Translator:\ NObodyGX<nobodygx@163.com>/' "$po"
    sed -i 's/Language-Team:\ none/Last-Team:\ NObodyGX<nobodygx@163.com>/' "$po"
    sed -i 's/Content-Type:\ text\/plain;\ charset=ASCII/Content-Type:\ text\/plain;\ charset=UTF-8/' "$po"
  fi

  if [ -f "${po}~" ]; then
    rm -f "${po}~"
  fi
  log_succ "update ${lang}"
}

main() {
  lang="$1"
  if [ -n "$lang" ]; then
    update_po "$lang"
    exit 0
  fi
  while read -r lang; do
    if [[ -n $lang && ${lang::1} != '#' ]]; then
      update_po "$lang"
    fi
  done <"$linguas"
}

main "$@"
