use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Reference(String),
    Identifier(String),
    Keyword(String),
    Connector,
    Number(String),
    Note(String),
    Colon,
    Comment(String),
    String(String),
    Symbol(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenType,
    position: Position,
}

pub fn tokenize(input: &str) -> Result<Vec<(TokenType, Position)>> {
    let keywords = vec!["sin", "meta", "sampler", "seq", "lpf", "range"];
    let mut tokens = Vec::new();
    let lines = input.lines().enumerate();

    for (line_number, line) in lines {
        let line = line.trim();
        println!("line {:?}", line);
        if line.is_empty() {
            continue;
        }
        if line.starts_with(r#"//"#) {
            tokens.push((
                TokenType::Comment(line[2..].to_string()),
                Position {
                    line: line_number,
                    column: 0,
                },
            ));
            continue;
        }
        let mut chars = line.chars().peekable();
        let mut char_number = 0;

        while let Some(&c) = chars.peek() {
            // println!("char {}", &c);
            if c.is_whitespace() {
                chars.next();
                char_number += 1;
            } else if c.is_alphabetic() {
                let mut operator = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() {
                        operator.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if keywords.contains(&&operator[..]) {
                    tokens.push((
                        TokenType::Keyword(operator),
                        Position {
                            line: line_number,
                            column: char_number,
                        },
                    ));
                } else {
                    tokens.push((
                        TokenType::Identifier(operator),
                        Position {
                            line: line_number,
                            column: char_number,
                        },
                    ));
                }
            // } else if c.is_numeric() || c == '_' {
            //     let mut note = String::new();
            //     while let Some(&c) = chars.peek() {
            //         if c.is_numeric() || c == '_' {
            //             note.push(chars.next().unwrap());
            //         } else {
            //             break;
            //         }
            //     }
            //     tokens.push(TokenType::Note(note));
            } else if c.is_numeric() || c == '.' || c == '_' {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() || c == '.' || c == '_' {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push((
                    TokenType::Number(number),
                    Position {
                        line: line_number,
                        column: char_number,
                    },
                ));
            } else if c == '~' {
                let mut identifier = String::new();
                identifier.push(chars.next().unwrap());
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push((
                    TokenType::Identifier(identifier),
                    Position {
                        line: line_number,
                        column: char_number,
                    },
                ));
            } else if c == '\\' {
                chars.next();
                let mut symbol = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        symbol.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push((
                    TokenType::Symbol(symbol),
                    Position {
                        line: line_number,
                        column: char_number,
                    },
                ));
            } else if c == ':' {
                chars.next();
                tokens.push((
                    TokenType::Colon,
                    Position {
                        line: line_number,
                        column: char_number,
                    },
                ));
            } else if c == '>' {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '>' {
                        chars.next();
                        tokens.push((
                            TokenType::Connector,
                            Position {
                                line: line_number,
                                column: char_number,
                            },
                        ));
                    } else {
                        bail!(format!(
                            "The connector should be >>, but get {} at {}:{}'",
                            c, line_number, char_number
                        ));
                    }
                }
            } else if c == '"' {
                chars.next();
                let mut string = String::new();
                while let Some(&c) = chars.peek() {
                    if c != '"' {
                        string.push(chars.next().unwrap());
                    } else {
                        chars.next();
                        break;
                    }
                }
                tokens.push((
                    TokenType::String(string),
                    Position {
                        line: line_number,
                        column: char_number,
                    },
                ));
            } else {
                let err = format!(
                    "Unexpected character '{} at {}:{}'",
                    c, line_number, char_number
                );
                bail!(err);
            }
        }
    }
    Ok(tokens)
}

use std::collections::HashMap;

#[derive(Debug)]
pub enum ParameterType {
    Number(f64),
    Note(String),
    Text(String),
    Reference(String),
}

#[derive(Debug)]
pub struct Operation {
    pub op: String,
    pub params: Vec<ParameterType>,
}

pub type ParsedResult = HashMap<String, Vec<Operation>>;

pub fn parse(tokens: &[(TokenType, Position)]) -> Result<ParsedResult> {
    let mut result = HashMap::new();
    let mut iter = tokens.iter().peekable();

    let mut current_key: Option<String> = None;
    let mut current_operations: Vec<Operation> = vec![];

    while let Some(&(ref token, _)) = iter.next() {
        match token {
            TokenType::Identifier(ref id) => {
                if let Some(key) = current_key {
                    result.insert(key, current_operations);
                }

                current_key = Some(id.clone());
                current_operations = vec![];
            }
            TokenType::Keyword(ref keyword) => {
                let mut operation = Operation {
                    op: keyword.clone(),
                    params: Vec::new(),
                };

                while let Some(&(next_token, _)) = iter.peek() {
                    match next_token {
                        TokenType::Number(ref number) => match number.parse() {
                            Ok(num) => {
                                operation.params.push(ParameterType::Number(num));
                            }
                            Err(_) => {
                                operation.params.push(ParameterType::Note(number.clone()));
                            }
                        },
                        TokenType::String(ref text) => {
                            operation.params.push(ParameterType::Text(text.clone()));
                        }
                        TokenType::Symbol(ref symbol) => {
                            operation.params.push(ParameterType::Text(symbol.clone()));
                        }
                        TokenType::Identifier(ref symbol) => {
                            operation
                                .params
                                .push(ParameterType::Reference(symbol.clone()));
                        }
                        TokenType::Connector => {
                            break;
                        }
                        TokenType::Comment(_) => {
                            break;
                        }

                        _ => {
                            bail!("Unexpected token in parameters {:?}", next_token);
                        }
                    }
                    iter.next(); // Consume the parameter token
                }

                current_operations.push(operation);
            }
            TokenType::Comment(_) | TokenType::Colon => {
                // Ignore comments and colons
            }
            _ => {
                println!("unexpected {:?}", token);
            }
        }
    }

    if let Some(key) = current_key {
        result.insert(key, current_operations);
    }

    Ok(result)
}
