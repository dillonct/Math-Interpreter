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
    Remainder,
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

        let ch = match self.peek_char() {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Times,
            '%' => Token::Remainder,
            '/' => Token::Divide,
            '(' => Token::Openpar,
            ')' => Token::Closepar,
            ';' => Token::Semicolon,
            '.' => Token::EOC,
            '<' => {
                self.next_char();
                if self.peek_char() == '-' {
                    Token::Assignment
                } else {
                    panic!("Assignment error");
                }
            },
            'a'..='z' | 'A'..='Z' => return self.get_identifier(),
            '0'..='9' => return self.get_number(),
            _ => panic!("Unexpected character to match token"),
        };

        self.next_char();
        ch
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



