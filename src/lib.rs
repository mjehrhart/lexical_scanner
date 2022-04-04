use std::{fs::OpenOptions, io::Read};

use enums::Token;

mod enums;
#[allow(unused_imports)]
#[warn(missing_docs)]
mod lexer;

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

pub fn lexer(path: &str) -> Vec<Token> {
    let this = read_template(path);

    if let Ok(page) = this {
        let tmp = page.clone();

        let mut token_container = vec![];
        let tokenizer = lexer::lexer::lexer::Tokenizer::new(&tmp);

        let mut i = 0;
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
            i += 1;

            if i > 100000 {
                break;
            }
        }

        return token_container;
    }

    vec![]
}

// Unit tests
#[cfg(test)]
mod unit_test {
    use super::*;
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
