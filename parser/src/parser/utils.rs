use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::{map, map_res, recognize};
use nom::error::Error;
use nom::multi::{count, many0};
use nom::Parser;
use nom::sequence::{pair, tuple};
use tsr_lexer::globals::{ByteResult, BytesSpan, Positioned, Span, TokenResult};
use tsr_lexer::token::{BuiltInType, Literal, Modifier, ReservedWord, Token};
use tsr_lexer::tokens::Tokens;
use tsr_lexer::util::complete_byte_slice_str_from_utf8;
use crate::tags::position;
//
//
// pub fn statement_indent<'a>(indent_count: usize) -> impl FnMut(Tokens<'a>) -> TokenResult<Positioned<Token>>{
//     //move |input: Tokens<'a>| recognize(count(alt((tag("    "), tag("\t"))), indent_count))(input)
//
//     map(
//         recognize(count(alt((tag("    "), tag("\t")),indent_count),|(start, result, end)| start.between(end).wrap(result),
// }
//
// pub fn statement_indent<'a>(input: Tokens) -> ByteResult<Positioned<Token>> {
//     map_res(
//         recognize(pair(
//             alt((alpha1, tag("_"))),
//             many0(alt((alphanumeric1, tag("_")))),
//         )),
//         |s: BytesSpan| {
//             let c = complete_byte_slice_str_from_utf8(s);
//             let s: Span = s.into();
//
//             c.map(|syntax| {
//                 s.wrap(match syntax {
//                     "public" => Token::Modifier(Modifier::Public),
//
//                     "false" => Token::Literal(Literal::Boolean(false)),
//
//                     _ => Token::Ident(syntax.to_string()),
//                 })
//             })
//         },
//     )(input)
// }
//
//
// pub fn positioned<'a, F, O1>(parser: F) -> impl FnMut(Tokens<'a>) -> TokenResult<'a, Positioned<O1>>
//     where
//         F: Parser<Tokens<'a>, O1, Error<Tokens<'a>>>,
// {
//     map(
//         tuple((position, parser, position)),
//         |(start, result, end)| start.between(end).wrap(result),
//     )
// }