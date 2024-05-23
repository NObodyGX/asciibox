use nom::bytes::complete::{is_a, take_until, take_while};

use nom::error::ParseError;
use nom::sequence::{delimited, terminated};
use nom::{AsChar, IResult, InputTakeAtPosition};

use super::data::GDirect;

pub fn valid_node_check<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        c == '-'
            || c == '<'
            || c == '>'
            || c == ';'
            || c == '\t'
            || c == '\r'
            || c == '\n'
    })
}


fn valid_name_char_check<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        c == '('
            || c == '['
            || c == '{'
            || c == '-'
            || c == '-'
            || c == '<'
            || c == '>'
            || c == ';'
            || c == '\t'
            || c == '\r'
            || c == '\n'
    })
}

// <--, -->, ---, <-->,
// ^--, --^, v--, --v
// <^-, <v-, -^>, -v>
pub fn valid_arrow_check<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        !(c == '-' || c == '<' || c == '>' || c == '^' || c == 'v')
    })
}

pub fn parse_node(input: &str) -> IResult<&str, &str> {
    let (remain, id) = valid_name_char_check(input)?;
    if remain.len() < 3
    || !(remain.starts_with("[") || remain.starts_with("(") || remain.starts_with("<")) {
        return Ok((id, id));
    }
    let (_remain, name) = delimited(is_a("[(<"), valid_name_char_check, is_a("])>"))(remain)?;
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
