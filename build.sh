#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)

cd $pwd
meson _build
cd $pwd/_build
ninja

cd $pwd/_build/src
./asciibox
cd -
cd -
cd -