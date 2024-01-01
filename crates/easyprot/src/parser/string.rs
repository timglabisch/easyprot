use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{escaped, is_not, tag, take_until};
use nom::character::complete::{char, none_of};
use nom::combinator::{not, verify};
use crate::parser::parser::{parse, parse_message};
use nom::multi::{fold_many0, many0};
use nom::sequence::delimited;


pub fn parse_string(s: &str) -> IResult<&str, String> {
    let (s, v) = escaped(none_of("\""), '\\', tag("\"")).parse(s)?;
    Ok((s, v.to_string()))
}

#[test]
fn test_parse_string() -> Result<(), ::anyhow::Error> {
    let (v, _) = parse_string(r#""foo foo""#)?;
    assert_eq!(v, r#""foo"foo""#);
    Ok(())
}