= Asciibox说明 =

== 简介 ==

asciibox 是一个意向简易化 ascii 文本操作的辅助工具，目前实现的功能有：

1. 使用 mermaid 语法完成 svgbob 的生成，可以直接生成 svgbob 的文本和图片
2. 对 asciidoc 的表格进行对齐美化

== 运行 ==

使用 `sh build.sh` 来生成并运行，

如果使用 cargo 进行调试运行，需要在 `config.rs` 里的 `PKGDATADIR` 下建立一个到编译文件夹里 `asciibox.gresource` 的链接，即 `ln -s $PKGDATADIR/asciibox.gresource $PROJECTDIR/_build/data/asciibox.gresource`，其中 PKGDATADIR 替换为指定 config.rs 里的路径名称，PROJECTDIR 替换为当前项目所在文件夹

== 路线图 ==

- [ ] svgbob 支持
    - [ ] 中文支持
    - [ ] 上下左右箭头支持
    - [ ] subgraph 支持
    - [ ] 左上下右上下扩展支持
- [ ] asciidoc 支持
    - [ ] 表格美化
    - [ ] 源码美化
    - [ ] md 转 asciidoc
