// use std::{fs, io};
//
// use tsr_lexer::Lexer;
//
// #[test]
// fn main() -> io::Result<()> {
//     let input = fs::read_to_string("main.tsx")?;
//     let code = input.as_bytes();
//
//     let (_, tokens) = Lexer::lex_tokens(code.into()).unwrap();
//
//     println!(
//         "[\n{}\n]",
//         tokens
//             .iter()
//             .map(|token| format!("  {token:?}"))
//             .collect::<Vec<_>>()
//             .join(",\n")
//     );
//
//     Ok(())
// }

use nom::{
    IResult,
    bytes::complete::{tag, is_not},
    character::complete::{multispace1, char},
    combinator::map,
    sequence::delimited,
    multi::many0,
};
use nom::character::complete::{multispace0, space0, space1};

#[derive(Debug)]
pub struct Token {
    pub indent_level: usize,
    pub value: String,
}

fn parse_line(input: &str) -> IResult<&str, Token> {
    let (input, indent) = multispace0(input)?;
    let indent_level = indent.len(); // 计算缩进级别
    let (input, value) = is_not("\n")(input)?;
    let (input, _) = char('\n')(input)?; // 消耗换行符

    Ok((input, Token {
        indent_level,
        value: value.to_string(),
    }))
}

fn parse(input: &str) -> IResult<&str, Vec<Token>> {
    many0(parse_line)(input)
}
#[test]
fn main() {
    let input = "    line 1\n  line 2\nline 3\n";
    match parse(input) {
        Ok((remaining, tokens)) => {
            println!("Parsed tokens: {:?}", tokens);
            println!("Remaining input: {:?}", remaining);
        },
        Err(err) => {
            eprintln!("Error parsing input: {:?}", err);
        }
    }
}
