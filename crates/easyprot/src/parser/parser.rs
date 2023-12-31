use nom::bytes::complete::{tag, take_until, take_while1};
use nom::{complete, IResult, Parser};
use nom::branch::alt;
use nom::character::complete::{space0, space1};
use nom::character::{is_alphanumeric, is_digit};
use nom::multi::many0;


pub fn parse(s: &str) -> IResult<&str, ProtoAst> {

    let (s, items) = ::nom::multi::many0(parse_message).parse(s)?;

    Ok((s, ProtoAst {
        syntax: None,
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


pub fn parse_message(s: &str) -> IResult<&str, Box<ItemMessage>> {
    let (s, _) = space0.parse(s)?;
    let (s, _) = tag("Message").parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, _) = tag("{").parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, lines) = many0(parse_field).parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, _) = tag("}").parse(s)?;
    let (s, _) = space0.parse(s)?;

    Ok((s, Box::new(ItemMessage {
        lines
    })))
}

enum EnumLine {
    EnumLineField(EnumLineField),
}

struct EnumLineField {}

pub fn parse_line(s: &str) -> IResult<&str, EnumLine> {
    let (s, _) = space0.parse(s)?;
    let (s, v1) = parse_field.parse(s)?;
    let (s, _) = space0.parse(s)?;
    Ok((s, EnumLine::EnumLineField(EnumLineField {

    })))
}

struct MessageField;


pub fn parse_field(s: &str) -> IResult<&str, EnumLine> {

    let (s, comments1) = parse_field_comments_dockblock.parse(s)?;

    let (s, _) = space0.parse(s)?;
    let (s, v1) = alt((tag("optional"), tag("repeated"))).parse(s)?;
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
    let (s, _) = space0.parse(s)?;
    let (s, _) = tag("=").parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, comments4) = parse_field_comments_dockblock.parse(s)?;
    let (s, version) = take_while1(|x| is_digit(x as u8)).parse(s)?;
    let (s, _) = space0.parse(s)?;
    let (s, comments4) = parse_field_comments_dockblock.parse(s)?;
    let (s, _) = tag(";").parse(s)?;
    let (s, comments4) = parse_field_comments_dockblock.parse(s)?;

    Ok((s, EnumLine::EnumLineField(EnumLineField {})))
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
fn parse_empty() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse("Message{}Message{}")?;
    assert_eq!(b.items.len(), 2);
    Ok(())
}

#[test]
fn parse_whitespace() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse("Message { } Message { } ")?;
    assert_eq!(b.items.len(), 2);
    Ok(())
}
