#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
tdir="${pwd}/test_pkg_build"
idir="${pwd}/../aur"
pkg="PKGBUILD"

if [ ! -d "${tdir}" ]; then
  mkdir -p "${tdir}"
  cd "${tdir}" || exit
  git clone ssh://aur@aur.archlinux.org/asciibox.git asciidoc_aur
  cd - || exit
fi

cp "${idir}/${pkg}" "${tdir}/${pkg}"

cd "${tdir}" || exit

updpkgsums

makepkg --printsrcinfo >.SRCINFO

makepkg -f

cp "${tdir}/${pkg}" "${idir}/${pkg}"

cd - || exit
