use std::cmp;

use crate::core::utils;

#[derive(Debug)]
pub struct TableData {
    pub title: String,
    pub w: usize,
    pub h: usize,
    data: Vec<Vec<String>>,
}

impl TableData {
    pub fn new(w: usize, h: usize) -> Self {
        let data = vec![vec!["".to_string(); w]; h];
        Self {
            title: "".to_string(),
            w,
            h,
            data,
        }
    }

    /// 给 x 列 y 行设置值，当越界时不会赋值
    pub fn set_cell(&mut self, x: usize, y: usize, v: &str) {
        if y >= self.h {
            return;
        }
        if x >= self.w {
            return;
        }
        self.data[y][x] = v.trim().to_string().clone();
    }

    /// 计算指定列的最大宽度，当越界时返回 0
    fn width(&self, x: usize, is_markdown: bool) -> usize {
        let mut v: usize = 0;
        if x > self.w {
            return v;
        }
        for (i, line) in self.data.iter().enumerate() {
            if is_markdown && i == 1 {
                let mut txt = String::new();
                for x in line.iter() {
                    txt.push_str(x.trim());
                    txt.push('\n');
                }
                if txt.contains("-") {
                    continue;
                }
            }
            v = std::cmp::max(v, utils::cn_length(&line[x]));
        }
        return v;
    }

    fn max_line_width(&self) -> usize {
        let mut w: usize = 5;

        for line in self.data.iter() {
            let mut cur_width: usize = 0;
            for cell in line.iter() {
                cur_width += 3 + utils::cn_length(cell);
            }
            w = cmp::max(w, cur_width);
        }
        return w;
    }

    pub fn to_markdown_table(&self) -> String {
        let mut cell_widths: Vec<usize> = Vec::new();
        for x in 0..self.w {
            // markdown 对齐需要三格
            cell_widths.push(std::cmp::max(self.width(x, true), 3));
        }

        let mut content: Vec<String> = Vec::new();
        // 正文
        for line in self.data.iter() {
            let mut xline = String::new();
            for (j, cell) in line.iter().enumerate() {
                let (v1, v2) = (cell_widths[j], utils::cn_length(cell));
                let blank = " ".repeat(cmp::max(v1, v2) - cmp::min(v1, v2));
                xline.push_str("| ");
                xline.push_str(cell);
                xline.push_str(blank.as_str());
                xline.push(' ');
            }
            xline.push('|');
            content.push(xline.clone());
        }
        // 添加表格对齐
        // 检测是否需要添加
        let line: &String = content.get(1).unwrap();
        if !line.contains("-") {
            let mut nline = String::new();
            for w in cell_widths.iter() {
                nline.push_str("| ");
                nline.push_str("-".repeat(*w).as_str());
                nline.push(' ');
            }
            nline.push('|');
            content.insert(1, nline);
        } else {
            let mut nline = String::new();
            let cells = self.data.get(1).unwrap();
            for (w, cell) in cells.iter().enumerate() {
                nline.push_str("| ");
                if cell.starts_with(":-") && cell.ends_with("-:") {
                    nline.push(':');
                    nline.push_str("-".repeat(cell_widths[w] - 2).as_str());
                    nline.push(':');
                } else if cell.starts_with(":-") {
                    nline.push(':');
                    nline.push_str("-".repeat(cell_widths[w] - 1).as_str());
                } else if cell.starts_with(":-") {
                    nline.push_str("-".repeat(cell_widths[w] - 1).as_str());
                    nline.push(':');
                } else {
                    nline.push_str("-".repeat(cell_widths[w]).as_str());
                }
                nline.push(' ');
            }
            nline.push('|');
            content.insert(1, nline);
            content.remove(2);
        }

        let mut result = String::new();
        for x in content.iter() {
            result.push_str(x.trim());
            result.push('\n');
        }
        result
    }

    pub fn to_asciidoc_table(&self) -> String {
        let mut cell_widths: Vec<usize> = Vec::new();
        for x in 0..self.w {
            cell_widths.push(self.width(x, false));
        }

        let mut content: Vec<String> = Vec::new();
        // 第 i 行
        for line in self.data.iter() {
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
        let w = self.max_line_width();
        let mut border = String::new();
        border.push('|');
        border.push_str("=".repeat(w).as_str());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_new() {
        let mut data = TableData::new(5, 3);
        data.set_cell(0, 0, "1");
        data.set_cell(1, 0, "2");
        data.set_cell(2, 0, "3");
        data.set_cell(3, 0, "4");
        data.set_cell(4, 0, "5");
        println!("{:#?}", data)
    }

    #[test]
    fn test_str() {
        let a = "aa|a\tbb|b";
        let b = a.matches("|").count();
        println!("len: {b}");
    }
}
