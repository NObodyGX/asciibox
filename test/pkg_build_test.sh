#!/bin/bash

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
tdir="${pwd}/pkg_build"

if [ ! -d ${tdir} ];then
  mkdir -p ${tdir}
fi

cp ${pwd}/../PKGBUILD ${tdir}/PKGBUILD

cd ${tdir}

updpkgsums

makepkg --printsrcinfo > .SRCINFO

makepkg -f

cp ${tdir}/PKGBUILD ${pwd}/../PKGBUILD

cd -



