use super::node::{ADirect, ASharp};

fn split_node_char<'a>(
    input: &'a str,
    l: char,
    r: char,
) -> Option<(&'a str, &'a str, ASharp, &'a str)> {
    let sharp = match l {
        '(' => ASharp::Round,
        '[' => ASharp::Square,
        '{' => ASharp::Circle,
        _ => ASharp::Round,
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

pub fn parse_node(input: &str) -> (&str, &str, ASharp, &str) {
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
    let mut left: usize = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '-' || c == '<' || c == '>' || c == '^' {
            break;
        }
        left = i;
    }

    let (id, remain) = if left + 1 != input.len() {
        input.split_at(left + 1)
    } else {
        (input, "")
    };

    (id, id, ASharp::Round, remain)
}

pub fn get_arrow(input: &str) -> ADirect {
    if input.starts_with("<-") && input.ends_with("->") {
        return ADirect::Double;
    } else if input.starts_with("<-") {
        return ADirect::Left;
    } else if input.ends_with("->") {
        return ADirect::Right;
    } else if input.ends_with("-^") {
        return ADirect::Up;
    } else if input.ends_with("-v") {
        return ADirect::Down;
    } else if input.starts_with("<^-") {
        return ADirect::LeftUp;
    } else if input.starts_with("<v-") {
        return ADirect::LeftDown;
    } else if input.starts_with("-^>") {
        return ADirect::RightUp;
    } else if input.starts_with("-v>") {
        return ADirect::RightDown;
    }
    ADirect::None
}

pub fn parse_edge(input: &str) -> (ADirect, String, String) {
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
    let arrow = get_arrow(arrow.trim());
    return (arrow, a_text, remain.to_string());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_node_parse() {
        assert_eq!(parse_node("a"), ("a", "a", ASharp::Round, ""));
        assert_eq!(parse_node("a1(bb)"), ("a1", "bb", ASharp::Round, ""));
        assert_eq!(parse_node("a2[bb ]"), ("a2", "bb", ASharp::Square, ""));
        assert_eq!(parse_node("a3[你好]"), ("a3", "你好", ASharp::Square, ""));
        assert_eq!(
            parse_node("a4[你好] cc"),
            ("a4", "你好", ASharp::Square, "cc")
        );
        assert_eq!(
            parse_node("天下[天下神一舞]"),
            ("天下", "天下神一舞", ASharp::Square, "")
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

        assert_eq!(parse_edge("-->").0, ADirect::Right);
        assert_eq!(
            parse_edge("--|aaa|-->bb"),
            (ADirect::Right, String::from("aaa"), String::from("bb"))
        );
        assert_eq!(parse_edge("<--").0, ADirect::Left);
        assert_eq!(parse_edge("<-->").0, ADirect::Double);
        assert_eq!(parse_edge("<-->").0, ADirect::Double);
        assert_eq!(parse_edge("--^").0, ADirect::Up);
        assert_eq!(parse_edge("--v").0, ADirect::Down);
        assert_eq!(parse_edge("-^>").0, ADirect::RightUp);
        assert_eq!(parse_edge("-v>").0, ADirect::RightDown);
        assert_eq!(parse_edge("<^-").0, ADirect::LeftUp);
        assert_eq!(parse_edge("<v-").0, ADirect::LeftDown);
    }
}
