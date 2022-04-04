# Documentation.

## Setup
Add this depedency to TOML
``` rust
[dependencies]
lexical_scanner = "0.1.4"
```

## Basic Usage
``` rust
use lexical_scanner;

fn main() {
    let path = "/Users/gues/my_file_to_read_into_tokens/temp.txt";
    let token_list = lexical_scanner::lexer(&path); 
}
```
That is all there is to do!  The lexical scanner returns a Vec<Token> for the user to handle as needed. Below is a simple way to view the tokens for unit testing:
```
for (i, token) in token_list.iter().enumerate(){
    println!("{}. {:?}", i, token);
}
```
