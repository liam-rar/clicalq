use std::io;

fn main() -> io::Result<()>{
    println!("Question: ");
    let mut quest = String::new();
    io::stdin().read_line(&mut quest).expect("Failed to read line");

    let chars = split(quest);
    let tokens = parse(chars);
    println!("{:?}", tokens);

    let rpn = to_rpn(tokens);
    println!("RPN: {:?}", rpn);

    let result = eval_rpn(rpn);
    println!("Result: {}", result);

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
    let mut last_token_was_operator_or_lparen = true; // Assume start of input is like after '('

    let mut chars_iter = chars.into_iter().peekable();

    while let Some(c) = chars_iter.next() {
        if c.is_digit(10) || c == '.' {
            num_buf.push(c);
            last_token_was_operator_or_lparen = false;
        } else if c == '-' {
            if last_token_was_operator_or_lparen {
                // Treat as unary minus
                num_buf.push(c);
            } else {
                // Treat as binary subtraction
                if !num_buf.is_empty() {
                    if let Ok(num) = num_buf.parse::<f64>() {
                        tokens.push(Token::Number(num));
                    }
                    num_buf.clear();
                }
                tokens.push(Token::Operator(c));
                last_token_was_operator_or_lparen = true;
            }
        } else if "+*/^".contains(c) {
            if !num_buf.is_empty() {
                if let Ok(num) = num_buf.parse::<f64>() {
                    tokens.push(Token::Number(num));
                }
                num_buf.clear();
            }
            tokens.push(Token::Operator(c));
            last_token_was_operator_or_lparen = true;
        } else if c == '(' {
            tokens.push(Token::LParen);
            last_token_was_operator_or_lparen = true;
        } else if c == ')' {
            if !num_buf.is_empty() {
                if let Ok(num) = num_buf.parse::<f64>() {
                    tokens.push(Token::Number(num));
                }
                num_buf.clear();
            }
            tokens.push(Token::RParen);
            last_token_was_operator_or_lparen = false;
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

fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut op_stack: Vec<Token> = Vec::new();

    fn precedence(op: char) -> u8 {
        match op {
            '^' => 3,
            '*' | '/' => 2,
            '+' | '-' => 1,
            _ => 0,
        }
    }

    fn is_right_associative(op: char) -> bool {
        op == '^'
    }

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Operator(op1) => {
                while let Some(Token::Operator(op2)) = op_stack.last() {
                    if (precedence(*op2) > precedence(op1)) ||
                       (precedence(*op2) == precedence(op1) && !is_right_associative(op1)) {
                        output.push(op_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                op_stack.push(Token::Operator(op1));
            }
            Token::LParen => op_stack.push(Token::LParen),
            Token::RParen => {
                while let Some(top) = op_stack.pop() {
                    if let Token::LParen = top {
                        break;
                    } else {
                        output.push(top);
                    }
                }
            }
        }
    }

    while let Some(tok) = op_stack.pop() {
        output.push(tok);
    }

    output
}

fn eval_rpn(tokens: Vec<Token>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(n),
            Token::Operator(op) => {
                let b = stack.pop().expect("Missing operand");
                let a = stack.pop().expect("Missing operand");
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    '^' => a.powf(b),
                    _ => panic!("Unknown operator {}", op),
                };
                stack.push(result);
            }
            _ => panic!("Unexpected token in RPN"),
        }
    }

    stack.pop().expect("No result on stack")
}
