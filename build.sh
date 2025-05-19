#!/bin/bash

is_clean=0
while getopts "rf" opt_sg; do
  case $opt_sg in
  f) is_clean=2 ;;
  r) is_clean=1 ;;
  ?) echo "unknown option: $opt_sg" ;;
  esac
done

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
bdir="${pwd}/target/build"
name=$(grep 'name =' "$pwd/Cargo.toml" | awk -F'"' '{print $2}')

function check_dirs() {
  if [ ! -d "$bdir" ]; then
    mkdir -p "$bdir"
  fi
}

function sudo_run() {
  sudo -u root -H sh -c "$1"
}

function sync_version() {
  local ver_o="" ver_n=""
  ver_o=$(sed -n '2p' "$pwd/meson.build" | grep -Eo "[0-9]+\.[0-9]+\.+[0-9]")
  ver_n=$(sed -n '3p' "$pwd/Cargo.toml" | grep -Eo "[0-9]+\.[0-9]+\.+[0-9]")
  if [ "$ver_o" != "$ver_n" ]; then
    echo "change Cargo: $ver_n --> $ver_o"
    sed -i "3s/${ver_n}/${ver_o}/g" Cargo.toml
  fi
}

function prepare_ui() {
  cd "${pwd}/data/ui" || exit

  for file in *.blp; do
    if [ -f "$file" ]; then
      nfile="${file%.blp}.ui"
      echo "$file --> $nfile"
      blueprint-compiler compile --output "$nfile" "$file"
    fi
  done
  cd - || exit
}

function build_resource() {
  cd "${pwd}/data" || exit
  glib-compile-resources "${name}.gresource.xml"
  if [ ! -d "${pwd}/data/bin" ]; then
    mkdir -p "${pwd}/data/bin"
  fi
  mv "${name}.gresource" "${pwd}/data/bin/${name}.gresource"
  cd - || exit
}

function rm_target() {
  local target="$bdir/src/$name"
  if [ -f "$target" ]; then
    rm -f "$target"
  fi
}

function build_target() {
  cd "$pwd" || exit
  if [ $is_clean -eq 1 ]; then
    meson setup "$bdir" --reconfigure
  elif [ $is_clean -eq 2 ]; then
    rm -rf "$bdir"
    meson setup "$bdir"
  else
    if [[ -d "$bdir" && $(git status --porcelain | grep -E '(blp|svg|xml|in|po)') -ge 1 ]]; then
      meson setup "$bdir" --reconfigure
    else
      meson setup "$bdir"
    fi
  fi
  cd "${bdir}" || exit
  meson compile
  cd - || exit
  cd - || exit
}

function run_target() {
  local target="$bdir/src/$name"
  if [ -f "$target" ]; then
    cd "$bdir/src" || exit
    ./"$name"
    cd - || exit
  else
    echo "[error]: build failed."
  fi
}

function main() {
  local mode="$1"
  local force="$2"

  if [[ "$force" == "-f" ]]; then
    rm -rf "$bdir"
  fi

  if [[ "$mode" == "prepare" ]]; then
    echo "------------------------start prepare ${mode} ${force}"
    check_dirs
    sync_version
    prepare_ui
    build_resource
  elif [[ "$mode" == "build" ]]; then
    echo "------------------------start build"
    check_dirs
    sync_version
    prepare_ui
    build_resource
    build_target
  else
    echo "------------------------start run"
    check_dirs
    sync_version
    prepare_ui
    build_resource
    build_target
    run_target
  fi
}

main "$@"
