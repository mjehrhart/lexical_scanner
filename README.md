# Documentation.

## Setup
Add this depedency to TOML
``` rust
[dependencies]
lexical_scanner = "0.1.9"
```

## Basic Usage
The two ways to perform a lexical scan is to pass in file path or pass in a string. Passing in a string is mostly used for testing while passing in a file path is common for every day work. A lexical scanner can produces thousand of tokens very quickly. For this reason, it is best to use a file path.
``` rust
use lexical_scanner;

fn main() {
    let path = "/Users/gues/my_file_to_read_into_tokens/temp.txt";
    let token_list = lexical_scanner::lexer(&path); 
}
```
That is all there is to do!  The lexical scanner returns a Vec<Token> for the user to handle as needed. 

To test with a string, all you need to do is call this
``` rust
use lexical_scanner;

fn main() {
    let text = "Water is a good choice of drink!";
    let token_list = lexical_scanner::lexer_as_str(&text); 
}
```

Below is a simple way to view the tokens for unit testing:
``` rust
for (i, token) in token_list.iter().enumerate(){
    println!("{}. {:?}", i, token);
}
```
## Supported Tokens
```
And,
AndAnd,
AndEq,
At,
Backslash,
BitCharacterCode7(String),
BitCharacterCode8(String),
BlockCommentStart(String),
BlockCommentStop(String),
BracketLeft,
BracketRight,
Byte(String),
ByteString(String),
Caret,
CaretEq,
CarriageReturn,
Character(String),
Colon,
Comma,
CurlyBraceLeft,
CurlyBraceRight,
Dollar,
Dot,
DotDot,
DotDotDot,
DotDotEq,
DoubleQuote,
Eq,
EqEq,
Ge,
Gt,
FatArrow,
InnerBlockDoc(String),
InnerLineDoc(String),
Le,
LineComment(String),
Lt,
Minus,
MinusEq,
Or,
OrEq,
OrOr,
OuterBlockDoc(String),
OuterLineDoc(String),
Newline,
Not,
NotEq,
Null,
Floating(String),
Numeric(String),
ParenLeft,
ParenRight,
PathSep,
Percent,
PercentEq,
Plus,
PlusEq,
Pound,
Question,
RArrow,
RawString(String),
RawByteString(String),
Semi,
Shl,
ShlEq,
Shr,
ShrEq,
SingleQuote,
Slash,
SlashEq,
Star,
StarEq,
Stopped(String), //for debugging
String(String),
Tab,
Undefined,
Underscore,
WhiteSpace,
Word(String),
KW_As,
KW_Async,
KW_Await,
KW_Break,
KW_Const,
KW_Contine,
KW_Crate,
KW_Dyn,
KW_Else,
KW_Enum,
KW_Extern,
KW_False,
KW_Fn,
KW_For,
KW_If,
KW_Impl,
KW_In,
KW_Let,
KW_Loop,
KW_Match,
KW_Mod,
KW_Move,
KW_Mut,
KW_Pub,
KW_Ref,
KW_Return,
KW_SELF,
KW_Self,
KW_Static,
KW_Struct,
KW_Super,
KW_Trait,
KW_True,
KW_Type,
KW_Union,
KW_Unsafe,
KW_Use,
KW_Where,
KW_While,
```

crates.io => https://crates.io/crates/lexical_scanner
github.com => https://github.com/mjehrhart/lexical_scanner
