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

# todo, add into meson.build
# only for test, need sudo
# cp $pwd/data/com.github.nobodygx.asciibox.gschema.xml /usr/share/glib-2.0/schemas/
# glib-compile-schemas /usr/share/glib-2.0/schemas/


cd $pwd
if [ $is_clean -eq 1 ]; then
  meson setup _build --reconfigure
elif [ $is_clean -eq 2 ]; then
  rm -rf _build
  meson setup _build
else
  meson setup _build
fi
cd $pwd/_build
meson compile

cd $pwd/_build/src
./asciibox
cd -
cd -
cd -