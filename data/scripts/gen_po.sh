#! /bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cdir=$(realpath "$pwd/../..")
name=$(grep 'name =' "$cdir/Cargo.toml" | awk -F'"' '{print $2}')
url=$(grep 'homepage =' "$cdir/Cargo.toml" | awk -F'"' '{print $2}')
pifile="$cdir/po/POTFILES.in"
linguas="$cdir/po/LINGUAS"
source "$cdir/data/scripts/env.sh"

gen_potfiles_in() {
  log_title "generate POTFILES.IN"
  cd "$cdir" || exit
  exclude_pattern="target|node_modules"
  echo "# DO NOT EDIT MANUALLY，GENERATE BY gen_potfiles.sh" >"$pifile"
  # 查找所有的 .ui 文件，不包括匹配排除模式的项
  find "data/ui" -name '*.ui' | grep -Ev "$exclude_pattern" >>"$pifile"
  # 查找所有的 .rs 文件，不包括匹配排除模式的项
  find "src" -name '*.rs' | grep -Ev "$exclude_pattern" >>"$pifile"

  log_succ "POTFILES.in has been generated."
  cd - || exit
}

gen_pot() {
  log_title "generate $name.pot"
  local pot="$cdir/po/$name.pot"
  if [ -f "$pot" ]; then
    rm -f "$pot"
  fi
  top_srcdir="${top_srcdir:-.}"
  srcdir="${srcdir:-$top_srcdir/po}"
  XGETTEXT_KEYWORDS="${XGETTEXT_KEYWORDS:- --keyword=_ --keyword=N_ --keyword=C_:1c,2 --keyword=NC_:1c,2 --keyword=g_dngettext:2,3 }"
  IFS=' ' read -r -a XGETTEXT_KEYWORDS_ARRAY <<<"$XGETTEXT_KEYWORDS"
  # 如果不将其作为列表传入，在解析的时候，整个参数将会被视为一个变量传入，导致参数解析出错
  xgettext "${XGETTEXT_KEYWORDS_ARRAY[@]}" \
    --default-domain="$name" \
    --directory="$top_srcdir" \
    --msgid-bugs-address="$url/issues/" \
    --package-name="$name" \
    --add-comments \
    --from-code=utf-8 \
    --flag=g_dngettext:2:pass-c-format \
    --flag=g_strdup_printf:1:c-format \
    --flag=g_string_printf:2:c-format \
    --flag=g_string_append_printf:2:c-format \
    --flag=g_error_new:3:c-format \
    --flag=g_set_error:4:c-format \
    --flag=g_markup_printf_escaped:1:c-format \
    --flag=g_log:3:c-format \
    --flag=g_print:1:c-format \
    --flag=g_printerr:1:c-format \
    --flag=g_printf:1:c-format \
    --flag=g_fprintf:2:c-format \
    --flag=g_sprintf:2:c-format \
    --flag=g_snprintf:3:c-format \
    --flag=g_scanner_error:2:c-format \
    --flag=g_scanner_warn:2:c-format \
    --flag=gtk_message_dialog_format_secondary_markup:2:c-format \
    --flag=gtk_message_dialog_format_secondary_text:2:c-format \
    --flag=gtk_message_dialog_new:5:c-format \
    --flag=gtk_message_dialog_new_with_markup:5:c-format \
    --files-from="$srcdir/POTFILES.in" \
    --output="$cdir/po/$name.pot"
  if [ -f "$cdir/po/$name.pot" ]; then
    log_succ "$name.pot has been generated"
  fi
}

update_po() {
  local lang pot po
  lang="$1"
  pot="$cdir/po/$name.pot"
  po="$cdir/po/${lang}.po"

  if [ -f "$po" ]; then
    msgmerge --quiet --update "$po" "$pot"
  else
    msginit --input="$pot" --locale="$lang" --output-file="$po" --no-translator
    sed -i 's/Last-Translator:\ Automatically\ generated/Last-Translator:\ NObodyGX<nobodygx@163.com>/' "$po"
    sed -i 's/Language-Team:\ none/Last-Team:\ NObodyGX<nobodygx@163.com>/' "$po"
    sed -i 's/Content-Type:\ text\/plain;\ charset=ASCII/Content-Type:\ text\/plain;\ charset=UTF-8/' "$po"
  fi

  if [ -f "${po}~" ]; then
    rm -f "${po}~"
  fi
  log_succ "${lang}.po has been update"
}

update_all_po() {
  log_title "generate all po"
  while read -r lang; do
    if [[ -n $lang && ${lang::1} != '#' ]]; then
      update_po "$lang"
    fi
  done <"$linguas"
}

main() {
  gen_potfiles_in
  gen_pot
  update_all_po

  log_succ "done"
}

main "$@"
