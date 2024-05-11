#!/bin/bash

is_clean=0
while getopts "f" opt_sg; do
  case $opt_sg in
    f) is_clean=1 ;;
    ?) echo "unknown option: $opt_sg" ;;
  esac
done

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)

cd $pwd
if [ $is_clean -eq 1 ]; then
  rm -rf _build
fi
meson _build
cd $pwd/_build
ninja

cd $pwd/_build/src
./asciibox
cd -
cd -
cd -