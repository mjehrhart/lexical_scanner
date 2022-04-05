use enums::Token;
use std::{fs::OpenOptions, io::Read};

pub mod enums;
pub mod lexer;

#[allow(unused_imports)]
#[warn(missing_docs)]

fn read_template(path: &str) -> Result<String, std::io::Error> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    let _ensual = match f.read_to_end(&mut buffer) {
        Ok(_bit_count) => {}
        Err(e) => {
            panic!(
                "let suc = match file_from.read_to_end(&mut buffer)^^^ERROR {:?}",
                e
            )
        }
    };

    let page = std::str::from_utf8(&buffer).unwrap();
    Ok(page.to_string())
}

pub fn test5() {
    println!("{}", 500);
}
/// Constructs a vector of tokens.
/// This straight forward lexical scanner is preset to support over 75 tokens.  The list of tokens can be found at this sites
/// github page.
/// Usually, this is the main method for generating tokens by passing in a file path to the document you want to perform a lexical scan on.
/// Example
/// ``` rust
/// pub use lexical_scanner::*;
/// pub use enums::*;
/// let path = "./test/test.txt";
/// let token_list = lexical_scanner::lexer(path);
/// println!("{:?}, ", token_list);
/// ```
///
pub fn lexer(path: &str) -> Vec<Token> {
    let this = read_template(path);

    if let Ok(page) = this {
        let tmp = page.clone();

        let mut token_container = vec![];
        let tokenizer = lexer::lexer::lexer::Tokenizer::new(&tmp);

        for token in tokenizer {
            match Some(token) {
                Some(t) => match t {
                    enums::Token::Undefined => break,
                    _ => {
                        //println!("{}. {:?}", i, t);
                        token_container.push(t);
                    }
                },
                None => break,
            }
        }

        return token_container;
    }

    vec![]
}

/// Constructs a vector of tokens.
/// This straight forward lexical scanner is preset to support over 75 tokens.  The list of tokens can be found at this sites
/// github page.
/// Usually, passing in a text string is used for debugging and testing.
/// Example
/// ``` rust
/// pub use lexical_scanner::*;
/// pub use enums::*;
/// let text = "The number 5.0 is > 1;";
/// let token_list = lexical_scanner::lexer_as_str(text);
/// ```
///
pub fn lexer_as_str(text: &str) -> Vec<Token> {
    let mut token_container = vec![];
    let tokenizer = lexer::lexer::lexer::Tokenizer::new(&text);

    for token in tokenizer {
        match Some(token) {
            Some(t) => match t {
                enums::Token::Undefined => break,
                _ => {
                    //println!("{}. {:?}", i, t);
                    token_container.push(t);
                }
            },
            None => break,
        }
    }

    return token_container;
}

// Unit tests
#[cfg(test)]
mod unit_test {
    //zuse super::*;
    use crate::{enums::Token, lexer::lexer::lexer::Tokenizer};

    #[test]
    fn test_basic_tokenizer() {
        let mut tokenizer = Tokenizer::new("Water is healthy!");
        assert_eq!(tokenizer.next().unwrap(), Token::Word("Water".to_string()));
        assert_eq!(tokenizer.next().unwrap(), Token::WhiteSpace);
        assert_eq!(tokenizer.next().unwrap(), Token::Word("is".to_string()));
        assert_eq!(tokenizer.next().unwrap(), Token::WhiteSpace);
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Word("healthy".to_string())
        );
        assert_eq!(tokenizer.next().unwrap(), Token::Not);
    }

    #[test]
    fn test_punctuation_tokenizer() {
        let mut tokenizer = Tokenizer::new("use super::*;");
        assert_eq!(tokenizer.next().unwrap(), Token::KW_Use);
        assert_eq!(tokenizer.next().unwrap(), Token::WhiteSpace);
        assert_eq!(tokenizer.next().unwrap(), Token::KW_Super);
        assert_eq!(tokenizer.next().unwrap(), Token::PathSep);
        assert_eq!(tokenizer.next().unwrap(), Token::Star);
        assert_eq!(tokenizer.next().unwrap(), Token::Semi);
    }

    #[test]
    fn test_numeric_tokenizer() {
        let mut tokenizer = Tokenizer::new("200 404.4 5_000");
        assert_eq!(tokenizer.next().unwrap(), Token::Numeric("200".to_string()));
        assert_eq!(tokenizer.next().unwrap(), Token::WhiteSpace);
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Floating("404.4".to_string())
        );
        assert_eq!(tokenizer.next().unwrap(), Token::WhiteSpace);
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Floating("5_000".to_string())
        );
    }
}
