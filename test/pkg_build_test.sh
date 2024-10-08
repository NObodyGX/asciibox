#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
tdir="${pwd}/pkg_build"

if [ ! -d "${tdir}" ]; then
  mkdir -p "${tdir}"
  cd "${tdir}" || exit
  git clone ssh://aur@aur.archlinux.org/asciibox.git asciidoc_aur
  cd - || exit
fi

cp "${pwd}"/../PKGBUILD "${tdir}"/PKGBUILD

cd "${tdir}" || exit

updpkgsums

makepkg --printsrcinfo >.SRCINFO

makepkg -f

cp "${tdir}"/PKGBUILD "${pwd}"/../PKGBUILD

cd - || exit
