# Documentation
This complete Lexer/Lexical Scanner produces over 115+ tokens for a string or a file path entry. The output is a Vector<Token> for the user to handle according to their needs. All tokens are included (including whitespace) as that is left to the user to decide how to use the tokens.

If you have any questions, comments, or need help, feel free to add a discussion here:

https://github.com/mjehrhart/lexical_scanner/discussions

To see an example of an output, check out the wiki page:

https://github.com/mjehrhart/lexical_scanner/wiki/Example

## Setup
Add this depedency to TOML
``` rust
[dependencies]
lexical_scanner = "0.1.18"
```

## Basic Usage 
The two ways to perform a lexical scan is to pass in file path or pass in a string. Passing in a string is mostly used for testing while passing in a file path is common for every day work. A lexical scanner can produces thousands and thousands of tokens very quickly. For this reason, it is best to use a file path.
``` rust
use lexical_scanner;

fn main() {
    let path = "/Users/gues/my_file_to_read_into_tokens/temp.txt";
    let token_list = lexical_scanner::lexer(&path); 
}

input -> 
: :: > >= >> < <= << => += -= *= /= &= ^= &= |= == != + - * / % ^ & && | || !  >>= <<= -> /// //! // /* */ /*! /**
output ->
0. Colon
1. WhiteSpace
2. PathSep
3. WhiteSpace
4. Gt
5. WhiteSpace
6. Ge
7. WhiteSpace
8. Shr
9. WhiteSpace
10. Lt
11. WhiteSpace
12. Le
13. WhiteSpace
14. Shl
15. WhiteSpace
16. FatArrow
17. WhiteSpace
18. PlusEq
19. WhiteSpace
20. MinusEq
21. WhiteSpace
22. StarEq
23. WhiteSpace
24. SlashEq
25. WhiteSpace
26. AndEq
27. WhiteSpace
28. CaretEq
29. WhiteSpace
30. AndEq
31. WhiteSpace
32. OrEq
33. WhiteSpace
34. EqEq
35. WhiteSpace
36. NotEq
37. WhiteSpace
38. Plus
39. WhiteSpace
40. Minus
41. WhiteSpace
42. Star
43. WhiteSpace
44. Slash
45. WhiteSpace
46. Percent
47. WhiteSpace
48. Caret
49. WhiteSpace
50. And
51. WhiteSpace
52. AndAnd
53. WhiteSpace
54. Or
55. WhiteSpace
56. OrOr
57. WhiteSpace
58. Not
59. WhiteSpace
60. LineComment("//")
61. WhiteSpace
62. BlockCommentStart("/*")
63. WhiteSpace
64. BlockCommentStop("*/")
65. WhiteSpace
66. ShrEq
67. WhiteSpace
68. ShlEq
69. WhiteSpace
70. RArrow
71. WhiteSpace
72. OuterLineDoc("///")
73. WhiteSpace
74. InnerLineDoc("//!")
75. WhiteSpace
76. InnerBlockDoc("/*!")
77. WhiteSpace
78. OuterBlockDoc("/**")
79. Newline
```
That is all there is to do!  The lexical scanner returns a Vec<Token> for the user to handle as needed. 

To test with a string, all you need to do is call this
``` rust
use lexical_scanner;

fn main() {
    let text = "The number 5.0 is > 1;";
    let token_list = lexical_scanner::lexer_as_str(&text); 
}
```

Below is a simple way to view the tokens for unit testing:
``` rust
for (i, token) in token_list.iter().enumerate(){
    println!("{}. {:?}", i, token);
}

output -> 
0. Word("The")
1. WhiteSpace
2. Word("number")
3. WhiteSpace
4. Floating("5.0")
5. WhiteSpace
6. Word("is")
7. WhiteSpace
8. Gt
9. WhiteSpace
10. Numeric("1")
11. Semi

```

## Custom keywords
There is a way to add in your own keyword identifiers.  Doing so will help manage parsing of the tokens.
``` rust
use lexical_scanner;

fn main() {
    let text = "The number 5.0 is left and nor right of the up and down 1;";
    let user_keywords = ["up", "down", "left", "right"];
    let token_list = lexical_scanner::lexer_with_user_keywords(&text, user_keywords.to_vec()); 
}
```

Below is a simple way to view the tokens for unit testing. You can see that "up", "down", left", and "right"
are have been tokenized into KW_UserDefined(String).
``` rust
for (i, token) in token_list.iter().enumerate(){
    println!("{}. {:?}", i, token);
}

output -> 
0. Word("The")
1. WhiteSpace
2. Word("number")
3. WhiteSpace
4. Floating("5.0")
5. WhiteSpace
6. Word("is")
7. WhiteSpace
8. KW_UserDefined("left")
9. WhiteSpace
10. Word("and")
11. WhiteSpace
12. Word("nor")
13. WhiteSpace
14. KW_UserDefined("right")
15. WhiteSpace
16. Word("of")
17. WhiteSpace
18. Word("the")
19. WhiteSpace
20. KW_UserDefined("up")
21. WhiteSpace
22. Word("and")
23. WhiteSpace
24. KW_UserDefined("down")
25. WhiteSpace
26. Numeric("1")
27. Semi
```


## Supported Tokens
```
& => And,
&& => AndAnd,
&= => AndEq,
@ => At,
\ => Backslash,
BitCharacterCode7(String),
BitCharacterCode8(String),
/* => BlockCommentStart(String),
*/ => BlockCommentStop(String),
[ => BracketLeft,
] => BracketRight,
b'H' => Byte(String),
b"Hello" => ByteString(String),
^ => Caret,
^ => CaretEq,
\r\n => CarriageReturn,
Character(String),
: => Colon,
, => Comma,
{ => CurlyBraceLeft,
} => CurlyBraceRight,
$ => Dollar,
. => Dot,
.. => DotDot,
... => DotDotDot,
..= => DotDotEq,
" => DoubleQuote,
= => Eq,
== => EqEq,
>= => Ge,
> => Gt,
=> => FatArrow,
//! => InnerBlockDoc(String),
/*! => InnerLineDoc(String),
< => Le,
// => LineComment(String),
<= Lt,
- => Minus,
-= => MinusEq,
| => Or,
|= => OrEq,
|| => OrOr,
/** => OuterBlockDoc(String),
/// => OuterLineDoc(String),
\n => Newline,
! => Not,
!= => NotEq,
Null,
3.14 => Floating(String),
314 => Numeric(String),
( => ParenLeft,
) => ParenRight,
:: => PathSep,
% => Percent,
%= => PercentEq,
+ => Plus,
+= => PlusEq,
# => Pound,
? => Question,
-> => RArrow,
r#"Hello"# => RawString(String),
rb#"Hello"# => RawByteString(String),
; => Semi,
<< => Shl,
<<= => ShlEq,
>> => Shr,
>>= => ShrEq,
' => SingleQuote,
/ => Slash,
/= => SlashEq,
* => Star,
*= => StarEq,
Stopped(String), //for debugging
"Hello" => String(String),
\t => Tab,
Undefined,
_ => Underscore,
' ' => WhiteSpace,
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
KW_UserDefined(String),
KW_Where,
KW_While,
```
  
crates.io => https://crates.io/crates/lexical_scanner  
github.com => https://github.com/mjehrhart/lexical_scanner  
  
