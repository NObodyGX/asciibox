#! /bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cdir=$(realpath "$pwd/../..")
name=$(grep 'name =' "$cdir/Cargo.toml" | awk -F'"' '{print $2}')
url=$(grep 'homepage =' "$cdir/Cargo.toml" | awk -F'"' '{print $2}')

gen_pot() {
  local pot="$cdir/po/$name.pot"
  if [ -f "$pot" ]; then
    rm -f "$pot"
  fi
  top_srcdir="${top_srcdir:-.}"
  srcdir="${srcdir:-$top_srcdir/po}"
  XGETTEXT_KEYWORDS="${XGETTEXT_KEYWORDS:- --keyword=_ --keyword=N_ --keyword=C_:1c,2 --keyword=NC_:1c,2 --keyword=g_dngettext:2,3 }"
  xgettext --default-domain="$name" \
          --directory="$top_srcdir" \
          --msgid-bugs-address="$url/issues/" \
          --package-name="$name" \
          --add-comments ${XGETTEXT_KEYWORDS}\
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
}

main() {
  echo "---------------gen pot"
  gen_pot
}

main "$@"
