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
bdir="_build"

# todo, add into meson.build
# only for test, need sudo
# cp $pwd/data/asciibox.gschema.xml /usr/share/glib-2.0/schemas/
# glib-compile-schemas /usr/share/glib-2.0/schemas/

function sync_version {
  local ver=$(sed -n '2p' meson.build | grep -Eo "[0-9]+\.[0-9]+\.+[0-9]")
  local ver2=$(sed -n '3p' Cargo.toml | grep -Eo "[0-9]+\.[0-9]+\.+[0-9]")
  if [ $ver != $ver2 ]; then
    echo "change Cargo: $ver2 --> $ver"
    sed -i "3s/${ver2}/${ver}/g" Cargo.toml
  fi
}

sync_version

function del_target {
  local target="$pwd/$bdir/src/asciibox"
  if [ -f $target ];then
    rm -f $target
  fi
}


cd $pwd
if [ $is_clean -eq 1 ]; then
  meson setup $bdir --reconfigure
elif [ $is_clean -eq 2 ]; then
  rm -rf $bdir
  meson setup $bdir
else
  meson setup $bdir
fi
cd $pwd/$bdir
meson compile

function run_target {
  local target="$pwd/$bdir/src/asciibox"
  if [ -f $target ];then
    cd $pwd/$bdir/src
    ./asciibox
    cd -
  else
    echo "[error]: build failed."
  fi

}
run_target

cd -
cd -