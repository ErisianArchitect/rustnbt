#![doc = r#"
This module is for parsing and generating SNBT. This module will only cover
Minecraft SNBT, and will not cover the Tag type extensions.

| Tag Type        | Syntax                                                                      |
|-----------------|-----------------------------------------------------------------------------|
|[Tag::Byte]      | `<number>b` or `<number>B`
|[Tag::Short]     | `<number>s` or `<number>S`
|[Tag::Int]       | `<integer_number>`
|[Tag::Long]      | `<number>l` or `<number>L`
|[Tag::Float]     | `<number>f` or `<number>F`
|[Tag::Double]    | `<decimal_number>`, `<number>d` or `<number>D`
|[Tag::ByteArray] | `[B; 0b, 1b, 2b]`
|[Tag::String]    | `"<text>"` or `'<text>'` or <identifier>
|[Tag::List]      | `[<tag>...]`
|[Tag::Compound]  | `{ <string:name> : <tag> }`
|[Tag::IntArray]  | `[I; 0, 1, 2 3]`
|[Tag::LongArray] | `[L; 0, 1, 2l, 3L]`

Note: Identifiers can include the following characters: [a-zA-Z0-9+-._].
For [Tag::List], the tag type for the list is determined by the type of the first tag.
"#]

use crate::*;
use crate::tag::*;
use chumsky::prelude::*;
use chumsky::primitive::{
    Container,
    OneOf,
    NoneOf,
};
use chumsky::Error;
use chumsky::text::Character;
use core::panic;
use std::collections::HashSet;
use std::result;

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Found invalid token(s).")]
    TokenizeError(Vec<Simple<char>>),
    #[error("Failed to parse SNBT.")]
    ParseFailure(Vec<Simple<Token>>),
}

fn no_case<C: Container<char>>(chars: C) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    chars.get_iter().for_each(|c| {
        set.insert(c);
        if c.is_lowercase() {
            set.extend(c.to_uppercase());
        } else {
            set.extend(c.to_lowercase());
        }
    });
    set
}

fn one_of_nc<C: Container<char>, E: Error<char>>(chars: C) -> OneOf<char,HashSet<char>,E> {
    one_of(no_case(chars))
}

fn none_of_nc<C: Container<char>, E: Error<char>>(chars: C) -> NoneOf<char,HashSet<char>,E> {
    none_of(no_case(chars))
}

#[derive(PartialEq, Eq,PartialOrd, Ord, Clone, Hash, Debug)]
pub enum ArrayType {
    Byte = 'B' as isize,
    Int = 'I' as isize,
    Long = 'L' as isize,
}

#[derive(PartialEq, Eq,PartialOrd, Ord, Clone, Hash, Debug)]
pub enum IntegerType {
    Byte,
    Int,
    Short,
    Long,
}

#[derive(PartialEq, Eq,PartialOrd, Ord, Clone, Hash, Debug)]
pub enum DecimalType {
    Float,
    Double,
}

impl From<char> for DecimalType {
    /// Must be either 'D' or 'F' (case-insensitive) or else it will panic.
    fn from(ch: char) -> Self {
        match ch.to_ascii_lowercase() {
            'f' => DecimalType::Float,
            'd' => DecimalType::Double,
            _ => panic!(),
        }
    }
}

impl From<char> for IntegerType {
    /// Must be one of 'B', 'S', or 'L' (case-insensitive) or else it will panic.
    fn from(ch: char) -> Self {
        match ch.to_ascii_lowercase() {
            'b' => IntegerType::Byte,
            's' => IntegerType::Short,
            'l' => IntegerType::Long,
            _ => panic!(),
        }
    }
}

impl From<char> for ArrayType {
    fn from(ch: char) -> Self {
        match ch {
            'B' => ArrayType::Byte,
            'I' => ArrayType::Int,
            'L' => ArrayType::Long,
            _ => panic!("Expected either 'B', 'I', or 'L'."),
        }
    }
}

#[derive(PartialEq, Eq,PartialOrd, Ord, Clone, Hash, Debug)]
pub enum Token {
    Comma,
    Colon,
    ArrayStart(ArrayType),
    OpenBracket,
    OpenBrace,
    CloseBrace,
    CloseBracket,
    Boolean(bool),
    Integer(String, IntegerType),
    Decimal(String, DecimalType),
    Identifier(String),
    StringLiteral(String),
}

macro_rules! token_api {
    ($($name:ident => $block:block)+) => {
        impl Token {
            $(
                pub fn $name() -> impl Parser<char, Token, Error = Simple<char>>
                $block
            )+

            pub fn parse<S: AsRef<str>>(source: S) -> Result<Vec<Token>, Vec<Simple<char>>> {
                choice((
                    $(
                        Self::$name().padded(),
                    )+
                ))
                .repeated().at_least(1)
                .collect::<Vec<Token>>()
                .parse(source.as_ref())
            }
        }
    };
}

