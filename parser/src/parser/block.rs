use crate::{ast::FunctionDeclaration, Parser, tags::{async_tag, function_tag, positioned}};

use nom::{combinator::{map, opt, value}, IResult, sequence::{preceded, tuple}};
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while1};
use nom::character::complete::{line_ending, space0, space1};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, terminated};
use nom_locate::position;

use tsr_lexer::{globals::{Positioned, TokenResult}, lex_comment, token::Modifier, tokens::Tokens};
use tsr_lexer::delimiters::lex_delimiter;
use tsr_lexer::globals::{ByteResult, BytesSpan};
use tsr_lexer::literal::lex_literal;
use tsr_lexer::operators::lex_operator;
use tsr_lexer::punctuation::lex_punctuation;
use tsr_lexer::reserved::lex_reserved_ident;
use tsr_lexer::space::{end_enter1, four_space, lex_space};
use tsr_lexer::state::AstState;
use tsr_lexer::token::{Punctuation, Token};
use crate::ast::Block;
use crate::parsing::{parse_code_block, parse_ident};
use crate::parsing::signatures::parse_call_signature;
use crate::parsing::statement::expression::parse_expression;
use crate::parsing::statement::parse_statement;
use crate::tags::{brace_close_tag, brace_open_tag, end_enter1_tag, end_enter_tag, fat_arrow_tag, four_space_tag, return_tag, semi_tag};


fn parse_indented_line(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("    "), // 确保有至少一个缩进（4个空格或1个制表符）
        take_while1(|c| c != '\n'),
    )(input)
}

impl Parser {


    pub fn parse_pine_code_block<'a>(&'a self, input: Tokens<'a>) -> TokenResult<Block> {
        let (input, start) = crate::tags::position(input)?;

        let (tokens, token) = take(1usize)(input)?;

        let token = &token.tok[0];
        match token.value {
            Token::Punctuation(Punctuation::FatArrow)=>{
                let (input, value) = preceded(
                    fat_arrow_tag,
                    many0(terminated(parse_statement, opt(semi_tag))),

                )(input)?;
                let (input, _) = opt(preceded(return_tag,alt((end_enter_tag,end_enter1_tag))))(input)?;

               // let (input, _) = opt(preceded(return_tag, line_ending))(input.)?;

                let (input, end) = crate::tags::position(input)?;
                println!("map:{:?}",input);
                Ok((input, start.between(end).wrap(value)))
            }
            _ => {

                let (input, value) = delimited(
                    brace_open_tag,
                    many0(terminated(parse_statement, opt(semi_tag))),
                    brace_close_tag,
                )(input)?;
                let (input, end) = crate::tags::position(input)?;
                println!("map:{:?}",input);
                Ok((input, start.between(end).wrap(value)))

            }
        }




    }
}