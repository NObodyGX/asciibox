# Asciibox readme

[简体中文说明](./readme_CN.md)

## description

asciibox is an auxiliary tool intended to simplify ascii text manipulation, include svgbob and asciidoc, the current implementation of the following features:

1. use mermaid syntax to generate svgbob, it will generate svgbob ascii text and output image(by svgbob)
2. align and beautify asciidoc tables.

## how to run

run `sh build.sh` in project root dir

> if you want to debug with cargo, you should replace gresource with your build, run `ln -s $PKGDATA_DIR/asciibox.gresource $PROJECTDIR/_build/data/asciibox.gresource`, `PKGDATA_DIR` define in `config.rs`, `PROJECTDIR` is project folder locate

## roadmap

- [ ] svgbob
    - [x] zh-cn support
    - [x] basic arrow(left/down/up/right) support
    - [ ] subgraph support
    - [ ] multi arrow support
    - [x] preview
- [ ] asciidoc
    - [ ] beautify table
    - [ ] beautify code
    - [ ] transform from markdown
    - [ ] preview

## install

### linux

```shell
# for arch linux
paru -S asciibox

# for other linux
git clone --depth=1 https://github.com/nobodygx/asciibox
meson setup build
meson compile -C build
meson install -C build

# run asciibox to enjoy!!
```

### windows

```shell

# Install MSYS2
pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc mingw-w64-x86_64-libadwaita

# add into paths
C:\msys64\mingw64\include
C:\msys64\mingw64\bin
C:\msys64\mingw64\lib

# install rust
rustup toolchain install stable-gnu
rustup default stable-gnu

# before cargo run
# sh build.sh  # --> to build asciibox, but without install
# cp data/com.github.nobodygx.asciibox.gschema.xml /usr/share/glib-2.0/schemas/
# cp _build/data/asciibox.gresource /mingw/share/asciibox/
# cargo run to enjoy
```


