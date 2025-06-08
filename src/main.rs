use std::io;

fn main() -> io::Result<()>{
    println!("Question: ");
    let mut quest = String::new();
    io::stdin().read_line(&mut quest).expect("Failed to read line");

    let chars = split(quest);
    let tokens = parse(chars);

    println!("{:?}", tokens);
    Ok(())
}

fn split(s: String) -> Vec<char>{
    s.chars().collect()
}

#[derive(Debug)]
enum Token{
    Number(f64),
    Operator(char),
    LParen,
    RParen,
}

fn parse(chars: Vec<char>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut num_buf = String::new();

    for c in chars {
        if c.is_digit(10) || c == '.' {
            num_buf.push(c);
        } else if "+-*/^".contains(c) {
            if !num_buf.is_empty() {
                if let Ok(num) = num_buf.parse::<f64>() {
                    tokens.push(Token::Number(num));
                }
                num_buf.clear();
            }
            tokens.push(Token::Operator(c));
        } else if c == '(' {
            if !num_buf.is_empty() {
                if let Ok(num) = num_buf.parse::<f64>() {
                    tokens.push(Token::Number(num));
                }
                num_buf.clear();
            }
            tokens.push(Token::LParen);
        } else if c == ')' {
            if !num_buf.is_empty() {
                if let Ok(num) = num_buf.parse::<f64>() {
                    tokens.push(Token::Number(num));
                }
                num_buf.clear();
            }
            tokens.push(Token::RParen);
        } else if c.is_whitespace() {
            continue;
        } else {
            panic!("Unexpected character: {}", c);
        }
    }

    if !num_buf.is_empty() {
        if let Ok(num) = num_buf.parse::<f64>() {
            tokens.push(Token::Number(num));
        }
    }

    tokens
}
