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
  nfile="pre-push"
  echo "#!/bin/bash

git push local main
" > "$nfile"
  mv -f "$nfile" "$pwd/.git/hooks/$nfile"
  chmod +x "$pwd/.git/hooks/$nfile"
  cd - || exit
}


main() {
  init_push_hook
}

main "$@"
