use nom::bytes::complete::tag;
use nom::{complete, IResult, Parser};
use nom::character::complete::space0;


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
    let (s, _) = tag("}").parse(s)?;
    let (s, _) = space0.parse(s)?;

    Ok((s, Box::new(Message {})))
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
