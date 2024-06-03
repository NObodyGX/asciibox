pub fn cn_length(input: &str) -> usize {
    let a: usize = input.len();
    let b: usize = input.chars().count();
    (a + b) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(cn_length("nihao"), 5);
        assert_eq!(cn_length("你好"), 4);
        assert_eq!(cn_length("你好hah"), 7);
        assert_eq!(cn_length("？。hah"), 7);
    }
}
