//! The lexical_scanner procceses supports over 115+ various tokens. All major punctuation patterns 
//! are supported.
use std::ops::Deref;

///Token field and description for lexical_scanner

#[allow(
    dead_code,
    clippy::upper_case_acronyms,
    clippy::enum_variant_names,
    non_camel_case_types
)]
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Token {
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
    KW_UserDefined(String),
    KW_Where,
    KW_While,
}

impl Deref for Token {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self
    }
}
