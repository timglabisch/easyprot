use nom::bytes::complete::{tag, take_until, take_while1};
use nom::{complete, IResult, Parser};
use nom::branch::alt;
use nom::character::complete::{multispace0, space0, space1};
use nom::character::{is_alphanumeric, is_digit};
use nom::combinator::opt;
use nom::multi::many0;
use crate::parser::string::parse_string;


pub fn parse(s: &str) -> IResult<&str, ProtoAst> {

    let (s, _) = multispace0.parse(s)?;
    let (s, syntax) = opt(parse_syntax).parse(s)?;
    let (s, items) = ::nom::multi::many0(parse_message).parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    Ok((s, ProtoAst {
        syntax,
        items
    }))
}

struct ProtoAst {
    syntax: Option<String>,
    items: Vec<Box<ItemMessage>>,
}

enum Item {
    ItemMessage(ItemMessage),
}

struct ItemMessage {
    lines: Vec<EnumLine>
}

pub fn parse_syntax(s: &str) -> IResult<&str, String> {
    let (s, _) = tag("syntax").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("=").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, version) = parse_string.parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag(";").parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    Ok((s, version))
}

pub fn parse_message(s: &str) -> IResult<&str, Box<ItemMessage>> {
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("Message").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("{").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, lines) = many0(parse_field).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("}").parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    Ok((s, Box::new(ItemMessage {
        lines
    })))
}

#[derive(Debug, Eq, PartialEq)]
enum EnumLine {
    EnumLineField(EnumLineField),
}

#[derive(Debug, Eq, PartialEq)]
struct EnumLineField {}

pub fn parse_line(s: &str) -> IResult<&str, EnumLine> {
    let (s, _) = multispace0.parse(s)?;
    let (s, v1) = parse_field.parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    Ok((s, EnumLine::EnumLineField(EnumLineField {

    })))
}

struct MessageField;

enum MessageFieldModifierCount {
    OPTIONAL,
    REPEATED,
}

pub fn parse_field(s: &str) -> IResult<&str, EnumLine> {

    let (s, comments1) = parse_field_comments_dockblock.parse(s)?;

    let (s, _) = multispace0.parse(s)?;
    let (s, modifier) = alt((tag("optional"), tag("repeated"))).parse(s)?;
    let (s, _) = space1.parse(s)?;

    let (s, comments2) = parse_field_comments_dockblock.parse(s)?;

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
    let (s, comments2) = parse_field_comments_dockblock.parse(s)?;
    let (s, ident) = take_while1(|x| is_alphanumeric(x as u8)).parse(s)?;
    let (s, comments3) = parse_field_comments_dockblock.parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("=").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, comments4) = parse_field_comments_dockblock.parse(s)?;
    let (s, version) = take_while1(|x| is_digit(x as u8)).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, comments4) = parse_field_comments_dockblock.parse(s)?;
    let (s, _) = tag(";").parse(s)?;
    let (s, comments4) = parse_field_comments_dockblock.parse(s)?;

    Ok((s, EnumLine::EnumLineField(EnumLineField {
   
    })))
}

struct FieldComment {
    comment: String,
}

pub fn parse_field_comments_dockblock(s: &str) -> IResult<&str, Vec<FieldComment>> {
    let (s, buf) = many0(parse_field_comment_dockblock).parse(s)?;
    Ok((s, buf))
}

pub fn parse_field_comment_dockblock(s: &str) -> IResult<&str, FieldComment> {
    let (s, buf) = tag("/**").parse(s)?;
    let (s, comment) = take_until("*/").parse(s)?;

    Ok((s, FieldComment {
        comment: comment.trim().to_string()
    }))
}


#[test]
fn test_parse_empty() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse("Message{}Message{}")?;
    assert_eq!(b.items.len(), 2);
    Ok(())
}

#[test]
fn test_parse_whitespace() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse("Message { } Message { } ")?;
    assert_eq!(b.items.len(), 2);
    Ok(())
}

#[test]
fn test_parse_multiline() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse(r#"
    Message
    {
    }
    "#)?;
    assert_eq!(b.items.len(), 1);
    Ok(())
}

#[test]
fn test_parse_syntax() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse(r#"syntax = "proto3"; Message { } "#)?;
    assert_eq!(b.syntax, Some("proto3".to_string()));
    Ok(())
}

#[test]
fn test_parse_fields() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse(r#"
    Message {
        optional string foo = 1;
        repeated string foo2 = 2;
    }
    "#)?;

    assert_eq!(b.items.len(), 1);
    assert_eq!(b.items[0].lines.len(), 2);
    assert_eq!(b.items[0].lines[0], EnumLine::EnumLineField(EnumLineField {

    }));

    Ok(())
}
