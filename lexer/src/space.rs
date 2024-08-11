use super::token::{Delimiter, Space, Token};
use crate::{
    globals::{ByteResult, BytesSpan, Positioned},
    syntax,
};
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::character::complete::{line_ending, multispace0, newline};
use nom::character::streaming::not_line_ending;
use nom::combinator::{map, recognize};
use nom::error::Error;
use nom::multi::count;
use nom::Parser;
use nom::sequence::tuple;

use crate::globals::{Span, TokenResult};
use crate::token::Space::EndEnter1;
use crate::tokens::Tokens;

syntax! {
   //four_space: "    " => Token::Space(Space::FourSpace);
   one_tab: "\\t" => Token::Space(Space::OneTab);
   end_enter: "\n" => Token::Space(Space::EndEnter);
   end_enter1: "\r\n" => Token::Space(Space::EndEnter1);

}

pub fn position(input: Tokens) -> TokenResult<Span> {
    let (_, pos) = take(1usize)(input)?;

    Ok((
        input,
        pos.tok.first().map(|token| token.span).unwrap_or_default(),
    ))
}


pub fn positioned<'a, F, O1>(parser: F) -> impl FnMut(Tokens<'a>) -> TokenResult<'a, Positioned<O1>>
    where
        F: Parser<Tokens<'a>, O1, Error<Tokens<'a>>>,
{
    map(
        tuple((position, parser, position)),
        |(start, result, end)| start.between(end).wrap(result),
    )
}
pub fn four_space(input: BytesSpan) -> ByteResult<Positioned<Token>> {
     let (input, position) = recognize(count(alt((tag("    "), tag("\t"))), 4))(input)?;
        println!("====={:?}",input);
    Ok((input, Positioned::new(Token::Space(Space::FourSpace), position.into())))

}
// pub fn lex_end_line(input: BytesSpan) -> ByteResult<Positioned<Token>> {
//
//     positioned(map(positioned(line_ending), |s| Token::Space(Space::EndLine)))
// }

pub fn lex_space(input: BytesSpan) -> ByteResult<Positioned<Token>> {
    alt((
        four_space,
        one_tab,
        // end_enter,
        // end_enter1,
    ))(input)
}
