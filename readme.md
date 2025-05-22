# Asciibox说明 #

[English Readme](./readme_EN.md)

## 简介 ##

asciibox 是一个意向简易化 ascii 文本操作的辅助工具，实现的功能有：

1. 使用 mermaid 语法完成 svgbob 的生成，可以直接生成 svgbob 的文本和图片
2. 对 asciidoc/markdown 的表格进行对齐美化
3. 支持 mermaid 的预览和导出


## 运行 ##

使用 `sh build.sh` 来生成并运行

> 注意⚠️：如果使用 cargo 进行调试运行，其内部也会调用 build.sh，请确保 `build.sh` 不被删除

## 路线图 ##

- [ ] 流程图(svgbob)
    - [x] svgbob 预览
    - [x] 中文支持
    - [x] 上下左右箭头支持
    - [ ] 左上下右上下扩展支持
    - [ ] subgraph 支持
- [ ] 表格
    - [x] asciidoc表格
    - [x] markdown表格
    - [x] markdown-gfm表格
    - [ ] 表格美化
    - [ ] 表格调整
- [ ] mermaid 支持
    - [x] mermaid 预览
    - [x] mermaid 转 svg
    - [x] mermaid 转 png
    - [x] mermaid 主题
    - [ ] mermaid 自定义主题
    - [ ] mermaid 配置

## 感谢

- 本工具使用了 [remix](https://remixicon.com/) 图标
