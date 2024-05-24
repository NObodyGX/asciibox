use nom::bytes::complete::{is_a, is_not, take_till};
use nom::sequence::delimited;
use nom::IResult;

use super::data::GDirect;

pub fn valid_node_check(input:&str) -> IResult<&str, &str> {
    take_till(|c| c == '-' || c == '<' || c == '>' || c == '\n')(input)
}

fn valid_name_check(input:&str) -> IResult<&str, &str> {
    take_till(|c|  c == '[' || c == '(')(input)
}

// <--, -->, ---, <-->,
// ^--, --^, v--, --v
// <^-, <v-, -^>, -v>
pub fn valid_arrow_check(input:&str) -> IResult<&str, &str> {
    take_till(|c|  c != '-' && c != '<' && c != '>' && c != '^' && c != 'v')(input)
}

pub fn parse_node(input: &str) -> IResult<&str, &str> {
    let (remain, id) = valid_name_check(input)?;
    if remain.len() < 3
    || !(remain.starts_with("[") || remain.starts_with("(")) {
        return Ok((id, id));
    }
    let (_remain, name) = delimited(is_a("[("), is_not("])"), is_a("])"))(remain)?;
    Ok((id, name))
}


pub fn parse_arrow(input:&str)-> GDirect {
    if input.starts_with("<-") {
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
    } else if input.starts_with("<") && input.ends_with(">") {
        return GDirect::Double;
    }
    GDirect::None
}
