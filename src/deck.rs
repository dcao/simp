//! Parsing text files into a list of slides.

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::char,
    combinator::opt,
    multi::separated_list0,
    IResult,
};

/// A Slide consists of either some text or a path to an image file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Slide {
    Text(String),
    Image(String),
}

/// A Deck is just a list of Slides.
pub type Deck = Vec<Slide>;

/// Parse a Slide
pub fn parse_slide(input: &str) -> IResult<&str, Slide> {
    let (input, i) = opt(char('@'))(input)?;
    let (input, f) = take_until("\n\n")(input)?;

    Ok((
        input,
        if i.is_some() {
            Slide::Image(f.to_string())
        } else {
            Slide::Text(f.to_string())
        },
    ))
}

/// Parse a Deck
pub fn parse_deck(input: &str) -> IResult<&str, Deck> {
    separated_list0(tag("\n\n"), parse_slide)(input)
}
