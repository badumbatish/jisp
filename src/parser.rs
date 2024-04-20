use crate::lexer;
use crate::lexer::Token;
use crate::object::Object;
use std::fmt;
pub struct ParseError {
    err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser error: {}", self.err)
    }
}

pub fn parse(program: &str) -> Result<Object, ParseError> {
    let token_result = lexer::tokenize(program);

    let mut tokens = token_result.into_iter().rev().collect::<Vec<_>>();
    let parsed_list = parse_list(&mut tokens)?;
    Ok(parsed_list)
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", token),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err(ParseError {
                err: format!("Did not find enough tokens"),
            });
        }
        let t = token.unwrap();
        // match t {
        //     Token::Keyword(k) => list.push(Object::Keyword(k)),
        //     Token::If => list.push(Object::If),
        //     Token::BinaryOp(b) => list.push(Object::BinaryOp(b)),
        //     Token::Integer(n) => list.push(Object::Integer(n)),
        //     Token::Float(f) => list.push(Object::Float(f)),
        //     Token::String(s) => list.push(Object::String(s)),
        //     Token::Symbol(s) => list.push(Object::Symbol(s)),
        //     Token::LParen => {
        //         tokens.push(Token::LParen);
        //         let sub_list = parse_list(tokens)?;
        //         list.push(sub_list);
        //     }
        //     Token::RParen => {
        //         return Ok(Object::List(Rc::new(list)));
        //     }
        // }
    }

    Ok(Object::Integer(1))
}
