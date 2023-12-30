use nom::bytes::complete::{tag, take_while1};
use nom::{complete, IResult, Parser};
use nom::branch::alt;
use nom::character::complete::{space0, space1};
use nom::character::{is_alphanumeric, is_digit};


pub fn parse(s: &str) -> IResult<&str, Vec<Box<Message>>> {
    ::nom::multi::many1(parse_message).parse(s)
}

struct Message {}


pub fn parse_message(s: &str) -> IResult<&str, Box<Message>> {
    let (s, _) = space0.parse(s)?;
    let (s, _) = tag("Message").parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, _) = tag("{").parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, _) = parse_field.parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, _) = tag("}").parse(s)?;
    let (s, _) = space0.parse(s)?;

    Ok((s, Box::new(Message {})))
}

enum EnumLine {
    EnumLineField(EnumLineField),
    EnumLineComment(EnumLineComment),
}

struct EnumLineField {}
struct EnumLineComment {}

pub fn parse_line(s: &str) -> IResult<&str, EnumLine> {
    let (s, _) = space0.parse(s)?;
    let (s, v1) = alt((parse_field, tag("repeated"))).parse(s)?;
    let (s, _) = space0.parse(s)?;
    Ok((s, EnumLine {}))
}

struct MessageField;

pub fn parse_comment(s: &str) -> IResult<&str, EnumLine> {
    Ok((s, EnumLine::EnumLineComment(EnumLineComment {
        
    })))
}

pub fn parse_field(s: &str) -> IResult<&str, EnumLine> {

    let (s, _) = space0.parse(s)?;
    let (s, v1) = alt((tag("optional"), tag("repeated"))).parse(s)?;
    let (s, _) = space1.parse(s)?;
    let (s, v1) = alt((
        tag("string"),
        tag("uint64"),
        tag("uint32"),
        tag("int64"),
        tag("int32"),
        tag("bool"),
        tag("bytes")
    )).parse(s)?;
    let (s, _) = space1.parse(s)?;
    let (s, ident) = take_while1(is_alphanumeric).parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, _) = tag("=").parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, version) = take_while1(is_digit).parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, _) = tag(";").parse(s)?;

    Ok((s, EnumLine::EnumLineField(EnumLineField {})))
}


#[test]
fn parse_empty() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse("Message{}Message{}")?;
    assert_eq!(b.len(), 2);
    Ok(())
}

#[test]
fn parse_whitespace() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse("Message { } Message { } ")?;
    assert_eq!(b.len(), 2);
    Ok(())
}
