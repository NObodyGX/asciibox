use super::TableData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableMode {
    Asciidoc,
    Markdown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MarkdownStyle {
    Normal,
    Github,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OriginTableMode {
    Asciidoc,
    Markdown,
    NoneByTab,
    NoneBySpace,
    None,
}

#[derive(Debug)]
pub struct TableFormator {
    pub cell_max_w: usize,
    pub line_max_w: usize,
}

impl TableFormator {
    pub fn new(w: usize, max_w: usize) -> Self {
        Self {
            cell_max_w: w,
            line_max_w: max_w,
        }
    }

    // 判断是否包含，如果超过 60% 的行包含，那么证明包含
    fn check_contain_symbol(&self, text: &str, symbol: &str) -> bool {
        let mut count = 0;
        let lines: Vec<&str> = text.split('\n').filter(|&s| !s.is_empty()).collect();
        let total = lines.len() * 6 / 10;
        for x in lines.iter() {
            let x = x.trim();
            if x.contains(symbol) {
                count += 1;
            }
            if count >= total {
                return true;
            }
        }
        return false;
    }

    fn check_origin_table_mode(&self, text: &str) -> OriginTableMode {
        // 不包含 | 则证明是非markdown或者asciidoc表格
        if !text.contains("|") {
            if self.check_contain_symbol(text, "\t") {
                return OriginTableMode::NoneByTab;
            }
            if self.check_contain_symbol(text, " ") {
                return OriginTableMode::NoneBySpace;
            }
            return OriginTableMode::None;
        }
        // 如果包含 markdown 表格对齐标识
        if text.contains("---")
            || text.contains(":--")
            || text.contains("--:")
            || text.contains(":-:")
        {
            return OriginTableMode::Markdown;
        }
        // 如果包含 asciidoc 表格标识
        if text.contains("|==") {
            return OriginTableMode::Asciidoc;
        }
        let lines: Vec<&str> = text.split('\n').filter(|&s| !s.is_empty()).collect();
        let line = lines[0].trim();
        // 如果刚开始是 asciidoc 的标题的话
        if line.starts_with(".") {
            return OriginTableMode::Asciidoc;
        }
        if line.starts_with("|") {
            // 原始风格的 markdown
            if line.ends_with("|") {
                return OriginTableMode::Markdown;
            } else {
                return OriginTableMode::Asciidoc;
            }
        }
        // 如果每层都有 | 的话，则证明是 markdown
        let mut markdown_flag = true;
        for x in lines.iter() {
            if !x.contains("|") {
                markdown_flag = false;
            }
        }
        if markdown_flag {
            return OriginTableMode::Markdown;
        }
        return OriginTableMode::None;
    }

    fn get_table_width(&self, lines: &Vec<&str>, omode: &OriginTableMode) -> usize {
        match omode {
            OriginTableMode::Markdown => {
                let line = lines[0].trim();
                if line.starts_with("|") {
                    return line.matches("|").count() - 1;
                }
                return line.matches("|").count() + 1;
            }
            OriginTableMode::Asciidoc => {
                for line in lines.iter() {
                    if line.starts_with(".") {
                        continue;
                    }
                    if line.starts_with("|==") {
                        continue;
                    }
                    return line.matches("|").count();
                }
                return 0;
            }
            OriginTableMode::NoneByTab => {
                let line = lines[0].trim();
                return line.matches("\t").count() + 1;
            }
            OriginTableMode::NoneBySpace => {
                let line = lines[0].trim();
                return line.matches(" ").count() + 1;
            }
            OriginTableMode::None => {
                let line = lines[0].trim();
                return line.matches(" ").count() + 1;
            }
        }
    }

    fn try_format_into_basic_table(&self, text: &str) -> Option<TableData> {
        let lines: Vec<&str> = text.split('\n').filter(|&s| !s.is_empty()).collect();
        let h = lines.len();
        if h < 2 {
            return None;
        }
        let omode = self.check_origin_table_mode(text);
        let w = self.get_table_width(&lines, &omode);
        let mut data = TableData::new(w, h, self.cell_max_w, self.line_max_w);
        match omode {
            OriginTableMode::Markdown => {
                for (i, line) in lines.iter().enumerate() {
                    // 正常风格
                    if line.starts_with("|") {
                        for (j, cell) in line.split("|").enumerate() {
                            if j == 0 {
                                continue;
                            }
                            data.set_cell(j - 1, i, cell);
                        }
                        continue;
                    }
                    // github 风格
                    for (j, cell) in line.split("|").enumerate() {
                        data.set_cell(j, i, cell);
                    }
                }
            }
            OriginTableMode::Asciidoc => {
                for (i, line) in lines.iter().enumerate() {
                    if i == 0 && line.starts_with(".") {
                        data.title = line.to_string().clone();
                    }
                    // 忽略掉表格的部分
                    if line.starts_with("|==") {
                        continue;
                    }
                    // TODO：这里需要asciidoc的语法做一点特殊分析
                    // 例如表格扩展之类的
                    for (j, cell) in line.split("|").enumerate() {
                        if j == 0 {
                            continue;
                        }
                        data.set_cell(j - 1, i, cell);
                    }
                }
            }
            OriginTableMode::NoneByTab => {
                for (i, line) in lines.iter().enumerate() {
                    for (j, cell) in line.split("\t").enumerate() {
                        data.set_cell(j, i, cell);
                    }
                }
            }
            OriginTableMode::NoneBySpace => {
                for (i, line) in lines.iter().enumerate() {
                    for (j, cell) in line.split(" ").enumerate() {
                        data.set_cell(j, i, cell);
                    }
                }
            }
            OriginTableMode::None => {
                // 先依照 space, 再用强行补 0 的方式
                for (i, line) in lines.iter().enumerate() {
                    for (j, cell) in line.split(" ").enumerate() {
                        data.set_cell(j, i, cell);
                    }
                }
            }
        }
        data.trim();
        return Some(data);
    }

    pub fn do_format(&mut self, text: &str, mode: &TableMode, style: MarkdownStyle) -> String {
        let data = self.try_format_into_basic_table(text);
        match data {
            Some(v) => match mode {
                TableMode::Markdown => {
                    return v.to_markdown_table(style);
                }
                TableMode::Asciidoc => {
                    return v.to_asciidoc_table();
                }
            },
            None => {
                return "error".to_string();
            }
        }
    }
}
