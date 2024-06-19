#[cfg(test)]
mod tests {

    use crate::core::svgbob::GSMap;

    #[test]
    fn test_map_render() {
        let mut gmap = GSMap::new(true);
        let m1code = "a";
        let mut result = String::new();
        result.push_str(".---.\n");
        result.push_str("| a |\n");
        result.push_str("'---'\n");
        assert_eq!(gmap.load_content(m1code), result);

        let mcode = "a[123]";
        result = String::new();
        result.push_str("+-----+\n");
        result.push_str("| 123 |\n");
        result.push_str("+-----+\n");
        assert_eq!(gmap.load_content(mcode), result);

        let mcode = "aa ---> b";
        result = String::new();
        result.push_str(".----.   .---.\n");
        result.push_str("| aa |-->| b |\n");
        result.push_str("'----'   '---'\n");
        assert_eq!(gmap.load_content(mcode), result);

        let mcode = "a-->b-->c --> d";
        result = String::new();
        result.push_str(".---.   .---.   .---.   .---.\n");
        result.push_str("| a |-->| b |-->| c |-->| d |\n");
        result.push_str("'---'   '---'   '---'   '---'\n");
        assert_eq!(gmap.load_content(mcode), result);

        let mcode = "aaa <--- b";
        result = String::new();
        result.push_str(".-----.   .---.\n");
        result.push_str("| aaa |<--| b |\n");
        result.push_str("'-----'   '---'\n");
        assert_eq!(gmap.load_content(mcode), result);

        let mcode = "aba ---v b";
        result = String::new();
        result.push_str(".-----.\n");
        result.push_str("| aba |\n");
        result.push_str("'-----'\n");
        result.push_str("   |\n");
        result.push_str("   v\n");
        result.push_str(".-----.\n");
        result.push_str("|  b  |\n");
        result.push_str("'-----'\n");
        assert_eq!(gmap.load_content(mcode), result);

        let mcode = "aca ---^ b";
        result = String::new();
        result.push_str(".-----.\n");
        result.push_str("|  b  |\n");
        result.push_str("'-----'\n");
        result.push_str("   ^\n");
        result.push_str("   |\n");
        result.push_str(".-----.\n");
        result.push_str("| aca |\n");
        result.push_str("'-----'\n");
        assert_eq!(gmap.load_content(mcode), result);
    }

    #[test]
    fn test_map_group_render() {
        let mut gmap = GSMap::new(true);
        let mut result = String::new();
        let code = "b <-- a --> c\n a --^ u\n a --v d";
        result.push_str(
            "
        .---.
        | u |
        '---'
          ^
          |
.---.   .---.   .---.
| b |<--| a |-->| c |
'---'   '---'   '---'
          |
          v
        .---.
        | d |
        '---'
",
        );
        assert_eq!(gmap.load_content(code), result[1..]);
    }
}
