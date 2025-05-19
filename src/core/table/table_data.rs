use std::cmp;

use super::table_format::MarkdownStyle;
use crate::utils;

#[derive(Debug)]
pub struct TableData {
    pub title: String,
    pub w: usize,
    pub h: usize,
    cell_max_w: usize,
    line_max_w: usize,
    data: Vec<Vec<String>>,
}

impl TableData {
    pub fn new(w: usize, h: usize, cell_max_w: usize, line_max_w: usize) -> Self {
        let data = vec![vec!["".to_string(); w]; h];
        Self {
            title: "".to_string(),
            w,
            h,
            cell_max_w,
            line_max_w,
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
        return cmp::min(w, self.line_max_w);
    }

    fn cell_line_widths(&self, is_markdown: bool) -> Vec<usize> {
        let mut cell_widths: Vec<usize> = Vec::new();
        for x in 0..self.w {
            // markdown 对齐需要三格
            cell_widths.push(std::cmp::min(
                std::cmp::max(self.width(x, is_markdown), 3),
                self.cell_max_w,
            ));
        }
        return cell_widths;
    }

    fn trim_start(&mut self) {
        // 清理首部连续空行
        let mut to_del: Vec<usize> = Vec::new();
        for (i, line) in self.data.iter().enumerate() {
            let mut flag = false;
            for text in line.iter() {
                if text.len() > 0 {
                    flag = true;
                    break;
                }
            }
            if !flag {
                to_del.push(i);
            } else {
                break;
            }
        }
        for i in to_del.iter().rev() {
            let _ = self.data.remove(*i);
        }
        self.h = self.data.len();
    }

    fn trim_end(&mut self) {
        let mut to_del: Vec<usize> = Vec::new();

        // 清理尾部连续空行
        for i in (0..self.data.len()).rev() {
            let line = &self.data[i];
            log::debug!("line: {:?}", line);
            let mut flag = false;
            for text in line.iter() {
                if text.len() > 0 {
                    flag = true;
                    break;
                }
            }
            if !flag {
                to_del.push(i);
            } else {
                break;
            }
        }
        log::debug!("data: {:?}", self.data);
        log::debug!("end size:{:?}", to_del);

        for i in to_del.iter() {
            let _ = self.data.remove(*i);
        }
        self.h = self.data.len();
    }

    pub fn trim(&mut self) {
        self.trim_start();
        self.trim_end();
    }

    pub fn to_normal_markdown_table(&self) -> String {
        return self.to_markdown_table(MarkdownStyle::Normal);
    }

    pub fn to_gfm_markdown_table(&self) -> String {
        return self.to_markdown_table(MarkdownStyle::Github);
    }

    fn to_markdown_table(&self, style: MarkdownStyle) -> String {
        let cell_widths = self.cell_line_widths(true);

        let mut content: Vec<String> = Vec::new();
        // 正文
        for line in self.data.iter() {
            let mut xline = String::new();
            for (j, cell) in line.iter().enumerate() {
                let (v1, v2) = (cell_widths[j], utils::cn_length(cell));
                let blank = if v1 > v2 {
                    " ".repeat(v1 - v2)
                } else {
                    "".to_string()
                };
                if style == MarkdownStyle::Normal || j != 0 {
                    xline.push_str("| ");
                }
                xline.push_str(cell);
                xline.push_str(blank.as_str());
                xline.push(' ');
            }
            if style == MarkdownStyle::Normal {
                xline.push('|');
            }
            content.push(xline.clone());
        }
        // 添加表格对齐
        // 检测是否需要添加
        let line: &String = content.get(1).unwrap();
        if !line.contains("-") {
            let mut nline = String::new();
            for (i, w) in cell_widths.iter().enumerate() {
                if style == MarkdownStyle::Normal || i != 0 {
                    nline.push_str("| ");
                }
                nline.push_str("-".repeat(*w).as_str());
                nline.push(' ');
            }
            if style == MarkdownStyle::Normal {
                nline.push('|');
            }
            content.insert(1, nline);
        } else {
            let mut nline = String::new();
            let cells = self.data.get(1).unwrap();
            for (w, cell) in cells.iter().enumerate() {
                if style == MarkdownStyle::Normal || w != 0 {
                    nline.push_str("| ");
                }
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
            if style == MarkdownStyle::Normal {
                nline.push('|');
            }
            content.insert(1, nline.trim().to_string());
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
        let cell_widths = self.cell_line_widths(false);

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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_data_new() {
        init();
        let mut data = TableData::new(5, 3, 33, 99);
        data.set_cell(0, 0, "1");
        data.set_cell(1, 0, "2");
        data.set_cell(2, 0, "3");
        data.set_cell(3, 0, "4");
        data.set_cell(4, 0, "5");
        log::debug!("test data: \n{:#?}", data)
    }

    #[test]
    fn test_str() {
        init();
        let a = "aa|a\tbb|b";
        let b = a.matches("|").count();
        log::debug!("len: {b}");
    }
}