token_api!{
    comma => { just(',').to(Token::Comma) }
    colon => { just(':').to(Token::Colon) }
    array_start => {
        just('[')
            .ignore_then(one_of_nc("bil"))
            .then_ignore(just(';'))
            .map(ArrayType::from)
            .map(Token::ArrayStart)
    }
    open_bracket => { just('[').to(Token::OpenBracket) }
    open_brace => { just('{').to(Token::OpenBrace) }
    close_brace => { just('}').to(Token::CloseBrace) }
    close_bracket => { just(']').to(Token::CloseBracket) }
    boolean => {
        choice((
            just("true").to(Token::Boolean(true)),
            just("false").to(Token::Boolean(false)),
        ))
    }
    integer => {
        just::<char, _, Simple<char>>('-')
            .or_not()
            .chain::<char, _, _>(text::int(10))
            .collect::<String>()
            .then(
                choice((
                    one_of_nc("bil").map(IntegerType::from),
                    none_of_nc("bil").rewind().to(IntegerType::Int),
                ))
            )
            .map(|(int_text, int_type)| Token::Integer(int_text, int_type))
    }
    decimal => {
        just::<char, _, Simple<char>>('-').or_not()
            .chain::<char,_,_>(text::int(10))
            .chain::<char,_,_>(just('.'))
            .chain::<char,_,_>(text::digits(10))
            .collect::<String>()
            .then(
                choice((
                    one_of_nc("df").map(DecimalType::from),
                    none_of_nc("df").rewind().to(DecimalType::Double)
                ))
            )
            .map(|(dec_str, dec_type)| Token::Decimal(dec_str, dec_type))
    }
    identifier => {
        choice::<_,Simple<char>>((
            filter(char::is_ascii_alphanumeric),
            one_of("+-_.")
        ))
        .repeated().at_least(1)
        .collect::<String>()
        .map(Token::Identifier)
    }
    string_literal => {
        let escape = just::<_,_,Simple<char>>('\\').ignore_then(
            just('\\')
                .or(just('/'))
                .or(just('"'))
                .or(just('\''))
                .or(just('b').to('\x08'))
                .or(just('f').to('\x0C'))
                .or(just('n').to('\n'))
                .or(just('r').to('\r'))
                .or(just('t').to('\t'))
        );
        Token::identifier().or(
            choice::<_,Simple<char>>((
                just('"')
                    .ignore_then(
                        none_of("\\\"").or(escape.clone()).repeated()
                    )
                    .then_ignore(just('"'))
                    .collect::<String>(),
                just('\'')
                    .ignore_then(
                        none_of("\\'").or(escape.clone()).repeated()
                    )
                    .then_ignore(just('\''))
                    .collect::<String>(),
            )).map(Token::StringLiteral))
    }
}

fn parser() -> impl Parser<Token, Tag, Error = Simple<Token>> {
    // mapped to String
    let ident = filter(|token| matches!(token, Token::Identifier(_) | Token::StringLiteral(_) | Token::Boolean(_)))
        .map(|token| {
            match token {
                Token::Identifier(text) => text,
                Token::StringLiteral(text) => text,
                Token::Boolean(on) => String::from(if on { "true" } else { "false" }),
                _ => unreachable!(),
            }
        });
    macro_rules! int_parser {
        (let $name:ident = $type:ident) => {
            
        };
    }
    let byte = filter(|token| matches!(token, Token::Integer(_, IntegerType::Byte)))
        .map(|token| {
            Tag::Byte(match token {
                Token::Integer(num_str, IntegerType::Byte) => i8::from_str_radix(&num_str, 10).unwrap_or_default(),
                _ => unreachable!(),
            })
        });
    // let short = filter
    
    // recursive(|compound| {

    // })
    todo!()
}

pub fn parse<S: AsRef<str>>(source: S) -> Result<Tag, ParseError> {
    match Token::parse(source) {
        Ok(tokens) => {
            match parser().parse(tokens) {
                Ok(tag) => Ok(tag),
                Err(errors) => Err(ParseError::ParseFailure(errors)),
            }
        },
        Err(errors) => Err(ParseError::TokenizeError(errors)),
    }
}

#[test]
fn foo() {
    // [warning]: DELETE ME!
    let result = Token::parse(r#"
{
    test : "Hello, world!\nThis is a test.",
    hello.world+test : 1b,
    True : true,
    "test" : [B; 0b, true, false, -1b]
}
    "#);
    if let Ok(result) = result {
        result.iter().for_each(|v| println!("Token: {:#?}", v));
    }
}