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
        // not really useful but prettier than using multiple ifs :\
        match self.kind {
            TokenKind::Plus|TokenKind::Minus => true,
            TokenKind::Times|TokenKind::Divide => true,
            _ => false
        }
    }
}

/**
 * TODO: refactor this ugly code
 */
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
                op.back().unwrap().kind.precedence() <= token.kind.precedence() {
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

fn evaluate_rpn(rpn: &mut VecDeque<Token>) -> i32 {
    let mut tmp = VecDeque::<Token>::new();
    while !rpn.is_empty() {
        let token = rpn.pop_front().unwrap();
        if token.is_op() {
            let y = tmp.pop_front().unwrap();
            let x = tmp.pop_front().unwrap();
            tmp.push_front(
                Token::new(
                    TokenKind::Number,
                    Some(
                        match token.kind {
                            TokenKind::Plus => y.value.unwrap() + x.value.unwrap(),
                            TokenKind::Minus => y.value.unwrap() - x.value.unwrap(),
                            TokenKind::Times => y.value.unwrap() * x.value.unwrap(),
                            TokenKind::Divide => y.value.unwrap() / x.value.unwrap(),
                            _ => 0
                        }
                    )
                )
            );
        }
        else {
            tmp.push_front(token);
        }
    }

    tmp.pop_back().unwrap().value.unwrap()
}

pub fn evaluate(input: &String) -> i32 {
    let mut q = lex(&input);
    let mut rpn = shunting_yard(&mut q);
    evaluate_rpn(&mut rpn)
}
