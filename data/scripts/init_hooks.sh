#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cdir=$(realpath "$pwd/../..")
source "$cdir/data/scripts/env.sh"

init_push_hook() {
  log_title "init_push_hook"
  local nfile
  cd "$pwd" || exit
  br=$(git remote | grep local)
  if [ -z "$br" ]; then
    return
  fi
  nfile="post-commit"
  echo "#!/bin/bash

git push local main
" > "$nfile"
  mv -f "$nfile" "$cdir/.git/hooks/$nfile"
  chmod +x "$cdir/.git/hooks/$nfile"
  cd - || exit
}


main() {
  init_push_hook
}

main "$@"
