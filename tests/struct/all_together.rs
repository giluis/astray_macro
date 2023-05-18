use astray_core::*;
use astray_macro::{set_token, SN};

set_token! {Token}

#[derive(SN, PartialEq, Clone)]
pub struct AssignStatement {
    ty: Type,

    ident: Identifier,

    #[from(Token::Assign)]
    equals_sign: Token,

    #[from(Token::LiteralInt(_))]
    lit_int: Token,
}

#[derive(SN, PartialEq, Clone)]
struct LiteralInt {
    #[from(Token::LiteralInt(_))]
    ident: Token,
}

#[derive(SN, PartialEq, Clone)]
struct Identifier {
    #[from(Token::Identifier(_))]
    ident: Token,
}

#[derive(SN, PartialEq, Clone)]
pub struct Type {
    #[from(Token::KInt)]
    int: Token,
}

fn main() {
    let mut iter = TokenIter::new(vec![
        t!(int),
        t!( ident "var1" ),
        t!( = ),
        t!( litint 1999 ),
    ]);
    let result = iter.parse::<AssignStatement>();

    let expected = AssignStatement::new(
        Type::new(Token::KInt),
        Identifier {
            ident: Token::Identifier("var1".to_string()),
        },
        Token::Assign,
        Token::LiteralInt(1999),
    );
    assert!(Ok(expected) == result);
}
