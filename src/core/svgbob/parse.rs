use super::node::{GDirect, GSharp};

fn split_node_char<'a>(
    input: &'a str,
    l: char,
    r: char,
) -> Option<(&'a str, &'a str, GSharp, &'a str)> {
    let sharp = match l {
        '(' => GSharp::Round,
        '[' => GSharp::Square,
        '{' => GSharp::Circle,
        _ => GSharp::Round,
    };

    match input.find(l) {
        Some(v) => match input.find(r) {
            Some(vv) => {
                let (node, r) = input.split_at(vv);
                let (id, n) = node.split_at(v);
                let (_, name) = n.split_at(1);
                let (_, remain) = r.split_at(1);
                return Some((id.trim(), name.trim(), sharp, remain.trim()));
            }
            None => {
                let (id, n) = input.split_at(v);
                let (_, name) = n.split_at(1);
                return Some((id.trim(), name.trim(), sharp, ""));
            }
        },
        None => {
            return None;
        }
    }
}

pub fn parse_node(input: &str) -> (&str, &str, GSharp, &str) {
    match split_node_char(input, '(', ')') {
        Some(v) => return v,
        None => {}
    }
    match split_node_char(input, '[', ']') {
        Some(v) => return v,
        None => {}
    }
    match split_node_char(input, '{', '}') {
        Some(v) => return v,
        None => {}
    }
    (input, input, GSharp::Round, "")
}

pub fn get_arrow(input: &str) -> GDirect {
    if input.starts_with("<-") && input.ends_with("->") {
        return GDirect::Double;
    } else if input.starts_with("<-") {
        return GDirect::Left;
    } else if input.ends_with("->") {
        return GDirect::Right;
    } else if input.ends_with("-^") {
        return GDirect::Up;
    } else if input.ends_with("-v") {
        return GDirect::Down;
    } else if input.starts_with("<^-") {
        return GDirect::LeftUp;
    } else if input.starts_with("<v-") {
        return GDirect::LeftDown;
    } else if input.starts_with("-^>") {
        return GDirect::RightUp;
    } else if input.starts_with("-v>") {
        return GDirect::RightDown;
    }
    GDirect::None
}

pub fn parse_arrow(input: &str) -> (GDirect, String, String) {
    let arrow: &str;
    let remain: &str;

    let mut state: usize = 0;
    let mut com_begin: usize = 0;
    let mut com_end: usize = 0;
    let mut end: usize = 0;
    // 0-正常
    // 1-进入箭头文字
    // 2-退出箭头文字
    for (i, c) in input.chars().enumerate() {
        if c == '|' {
            if state == 1 {
                state = 2;
                com_end = i;
            } else {
                state = 1;
                com_begin = i;
            }
            continue;
        }
        if c == '-' || c == '<' || c == '>' || c == ' ' || c == '^' || c == 'v' {
            end = i;
            continue;
        }
        if state != 1 {
            break;
        }
    }
    if end == 0 {
        (arrow, remain) = (input, "");
    } else {
        (arrow, remain) = input.split_at(end + 1);
    }
    let a_text: String = if com_begin == com_end {
        "".to_string()
    } else {
        String::from(arrow)
            .get(com_begin + 1..com_end)
            .unwrap()
            .to_string()
    };
    // TODO, parse arrow text
    let arrow = get_arrow(arrow);
    return (arrow, a_text, remain.to_string());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_node_parse() {
        assert_eq!(parse_node("a"), ("a", "a", GSharp::Round, ""));
        assert_eq!(parse_node("a1(bb)"), ("a1", "bb", GSharp::Round, ""));
        assert_eq!(parse_node("a2[bb ]"), ("a2", "bb", GSharp::Square, ""));
        assert_eq!(parse_node("a3[你好]"), ("a3", "你好", GSharp::Square, ""));
        assert_eq!(
            parse_node("a4[你好] cc"),
            ("a4", "你好", GSharp::Square, "cc")
        );
        assert_eq!(
            parse_node("天下[天下神一舞]"),
            ("天下", "天下神一舞", GSharp::Square, "")
        );
    }

    #[test]
    fn test_arrow_parse() {
        // 只支持两种
        // -->
        // --^
        // "--> --^ --v"
        // "--|aa|-->"
        // "--^> --v>"

        assert_eq!(parse_arrow("-->").0, GDirect::Right);
        assert_eq!(
            parse_arrow("--|aaa|-->bb"),
            (GDirect::Right, String::from("aaa"), String::from("bb"))
        );
        assert_eq!(parse_arrow("<--").0, GDirect::Left);
        assert_eq!(parse_arrow("<-->").0, GDirect::Double);
        assert_eq!(parse_arrow("<-->").0, GDirect::Double);
        assert_eq!(parse_arrow("--^").0, GDirect::Up);
        assert_eq!(parse_arrow("--v").0, GDirect::Down);
        assert_eq!(parse_arrow("-^>").0, GDirect::RightUp);
        assert_eq!(parse_arrow("-v>").0, GDirect::RightDown);
        assert_eq!(parse_arrow("<^-").0, GDirect::LeftUp);
        assert_eq!(parse_arrow("<v-").0, GDirect::LeftDown);
    }
}
