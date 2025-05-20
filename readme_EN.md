# Asciibox readme

## description

asciibox is an auxiliary tool intended to simplify ascii text manipulation, include mermaid, svgbob and asciidoc, the current implementation of the following features:

1. use mermaid syntax to generate svgbob, it will generate svgbob ascii text and output image(by svgbob)
2. align and beautify asciidoc tables.
3. mermaid preiew and output svg

## how to run

run `sh build.sh` in project root dir

> notice⚠️: if using cargo debug/run, makesure `build.rs` is exist.

## roadmap

- [ ] flowchart(svgbob impl)
    - [x] preview
    - [x] zh-cn support
    - [x] basic arrow(left/down/up/right) support
    - [ ] multi arrow support
    - [ ] subgraph support
- [ ] table
    - [x] asciidoc table
    - [x] markdown table
    - [x] markdown-gfm table
    - [ ] beautify table
    - [ ] op table
- [ ] mermaid
    - [x] mermaid preview
    - [x] mermaid to svg
    - [ ] mermaid to png
    - [ ] mermaid theme
    - [ ] mermaid config

## Thanks

- thanks to [remix](https://remixicon.com/) for icon
