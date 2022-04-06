//! The lexical_scanner procceses the user's input and converts to a vector of 75+ various tokens.
//! Lexical_scanner works using rust Iterator trait.  Using Peekable(), the library can safely 
//! view and speedily identify character patterns without using regex.

use enums::Token;
use std::{fs::OpenOptions, io::Read};

pub mod enums;
pub mod lexer;

#[allow(unused_imports)]
#[warn(missing_docs)]
 
 
/// Converts a file content's to a Vector of Tokens  
/// Input:: path: &str  
/// Return -> Vec<Token>   
/// Typically this is the main method for generating tokens by passing in a file path to the document you want to perform a lexical scan on.  
/// Example  
/// ``` rust
/// pub use lexical_scanner::*;
/// pub use enums::*;
/// 
/// let path = "./test/test.txt";
/// let token_list = lexical_scanner::lexer(path);
/// 
/// //Display tokens
/// for (i, token) in token_list.iter().enumerate(){
///     println!("{}. {:?}", i, token);
/// }
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

        let token_container =
            lexer::lexer::lexer::Tokenizer::transform_special_tokens_into_raw_byte_tokens(
                token_container,
            );

        return token_container;
    }

    vec![]
}

/// Converts a string to tokens  
/// Input:: text: &str  
/// Return -> Vec<Token>  
/// This is comonnly used for debugging and testing.  
/// Example  
/// ``` rust
/// pub use lexical_scanner::*;
/// pub use enums::*;
/// let text = "The number 5.0 is > 1;";
/// let token_list = lexical_scanner::lexer_as_str(text);
/// 
/// //Display tokens
/// for (i, token) in token_list.iter().enumerate(){
///     println!("{}. {:?}", i, token);
/// }
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

    let token_container =
        lexer::lexer::lexer::Tokenizer::transform_special_tokens_into_raw_byte_tokens(
            token_container,
        );

    return token_container;
}


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
