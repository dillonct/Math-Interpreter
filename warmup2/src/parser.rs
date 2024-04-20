use crate::tokenizer::{Token, Tokenizer};
use std::collections::HashMap;

pub struct Parser<'a> {
    token: Tokenizer<'a>,
    identifier_table: HashMap<String, i32>,

}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            token: Tokenizer::new(input),
            identifier_table: HashMap::new(), 
        }
    }

    pub fn parse_computation(&mut self) -> i32 {
        let computation = self.parse_expression();
        let test = self.token.peek_token();
        println!("{:?}", test);
        self.match_token(Token::EOC);

        computation
    }

    fn parse_expression(&mut self) -> i32 {
        let mut value = self.parse_term();
        loop {
            println!("sdfsfdsf");
            let token = self.token.peek_token();
            match token {
                Token::Plus => {
                    self.token.next_token();
                    value += self.parse_term();
                }
                Token::Minus => {
                    self.token.next_token();
                    value -= self.parse_term();
                }
                _ => {
                    break;
                }
            }
        }

        value
    }

    fn parse_term(&mut self) -> i32 {
        let mut value = self.parse_factor();
        loop {
            let token = self.token.peek_token();
            match token {
                Token::Times => {
                    self.token.next_token();
                    value *= self.parse_factor();
                }
                Token::Divide => {
                    self.token.next_token();
                    value /= self.parse_factor();
                }
                _ => {
                    break;
                }
            }
        }

        value
    }

    fn parse_factor(&mut self) -> i32 {
        let value: i32;

        let token = self.token.peek_token();

        match token {
            Token::Number(digits) => {
                value = digits as i32;
            },
            Token::Openpar => {
                self.token.next_token();
                value = self.parse_expression();
                self.match_token(Token::Closepar);
            },
            _ => {
                panic!("this shit dont work"); // fix this later
            },
        }

        value
    }

    fn match_token(&mut self, token_to_match: Token) {
        match self.token.peek_token() {
            token if token == token_to_match => (),
            _ => panic!("Does not close {:?}", token_to_match),
        }
    }

}




