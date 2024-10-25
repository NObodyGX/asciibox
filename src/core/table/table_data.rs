macro_rules! vec2d {
    ( $($($x:expr),+ $(,)?);* $(;)? ) => (
        vec![$(
            vec![$($x),+],
        )*]
    );
}

#[derive(Debug)]
pub struct TableData {
    pub title: String,
    pub w: usize,
    pub h: usize,
    data: Vec<Vec<String>>,
}

impl TableData {
    pub fn new(w: usize, h: usize) -> Self {
        let data = vec![vec!["".to_string(); h]; w];
        Self {
            title: "".to_string(),
            w,
            h,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_new() {
        let data = TableData::new(10, 2);
        println!("{:#?}", data)
    }
}
