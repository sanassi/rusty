use std::io;
use std::collections::VecDeque;

#[derive(Debug)]
enum TokenKind {
    Plus,
    Minus,
    Times,
    Number
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    value: Option<i32>
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
                q.push_back(Token {
                    kind: TokenKind::Number,
                    value: Some(s.parse().unwrap())
                });
            },
            _ => i += 1
        }
    }

    q
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let mut q = lex(&input);
    while !q.is_empty() {
        let val = q.pop_front();
        println!("{:#?}", val);
    }
}
