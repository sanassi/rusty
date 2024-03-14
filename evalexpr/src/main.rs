use std::io;
use std::collections::VecDeque;

#[derive(PartialEq)]
#[derive(Debug)]
enum TokenKind {
    Plus,
    Minus,
    Times,
    Number,
    Divide,
    RParen,
    LParen
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    value: Option<i32>
}

impl TokenKind {
    fn precedence(&self) -> i32 {
        match self {
            TokenKind::RParen | TokenKind::LParen => 1,
            TokenKind::Times | TokenKind::Divide => 2,
            TokenKind::Plus | TokenKind::Minus => 4,
             _ => 0
        }
    } 
}

impl Token {
    fn new(kind: TokenKind, value: Option<i32>) -> Token {
        Token {
            kind: kind,
            value: value
        }
    }

    fn new_kind(kind: TokenKind) -> Token {
        Token::new(kind, None)
    }

    fn is_op(&self) -> bool {
        match self.kind {
            TokenKind::Plus|TokenKind::Minus|TokenKind::Times|TokenKind::Divide => true,
                _ => false
                
        }
    }
}

fn lex(input: &String) -> VecDeque<Token> {
    let mut q = VecDeque::<Token>::new();

    let mut i = 0;
    let bytes = input.as_bytes();
    while i < input.len() {
        match bytes[i] as char {
            '0'..='9' => {
                let mut s = String::new();
                while i < input.len() && (bytes[i] as char).is_digit(10){
                    s.push(bytes[i] as char);
                    i += 1;
                }
                q.push_back(Token::new(TokenKind::Number, Some(s.parse().unwrap())));
            },
            '+' => {
                q.push_back(Token::new_kind(TokenKind::Plus));
               i += 1;
            },
            '-' => {
                q.push_back(Token::new_kind(TokenKind::Minus));
                i += 1;
            },
            '*' => {
                q.push_back(Token::new_kind(TokenKind::Times));
                i += 1;
            },
            '/' => {
                q.push_back(Token::new_kind(TokenKind::Divide));
                i += 1;
            },
            '(' => {
                q.push_back(Token::new_kind(TokenKind::LParen));
                i += 1;
            },
            ')' => {
                q.push_back(Token::new_kind(TokenKind::RParen));
                i += 1;
            },
            _ => i += 1
        }
    }

    q
}

fn shunting_yard(q: &mut VecDeque<Token>) -> VecDeque<Token> {
    let mut rpn = VecDeque::<Token>::new();
    let mut op = VecDeque::<Token>::new();

    while !q.is_empty() {
        let token = q.pop_front().unwrap();
        if token.kind == TokenKind::Number {
            rpn.push_back(token);
        }
        else if token.is_op() {
            while !op.is_empty() && 
                op.back().unwrap().is_op() && 
                op.front().unwrap().kind != TokenKind::LParen && 
                op.back().unwrap().kind.precedence() < token.kind.precedence() {
                rpn.push_back(op.pop_back().unwrap());
            }
            op.push_back(token);
        }
        else if token.kind == TokenKind::LParen {
            op.push_back(token);
        }
        else if token.kind == TokenKind::RParen {
            while !op.is_empty() && op.back().unwrap().kind != TokenKind::LParen {
                assert!(!op.is_empty());
                rpn.push_back(op.pop_back().unwrap());
            }

            assert!(op.back().unwrap().kind == TokenKind::LParen);
            op.pop_back();
        }
    }

    while !op.is_empty() {
        rpn.push_back(op.pop_back().unwrap());
    }

    rpn
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let mut q = lex(&input);
    let mut rpn = shunting_yard(&mut q);
    while !rpn.is_empty() {
        println!("{:#?}", rpn.pop_back());
    }
}
