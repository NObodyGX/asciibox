use grid::Grid;

pub struct AsciiGrid {
    grid: Grid<char>,
}

impl AsciiGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Grid::new(height, width);
        grid.fill(' ');
        Self { grid }
    }

    pub fn set(&mut self, x: usize, y: usize, ch: char) {
        if y < self.grid.rows() && x < self.grid.cols() {
            self.grid[(y, x)] = ch;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        if y < self.grid.rows() && x < self.grid.cols() {
            Some(self.grid[(y, x)])
        } else {
            None
        }
    }

    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, ch: char) {
        // Bresenham 算法
        let dx = x2 as i32 - x1 as i32;
        let dy = y2 as i32 - y1 as i32;
        let steps = dx.abs().max(dy.abs());

        for i in 0..=steps {
            let x = x1 as f32 + (dx as f32 * i as f32 / steps as f32);
            let y = y1 as f32 + (dy as f32 * i as f32 / steps as f32);
            self.set(x.round() as usize, y.round() as usize, ch);
        }
    }

    pub fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize) {
        // 顶边
        for i in 0..width {
            self.set(x + i, y, '-');
        }
        // 底边
        for i in 0..width {
            self.set(x + i, y + height - 1, '-');
        }
        // 左边
        for i in 0..height {
            self.set(x, y + i, '|');
        }
        // 右边
        for i in 0..height {
            self.set(x + width - 1, y + i, '|');
        }
        // 角落
        self.set(x, y, '+');
        self.set(x + width - 1, y, '+');
        self.set(x, y + height - 1, '+');
        self.set(x + width - 1, y + height - 1, '+');
    }

    pub fn draw_box_with_name(
        &mut self,
        name: &String,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        self.draw_box(x, y, width, height);
        for (j, line) in name.split("\n").enumerate() {
            for (i, ch) in line.chars().enumerate() {
                // 从左开始，但保留一个空格
                self.set(x + i + 2, y + j + 1, ch);
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for row in self.grid.iter_rows() {
            for ch in row {
                result.push(*ch);
            }
            result.push('\n');
        }
        result
    }
}

mod test {

    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn test_abc() {
        let mut grid = AsciiGrid::new(24, 6);

        // 绘制流程图节点
        grid.draw_box(2, 1, 16, 3); // Start 框
        // grid.draw_box(2, 6, 12, 3); // End 框

        // 绘制连接线
        //grid.draw_line(6, 4, 6, 6, '|');

        // 添加文本（需要手动实现）
        // ...

        let ascii = grid.to_string();
        println!("{}", ascii);

        // 转换为 SVG
        let svg = svgbob::to_svg(&ascii);
        std::fs::write("output.svg", svg).unwrap();
    }
}
