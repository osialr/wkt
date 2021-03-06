// Copyright 2014-2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::iter::Peekable;
use std::str;

#[derive(PartialEq, Debug)]
pub enum Token {
    Comma,
    Number(f64),
    ParenClose,
    ParenOpen,
    Word(String),
}

fn is_whitespace(c: char) -> bool {
    match c {
        '\n' | '\r' | '\t' | ' ' => true,
        _ => false,
    }
}

fn is_numberlike(c: char) -> bool {
    match c {
        c if c.is_numeric() => true,
        '.' | '-' | '+' => true,
        _ => false,
    }
}

pub type PeekableTokens<'a> = Peekable<Tokens<'a>>;

pub struct Tokens<'a> {
    chars: Peekable<str::Chars<'a>>,
}

impl<'a> Tokens<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Tokens {
            chars: input.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // TODO: should this return Result?
        let next_char = self.chars.next()?;

        match next_char {
            '\0' => None,
            '(' => Some(Token::ParenOpen),
            ')' => Some(Token::ParenClose),
            ',' => Some(Token::Comma),
            c if is_whitespace(c) => self.next(),
            c if is_numberlike(c) => {
                let mut number = c.to_string() + &self.read_until_whitespace().unwrap_or_default();
                match number.trim_left_matches('+').parse::<f64>() {
                    Ok(parsed_num) => Some(Token::Number(parsed_num)),
                    Err(e) => panic!("Could not parse number: {}", e),
                }
            }
            c => {
                let word = c.to_string() + &self.read_until_whitespace().unwrap_or_default();
                Some(Token::Word(word))
            }
        }
    }
}

impl<'a> Tokens<'a> {
    fn read_until_whitespace(&mut self) -> Option<String> {
        let next_char = *self.chars.peek()?;

        match next_char {
            '\0' | '(' | ')' | ',' => None,
            c if is_whitespace(c) => {
                let _ = self.chars.next();
                None
            }
            _ => Some(
                self.chars.next().unwrap().to_string()
                    + &self.read_until_whitespace().unwrap_or_default(),
            ),
        }
    }
}

#[test]
fn test_tokenizer_empty() {
    let test_str = "";
    let tokens: Vec<Token> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![]);
}

#[test]
fn test_tokenizer_1word() {
    let test_str = "hello";
    let tokens: Vec<Token> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![Token::Word("hello".to_string())]);
}

#[test]
fn test_tokenizer_2words() {
    let test_str = "hello world";
    let tokens: Vec<Token> = Tokens::from_str(test_str).collect();
    assert_eq!(
        tokens,
        vec![
            Token::Word("hello".to_string()),
            Token::Word("world".to_string()),
        ]
    );
}

#[test]
fn test_tokenizer_1number() {
    let test_str = "4.2";
    let tokens: Vec<Token> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![Token::Number(4.2)]);
}

#[test]
fn test_tokenizer_1number_plus() {
    let test_str = "+4.2";
    let tokens: Vec<Token> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![Token::Number(4.2)]);
}

#[test]
fn test_tokenizer_2numbers() {
    let test_str = ".4 -2";
    let tokens: Vec<Token> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![Token::Number(0.4), Token::Number(-2.0)]);
}

#[test]
fn test_tokenizer_point() {
    let test_str = "POINT (10 -20)";
    let tokens: Vec<Token> = Tokens::from_str(test_str).collect();
    assert_eq!(
        tokens,
        vec![
            Token::Word("POINT".to_string()),
            Token::ParenOpen,
            Token::Number(10.0),
            Token::Number(-20.0),
            Token::ParenClose,
        ]
    );
}
