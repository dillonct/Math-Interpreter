#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Computation,
    Variable,
    Plus,
    Minus,
    Times,
    Divide,
    Assignment,
    Openpar,
    Closepar,
    Semicolon,
    EOC,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Tokenizer<'a> {
    // create an instance
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let c = self.peek_char();
        match c {
            '+' => { self.next_char(); Token::Plus },
            '-' => { self.next_char(); Token::Minus },
            '*' => { self.next_char(); Token::Times },
            '/' => { self.next_char(); Token::Divide },
            '(' => { self.next_char(); Token::Openpar },
            ')' => { self.next_char(); Token::Closepar },
            ';' => { self.next_char(); Token::Semicolon },
            '.' => { self.next_char(); Token::EOC },
            '<' => {
                self.next_char();
                if self.peek_char() == '-' {
                    Token::Assignment
                } else {
                    panic!("Assignment error");
                }
            },
            _ if c.is_alphabetic() => self.get_identifier(),
            _ if c.is_numeric() => self.get_number(),
            _ => panic!("Unexpected character: {}", c),
        }

    }

    pub fn peek_token(&mut self) -> Token {
        let previous_index = self.index;
        let token = self.next_token();
        self.index = previous_index;

        token
    }

    fn peek_char(&self) -> char {
        self.input.chars().nth(self.index).unwrap_or('\0')
    }
    
    fn next_char(&mut self) -> char {
        let c = self.input.chars().nth(self.index).unwrap_or('\0');
        self.index += 1;

        c 
    }

    fn consume_identifier(&mut self) -> String {
        let mut string = String::new();
        loop {
            let c = self.input.chars().nth(self.index).unwrap_or('\0');
            if c == ' ' || c == '.' || c == '\0' || !c.is_alphanumeric() {
                break;
            } 

            string.push(c);
            self.index += 1;
        }

        self.skip_whitespace();

        string
    }

    fn consume_number(&mut self) -> String {
        let mut string = String::new();
        loop {
            let c = self.input.chars().nth(self.index).unwrap_or('\0');
            if c == ' ' || c == '.' || c == '\0' || !c.is_numeric() {
                break;
            } 

            string.push(c);
            self.index += 1;
        }

        self.skip_whitespace();

        string
    }
    fn get_identifier(&mut self) -> Token {
        let name = self.consume_identifier();
        match name.as_str() {
            "var" => Token::Variable,
            "computation" => Token::Computation,
            _ => Token::Identifier(name),
        }
    }
    
    fn get_number(&mut self) -> Token {
        let value = self.consume_number();
        
        Token::Number(value.parse::<i32>().unwrap())
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek_char();
            match c {
                ' ' => self.index += 1,
                _ => break,
            }
        }
    }
}


