use nom::bytes::complete::{is_a, is_not, take_till};
use nom::sequence::delimited;
use nom::IResult;

use super::node::GDirect;

pub fn valid_node_check(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == '-' || c == '<' || c == '>' || c == '\n')(input)
}

fn valid_name_check(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == '[' || c == '(')(input)
}

// <--, -->, ---, <-->,
// ^--, --^, v--, --v
// <^-, <v-, -^>, -v>
pub fn valid_arrow_check(input: &str) -> IResult<&str, &str> {
    take_till(|c| c != '-' && c != '<' && c != '>' && c != '^' && c != 'v')(input)
}

pub fn parse_node(input: &str) -> IResult<&str, &str> {
    let (remain, id) = valid_name_check(input)?;
    if remain.len() < 3 || !(remain.starts_with("[") || remain.starts_with("(")) {
        return Ok((id, id));
    }
    let (_remain, name) = delimited(is_a("[("), is_not("])"), is_a("])"))(remain)?;
    Ok((id, name))
}

pub fn parse_arrow(input: &str) -> GDirect {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_node_parse() {
        assert_eq!(parse_node("a"), Ok(("a", "a")));
        assert_eq!(parse_node("a1(bb)"), Ok(("a1", "bb")));
        assert_eq!(parse_node("a2[bb] c"), Ok(("a2", "bb")));
        assert_eq!(parse_node("a3[你好]"), Ok(("a3", "你好")));
        assert_eq!(
            parse_node("a4[梦九天\n无应变]"),
            Ok(("a4", "梦九天\n无应变"))
        );
        assert_eq!(
            parse_node("a5[梦九天\naaa\n七重关]"),
            Ok(("a5", "梦九天\naaa\n七重关"))
        );
        assert_eq!(parse_node("天下[天下神一舞]"), Ok(("天下", "天下神一舞")));
    }

    #[test]
    fn test_arrow_parse() {
        assert_eq!(parse_arrow("-->").to_string(), GDirect::Right.to_string());
        assert_eq!(parse_arrow("<--").to_string(), GDirect::Left.to_string());
        assert_eq!(parse_arrow("<-->").to_string(), GDirect::Double.to_string());
        assert_eq!(parse_arrow("<->").to_string(), GDirect::Double.to_string());
        assert_eq!(parse_arrow("--^").to_string(), GDirect::Up.to_string());
        assert_eq!(parse_arrow("--v").to_string(), GDirect::Down.to_string());
        assert_eq!(parse_arrow("-^>").to_string(), GDirect::RightUp.to_string());
        assert_eq!(
            parse_arrow("-v>").to_string(),
            GDirect::RightDown.to_string()
        );
        assert_eq!(parse_arrow("<^-").to_string(), GDirect::LeftUp.to_string());
        assert_eq!(
            parse_arrow("<v-").to_string(),
            GDirect::LeftDown.to_string()
        );
    }
}
