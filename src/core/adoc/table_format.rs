use std::cmp;

use crate::core::utils::{self, cn_length};

#[derive(Debug)]
pub struct TableFormator {
    pub title: String,
    pub w: usize,
    pub max_w: usize,
    start_count: usize, // 有效表格头
    end_count: usize,   // 有效表格尾
}

impl TableFormator {
    pub fn new(w: usize, max_w: usize) -> Self {
        Self {
            title: "".to_string(),
            w,
            max_w,
            start_count: 0,
            end_count: 0,
        }
    }

    pub fn do_format(&mut self, input: &str) -> String {
        if !self.check_content(input) {
            return "".to_string();
        }
        let lines: Vec<&str> = input.split('\n').filter(|&s| !s.is_empty()).collect();
        let adata = self.prepare_content(lines);
        let result = self.format_content(adata);
        return result;
    }

    // 检查内容是否包含表格
    fn check_content(&self, input: &str) -> bool {
        let lines: Vec<&str> = input.split('\n').filter(|&s| !s.is_empty()).collect();
        if lines.len() < 2 {
            return false;
        }
        for line in lines.iter() {
            // asciidoc or markdown table
            if line.starts_with("|") {
                return true;
            }
            // github style markdown table
            if line.contains(" | ") {
                return true;
            }
        }
        return true;
    }

    // 准备用于转换的表格内容，直接过滤不要的内容
    // todo：直接覆盖原有内容
    fn prepare_content(&mut self, lines: Vec<&str>) -> Vec<Vec<String>> {
        let mut data: Vec<Vec<String>> = Vec::new();
        let mut idx: usize = lines.len();
        // 包含标题
        if lines[0].starts_with(".") {
            self.title = lines[0].to_string();
            idx = 0;
            self.start_count = 1;
        }

        for (i, line) in lines.iter().enumerate() {
            if i == idx {
                continue;
            }
            let nline: String = line.trim().to_string();
            if nline.len() == 0 {
                continue;
            }
            // 如果是 asciidoc 列表
            if nline.starts_with("* ") {
                continue;
            }
            // 如果是 md 列表
            if nline.starts_with("- ") {
                continue;
            }
            // 如果不含表格内容
            if !nline.contains("|") {
                continue;
            }
            self.end_count = i + 1;
            // 如果是 asciidoc 或者 md 表格
            let mut skip_header = false;
            if nline.starts_with("|") {
                skip_header = true;
            }
            let mut aline: Vec<String> = Vec::new();
            let aaa: Vec<_> = nline.split('|').collect();
            for (i, word) in aaa.iter().enumerate() {
                if skip_header && i == 0 {
                    continue;
                }
                let b = word.trim().to_string();
                aline.push(b.clone());
            }
            data.push(aline);
            continue;
        }
        data
    }

    fn format_content(&self, data: Vec<Vec<String>>) -> String {
        // 计算每列的最大宽度，保证相同
        let mut cell_widths: Vec<usize> = Vec::new();
        for line in data.iter() {
            for (i, cell) in line.iter().enumerate() {
                while i >= cell_widths.len() {
                    cell_widths.push(0);
                }
                // 超出 w 的空格不做处理
                if cn_length(cell) > self.w {
                    continue;
                }
                cell_widths[i] = cmp::max(cell_widths[i], cn_length(cell));
            }
        }
        // 生成表格内容
        let mut content: Vec<String> = Vec::new();
        for line in data.iter() {
            let mut xline = String::new();
            for (j, cell) in line.iter().enumerate() {
                let symbol = if j == 0 { "| " } else { " | " };
                let (v1, v2) = (cell_widths[j], utils::cn_length(cell));
                let blank = " ".repeat(cmp::max(v1, v2) - cmp::min(v1, v2));
                xline.push_str(symbol);
                xline.push_str(cell);
                xline.push_str(blank.as_str());
            }
            content.push(xline.clone());
        }
        // 添加表头和表尾
        let mut total_w = cell_widths.len() * 3 - 2;
        for length in cell_widths.iter() {
            total_w += length;
        }
        let mut border = String::new();
        border.push('|');
        border.push_str("=".repeat(std::cmp::min(self.max_w, total_w)).as_str());
        content.insert(0, border.clone());
        content.push(border);

        // 添加标题
        if self.title.len() > 0 {
            content.insert(0, self.title.clone() + "\n");
        }

        let mut result = String::new();
        for x in content.iter() {
            result.push_str(x.trim());
            result.push('\n');
        }
        result
    }
}
