use super::TableData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableMode {
    Markdown,
    Asciidoc,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OriginTableMode {
    Markdown,
    Asciidoc,
    NoneByTab,
    NoneBySpace,
    None,
}

#[derive(Debug)]
pub struct TableFormator {
    pub w: usize,
    pub max_w: usize,
}

impl TableFormator {
    pub fn new(w: usize, max_w: usize) -> Self {
        Self { w, max_w }
    }

    fn check_origin_table_mode(&self, text: &str) -> OriginTableMode {
        // 不包含 | 则证明是非markdown或者asciidoc表格
        if !text.contains("|") {
            // 如果
            let mut space_flag = true;
            let mut tab_flag = true;
            let lines: Vec<&str> = text.split('\n').filter(|&s| !s.is_empty()).collect();
            for x in lines.iter() {
                let x = x.trim();
                if x.contains(" ") {
                    space_flag = false;
                }
                if !x.contains("\t") {
                    tab_flag = false;
                }
                if !space_flag && !tab_flag {
                    break;
                }
            }
            if tab_flag {
                return OriginTableMode::NoneByTab;
            }
            if space_flag {
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
                let mut line = lines[0].trim();
                if line.starts_with(".") {
                    line = lines[1].trim();
                }
                return line.matches("|").count();
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

    fn try_format_into_basic_table(&self, input: &str) -> Option<TableData> {
        let lines: Vec<&str> = input.split('\n').filter(|&s| !s.is_empty()).collect();
        let h = lines.len();
        if h < 2 {
            return None;
        }
        let omode = self.check_origin_table_mode(input);
        let w = self.get_table_width(&lines, &omode);
        let mut data = TableData::new(w, h);
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
                    if line.starts_with("|=") {
                        continue;
                    }
                    // TODO：这里需要asciidoc的语法做一点特殊分析
                    // 例如表格扩展之类的
                    for (j, cell) in line.split("|").enumerate() {
                        data.set_cell(j, i, cell);
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
        return Some(data);
    }

    pub fn do_format(&mut self, input: &str, mode: TableMode) -> String {
        let data = self.try_format_into_basic_table(input);
        match data {
            Some(v) => match mode {
                TableMode::Markdown => {
                    return v.to_markdown_table();
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
