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
