use std::usize;

use super::cell::{ASharp, Direct};

#[allow(dead_code)]
pub fn parse_node(input: &str) -> (String, String, ASharp, String) {
    let mut isharp = ASharp::Round;
    let mut iid = String::new();
    let mut iname = String::new();
    let mut iremain = String::new();
    let mut state: u8 = 0;

    for c in input.chars() {
        match c {
            '[' => {
                state = 1;
                isharp = ASharp::Round;
            }
            '(' => {
                state = 1;
                isharp = ASharp::Square;
            }
            '{' => {
                state = 1;
                isharp = ASharp::Circle;
            }
            ']' | ')' | '}' => {
                if state == 1 {
                    state = 2;
                }
            }
            '-' | '<' | '>' => match state {
                0 => {
                    state = 2;
                    iremain.push(c);
                }
                1 => iname.push(c),
                2 => iremain.push(c),
                _ => {}
            },
            _ => match state {
                0 => iid.push(c),
                1 => iname.push(c),
                2 => iremain.push(c),
                _ => {}
            },
        }
    }

    if iname.len() == 0 {
        return (iid.clone(), iid, isharp, iremain);
    }

    (iid, iname, isharp, iremain)
}

pub fn get_arrow(input: &str) -> Direct {
    if input.starts_with("<-") && input.ends_with("->") {
        return Direct::Double;
    } else if input.starts_with("<-") {
        return Direct::Left;
    } else if input.ends_with("->") {
        return Direct::Right;
    } else if input.ends_with("-^") {
        return Direct::Up;
    } else if input.ends_with("-v") {
        return Direct::Down;
    } else if input.starts_with("<^-") {
        return Direct::LeftUp;
    } else if input.starts_with("<v-") {
        return Direct::LeftDown;
    } else if input.starts_with("-^>") {
        return Direct::RightUp;
    } else if input.starts_with("-v>") {
        return Direct::RightDown;
    }
    Direct::None
}

pub fn parse_edge(input: &str) -> (Direct, String, String) {
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
    fn test_chinese_parse() {
        let a = "你好 --> aaaa";
        assert_eq!(a.len(), a.chars().count());
    }

    fn ppp(i: &str, o1: &str, o2: &str, sharp: ASharp, o3: &str) {
        assert_eq!(
            parse_node(i),
            (o1.to_string(), o2.to_string(), sharp, o3.to_string())
        );
    }

    #[test]
    fn test_node_parse() {
        ppp("a", "a", "a", ASharp::Round, "");
        ppp("a1(bb)", "a1", "bb", ASharp::Square, "");
        ppp("a2[bb ]", "a2", "bb ", ASharp::Round, "");
        ppp("a3[你好]", "a3", "你好", ASharp::Round, "");
        ppp("a3[你好] cc", "a3", "你好", ASharp::Round, " cc");
        ppp(
            "天下无敌[天上来客]",
            "天下无敌",
            "天上来客",
            ASharp::Round,
            "",
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

        assert_eq!(parse_edge("-->").0, Direct::Right);
        assert_eq!(
            parse_edge("--|aaa|-->bb"),
            (Direct::Right, String::from("aaa"), String::from("bb"))
        );
        assert_eq!(parse_edge("<--").0, Direct::Left);
        assert_eq!(parse_edge("<-->").0, Direct::Double);
        assert_eq!(parse_edge("<-->").0, Direct::Double);
        assert_eq!(parse_edge("--^").0, Direct::Up);
        assert_eq!(parse_edge("--v").0, Direct::Down);
        assert_eq!(parse_edge("-^>").0, Direct::RightUp);
        assert_eq!(parse_edge("-v>").0, Direct::RightDown);
        assert_eq!(parse_edge("<^-").0, Direct::LeftUp);
        assert_eq!(parse_edge("<v-").0, Direct::LeftDown);
    }
}
