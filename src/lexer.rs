use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
pub enum Token {
    Num(f64),
    Symbol(String),
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let program_input = input.replace("(", " ( ").replace(")", " ) ");

    let words = program_input.split_whitespace();

    let mut tokens: Vec<Token> = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let i = parse_num(word);
                if i.is_ok() {
                    tokens.push(Token::Num(i.unwrap()));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
                }
            }
        }
    }

    tokens
}

pub fn parse_num(num_str: &str) -> Result<f64, ParseFloatError> {
    num_str.parse::<f64>()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "(+ 1 2)";
        let expected = vec![
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Num(1 as f64),
            Token::Num(2 as f64),
            Token::RParen,
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_2() {
        let input = "(+ (+ 1 2) 2)";
        let expected = vec![
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Num(1.0),
            Token::Num(2.0),
            Token::RParen,
            Token::Num(2.0),
            Token::RParen,
        ];
        assert_eq!(tokenize(input), expected);
    }
}
