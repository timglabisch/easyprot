use nom::bytes::complete::{tag, tag_no_case, take_until, take_while1};
use nom::{complete, IResult, Parser};
use nom::branch::alt;
use nom::character::complete::{multispace0, multispace1, space0, space1};
use nom::character::{is_alphanumeric, is_digit};
use nom::combinator::{map, opt, value};
use nom::error::{ErrorKind, ParseError};
use nom::multi::many0;
use crate::parser::string::parse_string;


pub fn parse(s: &str) -> IResult<&str, ProtoAst> {

    let (s, _) = multispace0.parse(s)?;
    let (s, syntax) = opt(parse_syntax).parse(s)?;
    let (s, items) = ::nom::multi::many0(parse_element).parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    Ok((s, ProtoAst {
        syntax,
        items
    }))
}

pub fn parse_element(s: &str) -> IResult<&str, Element> {
    let (s, v) = alt((
        map(parse_message, |v| Element::ItemMessage(v)),
        map(parse_enum, |v| Element::ItemEnum(v)),
    )).parse(s)?;

    Ok((s, v))
}

struct ProtoAst {
    syntax: Option<String>,
    items: Vec<Element>,
}

enum Element {
    ItemMessage(ItemMessage),
    ItemEnum(ItemEnum),
}

struct ItemMessage {
    ident: String,
    fields: Vec<ItemMessageField>
}

struct ItemEnum {
    ident: String,
    fields: Vec<ItemEnumField>
}

pub fn parse_syntax(s: &str) -> IResult<&str, String> {
    let (s, _) = tag_no_case("syntax").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("=").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, version) = parse_string.parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag(";").parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    Ok((s, version))
}

enum Item {
    Message(ItemMessage),
    Enum(ItemEnum)
}

pub fn parse_enum(s: &str ) -> IResult<&str, ItemEnum> {
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag_no_case("Enum").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, ident) = take_while1(|v| is_alphanumeric(v as u8)).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("{").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, fields) = many0(parse_enum_field).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("}").parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    Ok((s, ItemEnum {
        ident: ident.to_string(),
        fields
    }))
}

pub fn parse_message(s: &str) -> IResult<&str, ItemMessage> {
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag_no_case("Message").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, ident) = take_while1(|v| is_alphanumeric(v as u8)).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("{").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, fields) = many0(alt((
        parse_message_field_standard,
        parse_message_field_oneof,
    ))).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("}").parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    Ok((s, ItemMessage {
        ident: ident.to_string(),
        fields
    }))
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
enum ItemMessageField {
    STANDARD(ItemMessageFieldStandard),
    ONEOF(ItemMessageFieldOneOf)
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
struct ItemMessageFieldStandard {
    modifier: Option<MessageFieldModifierCount>,
    field_type: MessageFieldType,
    ident: String,
    id: u64,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
struct ItemMessageFieldOneOf {
    fields: Vec<ItemMessageField>,
    ident: String,
}

#[derive(Debug, Eq, PartialEq)]
struct ItemEnumField {
    ident: String,
    id: u64,
}

struct MessageField;

#[derive(Clone, Debug, Eq, PartialOrd, PartialEq)]
enum MessageFieldModifierCount {
    OPTIONAL,
    REPEATED,
}

#[derive(Clone, Debug, Eq, PartialOrd, PartialEq)]
enum MessageFieldType {
    STRING,
    UINT64,
    UINT32,
    INT64,
    INT32,
    BOOL,
    BYTES,
}

pub fn parse_enum_field(s: &str) -> IResult<&str, ItemEnumField> {
    let (s, _) = multispace0.parse(s)?;
    let (s, ident) = take_while1(|x| is_alphanumeric(x as u8)).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("=").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, id) : (_, &str) = take_while1(|x| is_digit(x as u8)).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag(";").parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    Ok((s, ItemEnumField {
        ident: ident.to_string(),
        id: match id.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(nom::Err::Error(ParseError::from_error_kind(s, ErrorKind::Digit))),
        },
    }))
}

