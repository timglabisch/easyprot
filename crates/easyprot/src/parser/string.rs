use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{escaped, is_not, tag, take_until};
use nom::character::complete::{anychar, char, none_of};
use nom::combinator::{map, not, value, verify};
use crate::parser::parser::{parse, parse_message};
use nom::multi::{fold_many0, many0, many_till};
use nom::sequence::{delimited, preceded};


#[derive(Debug, Clone, PartialEq, Eq)]
enum StringFragment {
    Literal(String),
    EscapedChar(String),
}

fn parse_escaped_char(input: &str) -> IResult<&str, String> {
    let (s, char) = tag("\\\"")(input)?;

    Ok((s, char.to_string()))
}

fn parse_literal(s: &str) -> IResult<&str, String> {

    // `is_not` parses a string of 0 or more characters that aren't one of the
    // given characters.
    let not_quote_slash = is_not("\"\\");

    // `verify` runs a parser, then runs a verification function on the output of
    // the parser. The verification function accepts out output only if it
    // returns true. In this case, we want to ensure that the output of is_not
    // is non-empty.
    let (s, v) = verify(not_quote_slash, |s: &str| !s.is_empty())(s)?;

    Ok((s, v.to_string()))
}

fn parse_fragment(input: &str) -> IResult<&str, StringFragment>
{
    let (s, v) = alt((
        // The `map` combinator runs a parser, then applies a function to the output
        // of that parser.
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(input)?;

    Ok((s, v))
}

pub fn parse_string(s: &str) -> IResult<&str, String> {
    let build_string = fold_many0(
        // Our parser function– parses a single string fragment
        parse_fragment,
        // Our init value, an empty string
        String::new,
        // Our folding function. For each fragment, append the fragment to the
        // string.
        |mut string, fragment| {
            match fragment {
                StringFragment::Literal(s) => string.push_str(&s),
                StringFragment::EscapedChar(c) => string.push_str(&c),
            }
            string
        },
    );

    // Finally, parse the string. Note that, if `build_string` could accept a raw
    // " character, the closing delimiter " would never match. When using
    // `delimited` with a looping parser (like fold_many0), be sure that the
    // loop won't accidentally match your closing delimiter!
    let (s, v) = delimited(char('"'), build_string, char('"'))(s)?;

    Ok((s, v))
}

#[test]
fn test_parse_string() -> Result<(), ::anyhow::Error> {
    assert_eq!(parse_string(r#""foo foo""#)?.1, r#"foo foo"#);
    assert_eq!(parse_string(r#""foo\"foo""#)?.1, r#"foo\"foo"#);

    Ok(())
}