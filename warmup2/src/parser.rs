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
        self.match_token(Token::Computation);
        self.parse_variables();
        // let computation = self.parse_expression();
        let computation = self.parse_multiple_expressions();
        //self.match_token(Token::EOC);

        computation
    }

    fn parse_multiple_expressions(&mut self) -> i32 {
        let mut value = self.parse_expression();
        println!("{}", value);

        loop {
            match self.token.peek_token() {
                Token::Semicolon => { 
                    self.token.next_token();
                    value = self.parse_expression();
                    println!("{}", value);
                },
                _ => break,
            }

        }

        self.match_token(Token::EOC);

        value
    }
    

    fn parse_variables(&mut self) {
        loop {
            match self.token.peek_token() {
                Token::Variable => { self.token.next_token(); self.parse_assignment() },
                _ => break,
            }
        }
    }

    fn parse_assignment(&mut self) {
        match self.token.next_token() {
            Token::Identifier(string) => {
                self.match_token(Token::Assignment);
                let value = self.parse_expression();
                self.identifier_table.insert(string, value);
                self.match_token(Token::Semicolon);
            },
            _ => panic!("Expected an identifier"),
        }
    }


    fn parse_expression(&mut self) -> i32 {
        let mut value = self.parse_term();
        loop {
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
        
        let token1 = self.token.peek_token();

        match self.token.peek_token() {
            Token::Identifier(name) => {
                self.token.next_token();
                value = *self.identifier_table.get(&name).expect("Variable does not exist");
            },
            Token::Number(digits) => {
                self.token.next_token();
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
        match self.token.next_token() {
            token if token == token_to_match => (),
            _ => panic!("Does not match (open/close) {:?}", token_to_match),
        }


    }

}