pub fn parse_message_field_standard(s: &str) -> IResult<&str, ItemMessageField> {

    let (s, comments1) = parse_field_comments_dockblock.parse(s)?;

    let (s, _) = multispace0.parse(s)?;
    let (s, modifier) = opt(alt((
        value(MessageFieldModifierCount::OPTIONAL, tag_no_case("optional")),
        value(MessageFieldModifierCount::REPEATED, tag_no_case("repeated")),
    ))).parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    let (s, comments2) = parse_field_comments_dockblock.parse(s)?;

    let (s, field_type) = alt((
        value(MessageFieldType::STRING, tag_no_case("string")),
        value(MessageFieldType::UINT64, tag_no_case("uint64")),
        value(MessageFieldType::UINT32, tag_no_case("uint32")),
        value(MessageFieldType::INT64, tag_no_case("int64")),
        value(MessageFieldType::INT32, tag_no_case("int32")),
        value(MessageFieldType::BOOL, tag_no_case("bool")),
        value(MessageFieldType::BYTES, tag_no_case("bytes")),
    )).parse(s)?;

    let (s, _) = multispace1.parse(s)?;
    let (s, comments2) = parse_field_comments_dockblock.parse(s)?;
    let (s, ident) = take_while1(|x| is_alphanumeric(x as u8)).parse(s)?;
    let (s, comments3) = parse_field_comments_dockblock.parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("=").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, comments4) = parse_field_comments_dockblock.parse(s)?;
    let (s, id) : (_, &str) = take_while1(|x| is_digit(x as u8)).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, comments4) = parse_field_comments_dockblock.parse(s)?;
    let (s, _) = tag(";").parse(s)?;
    let (s, comments4) = parse_field_comments_dockblock.parse(s)?;

    Ok((s, ItemMessageField::STANDARD(ItemMessageFieldStandard {
        modifier,
        field_type,
        ident: ident.to_string(),
        id: match id.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(nom::Err::Error(ParseError::from_error_kind(s, ErrorKind::Digit))),
        },
    })))
}

struct FieldComment {
    comment: String,
}

pub fn parse_message_field_oneof(s: &str) -> IResult<&str, ItemMessageField> {
    let (s, _) = multispace0.parse(s)?;
    let (s, buf) = tag("oneof").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, ident) = take_while1(|x| is_alphanumeric(x as u8)).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("{").parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, fields) = many0(parse_message_field_standard).parse(s)?;
    let (s, _) = multispace0.parse(s)?;
    let (s, _) = tag("}").parse(s)?;
    let (s, _) = multispace0.parse(s)?;

    Ok((s, ItemMessageField::ONEOF(ItemMessageFieldOneOf {
        ident: ident.to_string(),
        fields
    })))
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
    let (a, b) = parse("Message A {}Message B{}")?;
    assert_eq!(b.items.len(), 2);
    Ok(())
}

#[test]
fn test_parse_whitespace() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse("Message A { } Message B { } ")?;
    assert_eq!(b.items.len(), 2);
    Ok(())
}

#[test]
fn test_parse_enum() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse("enum A { FOO = 2; BAR = 3; } ")?;
    assert_eq!(b.items.len(), 1);
    Ok(())
}

#[test]
fn test_parse_multiline() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse(r#"
    Message
    Foo
    {
    }
    "#)?;
    assert_eq!(b.items.len(), 1);
    Ok(())
}

#[test]
fn test_parse_syntax() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse(r#"syntax = "proto3"; Message X { } "#)?;
    assert_eq!(b.syntax, Some("proto3".to_string()));
    Ok(())
}

#[test]
fn test_parse_oneof() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse(r#"
    Message MsgName {
         oneof testoneof {
            string name = 4;
            string submessage = 9;
         }
    }
    "#)?;

    assert_eq!(b.items.len(), 1);

    Ok(())
}

#[test]
fn test_parse_fields() -> Result<(), ::anyhow::Error> {
    let (a, b) = parse(r#"
    Message MsgName {
        optional string foo = 1;
        repeated uint64 foo2 = 2;
    }
    "#)?;

    assert_eq!(b.items.len(), 1);

    let msg = match &b.items[0] {
        Element::ItemMessage(m) => m,
        _ => panic!("invalid type"),
    };

    assert_eq!(msg.fields.len(), 2);
    assert_eq!(msg.fields[0], ItemMessageField::STANDARD(ItemMessageFieldStandard {
        modifier: Some(MessageFieldModifierCount::OPTIONAL),
        field_type: MessageFieldType::STRING,
        ident: "foo".to_string(),
        id: 1,
    }));


    Ok(())
}
