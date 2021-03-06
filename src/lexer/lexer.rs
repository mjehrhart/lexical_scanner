//! The lexical_scanner procceses the user's input and converts to a vector of 115+ various tokens.
//! Lexical_scanner works using rust Iterator trait. The fn next() allows the library to safely
//! view and identify character patterns without using regex.

#[allow(
    unused_variables,
    dead_code,
    non_camel_case_types,
    unused_imports,
    clippy::module_inception
)]

pub mod lexer {
    use super::*;
    use crate::enums::Token;
    use std::any::type_name;
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::iter::Peekable;
    use std::str::Chars;
    use std::vec::IntoIter;

    #[derive(Debug, Clone)]
    pub struct Tokenizer<'a> {
        pub expr: Peekable<Chars<'a>>,
        pub keywords: HashMap<&'a str, Token>,
    }

    impl<'a> Iterator for Tokenizer<'a> {
        type Item = Token;

        fn next(&mut self) -> Option<Token> {
            let next_char = self.expr.peek();
            match next_char {
                // (1) String Value
                Some(c) if Self::starts_with_double_quote(*c) => {
                    let mut value = c.to_string();
                    self.expr.next();
                    while let Some(peeking) = self.expr.peek() {
                        match Some(peeking) {
                            Some(c) if Self::starts_with_double_quote(*c) => {
                                value.push(*c);
                                self.expr.next();
                                break;
                            }
                            Some(c) => {
                                value.push(*c);
                                self.expr.next();
                            }
                            None => break,
                        }
                    }

                    Some(Token::String(value))
                }
                // (1) WhiteSpace
                Some(c) if Self::is_whitespace(*c) => {
                    self.expr.next();
                    Some(Token::WhiteSpace)
                }
                // (9) \x41 \x7F \n \r \t \\ \0 \' \"
                Some(c) if Self::is_escaped(*c) => match Some(c) {
                    //Some('\x41') => return Some(Token::BitCharacterCode7(c.to_string())),
                    //Some('\x7F') => return Some(Token::BitCharacterCode8(c.to_string())),
                    Some('\n') => {
                        self.expr.next();
                        Some(Token::Newline)
                    }
                    Some('\r') => {
                        self.expr.next();
                        Some(Token::CarriageReturn)
                    }
                    Some('\t') => {
                        self.expr.next();
                        Some(Token::Tab)
                    }
                    Some('\\') => {
                        self.expr.next();
                        Some(Token::Backslash)
                    }
                    Some('\0') => {
                        self.expr.next();
                        Some(Token::Null)
                    }
                    Some('\'') => {
                        self.expr.next();
                        Some(Token::SingleQuote)
                    }
                    Some('\"') => {
                        self.expr.next();
                        Some(Token::DoubleQuote)
                    }
                    Some(c) => {
                        println!("Token::Undefined(1)::'{}'", c);
                        self.expr.next();
                        Some(Token::Undefined)
                    }
                    None => {
                        println!("Token::Undefined(2)::'None'");
                        self.expr.next();
                        Some(Token::Undefined)
                    }
                },
                // (7) @ _ , ; # $ ?
                Some(c) if Self::is_lesser_punctutation(*c) => match Some(c) {
                    Some('@') => {
                        self.expr.next();
                        Some(Token::At)
                    }
                    Some('_') => {
                        self.expr.next();
                        Some(Token::Underscore)
                    }
                    Some(',') => {
                        self.expr.next();
                        Some(Token::Comma)
                    }
                    Some(';') => {
                        self.expr.next();
                        Some(Token::Semi)
                    }
                    Some('#') => {
                        self.expr.next();
                        Some(Token::Pound)
                    }
                    Some('$') => {
                        self.expr.next();
                        Some(Token::Dollar)
                    }
                    Some('?') => {
                        self.expr.next();
                        Some(Token::Question)
                    }
                    Some(c) => {
                        println!("Token::Undefined(3)::'{}'", c);
                        Some(Token::Undefined)
                    }
                    None => {
                        //println!("Token::Undefined(4)::'None'");
                        Some(Token::Undefined)
                    }
                },
                // (5) Numeric, . .. ... ..=
                Some(c) if Self::is_numeric_with_dot(*c) => {
                    let mut value = c.to_string();
                    self.expr.next();
                    while let Some(peeking) = self.expr.peek() {
                        match Some(peeking) {
                            Some(cc) if Self::is_numeric_with_dot_eq_underscore(*cc) => {
                                value.push(*cc);
                                self.expr.next();
                            }
                            Some(_) => {
                                break;
                            }
                            None => break,
                        }
                    }

                    match Some(value.as_str()) {
                        Some(".") => return Some(Token::Dot),
                        Some("..") => return Some(Token::DotDot),
                        Some("...") => return Some(Token::DotDotDot),
                        Some("..=") => return Some(Token::DotDotEq),
                        Some(_) => {}
                        None => {}
                    }
                    if value.contains('.') || value.contains('_') {
                        return Some(Token::Floating(value));
                    }

                    Some(Token::Numeric(value))
                }
                // (41) = : :: > >= >> < <= << => += -= *= /= &= ^= &= |= == != + - * / % ^ & && | || ! // /* */ >>= <<= -> /// //! /*! /**
                Some(c) if Self::is_punctuation(*c) => {
                    let (token, next_this_times) =
                        Self::next_punctuation(c.to_string(), self.expr.clone());

                    //Advance 'next()' x times position since self.expr was cloned()
                    for i in 0..next_this_times {
                        self.expr.next();
                    }
                    token
                }
                // (6) {}[]()
                Some(c) if Self::bracket_delimiters(*c) => match Some(c) {
                    Some('{') => {
                        self.expr.next();
                        Some(Token::CurlyBraceLeft)
                    }
                    Some('}') => {
                        self.expr.next();
                        Some(Token::CurlyBraceRight)
                    }
                    Some('[') => {
                        self.expr.next();
                        Some(Token::BracketLeft)
                    }
                    Some(']') => {
                        self.expr.next();
                        Some(Token::BracketRight)
                    }
                    Some('(') => {
                        self.expr.next();
                        Some(Token::ParenLeft)
                    }
                    Some(')') => {
                        self.expr.next();
                        Some(Token::ParenRight)
                    }
                    Some(c) => {
                        //self.expr.next();
                        println!("Token::Undefined(5)::'{}'", c);
                        Some(Token::Undefined)
                    }
                    None => {
                        //self.expr.next();
                        //println!("Token::Undefined(6)::'None'");
                        Some(Token::Undefined)
                    }
                },
                // Word()
                Some(c) if Self::is_word(*c) => {
                    let mut value = c.to_string();
                    self.expr.next();
                    while let Some(peeking) = self.expr.peek() {
                        match Some(peeking) {
                            Some(cc) if Self::is_word(*peeking) => {
                                value.push(*cc);
                                self.expr.next();
                            }
                            Some(_) => {
                                break;
                            }
                            None => break,
                        }
                    }

                    //Check if word is KeyWord
                    let flag = self.keywords.get(&*value);
                    match flag {
                        Some(_) => {
                            let token = Self::translate_token_to_keyword_token(
                                flag.unwrap(),
                                value.to_string(),
                            );
                            Some(token.unwrap())
                        }
                        None => Some(Token::Word(value)),
                    }
                }
                // Catch All
                Some(c) => {
                    let value = c.to_string();
                    self.expr.next();
                    Some(Token::Character(value))
                }
                None => {
                    //println!("Token::Undefined(7)::'None'");
                    Some(Token::Undefined)
                }
            }
        }
    }


    pub mod numeric {
        use crate::enums::Token;
        use crate::lexer::lexer::lexer::Tokenizer;
        use std::{iter::Peekable, str::Chars};

        impl<'a> Tokenizer<'a> {
            /// Check if character is numeric or contains a Dot or Underscore
            pub fn is_numeric_with_dot(c: char) -> bool {
                c.is_ascii_digit() || c == '.' || c == '_'
            }

            /// Check if character is numeric or contains a Dot or Underscore or Eq
            /// This is a sub guard used for more detailed matching
            pub fn is_numeric_with_dot_eq_underscore(c: char) -> bool {
                c.is_ascii_digit() || c == '.' || c == '_' || c == '='
            }
        }
    }

    pub mod escapes {
        use crate::enums::Token;
        use crate::lexer::lexer::lexer::Tokenizer;

        /// Check if character is escaped
        //TODO add support for "c == '\x41'""
        impl<'a> Tokenizer<'a> {
            pub fn is_escaped(c: char) -> bool {
                //c == '\x41'
                c == '\n'
                    || c == '\r'
                    || c == '\t'
                    || c == '\\'
                    || c == '\0'
                    || c == '\x7F'
                    || c == '\''
                    || c == '\"'
            }

            /// Check if character is a type of bracket
            pub fn bracket_delimiters(c: char) -> bool {
                c == '{' || c == '[' || c == '(' || c == ')' || c == ']' || c == '}'
            }
        }
    }

    pub mod generic {
        use std::{iter::Peekable, str::Chars};

        use crate::enums::Token;
        use crate::lexer::lexer::lexer::Tokenizer;

        impl<'a> Tokenizer<'a> {
            /// Check if character is "
            pub fn starts_with_double_quote(c: char) -> bool {
                c == '"'
            }

            /// Check if character is whitespace
            pub fn is_whitespace(c: char) -> bool {
                c == ' '
            }

            /// Check if character is alphanumeric or underscore
            /// Words can start with or contain an underscore
            pub fn is_word(c: char) -> bool {
                c.is_alphanumeric() || c == '_'
            }

            /// Check if character is a major punctutation
            pub fn is_punctuation(c: char) -> bool {
                //println!("___________ c is '{}'", &c);
                c == '.'
                    || c == '+'
                    || c == '-'
                    || c == '*'
                    || c == '/'
                    || c == '%'
                    || c == '^'
                    || c == '&'
                    || c == '|'
                    || c == '!'
                    || c == ':'
                    || c == '>'
                    || c == '='
                    || c == '<'
            }

            /// Check if character is a lesser punctutation
            /// Punctuation are split up for ease of control
            pub fn is_lesser_punctutation(c: char) -> bool {
                c == '@' || c == '_' || c == ',' || c == ';' || c == '#' || c == '$' || c == '?'
            }
        }
    }

}
