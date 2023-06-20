use astray_core::*;
use astray_macro::{set_token, SN};

set_token! {Token}

#[derive(SN, PartialEq, Clone)]
pub struct AssignStatement {
    ty: Type,

    ident: Identifier,

    #[pattern(Token::Assign)]
    equals_sign: Token,

    #[pattern(Token::LiteralInt(_))]
    lit_int: Token,
}

#[derive(SN, PartialEq, Clone)]
struct LiteralInt {
    #[pattern(Token::LiteralInt(_))]
    ident: Token,
}

#[derive(SN, PartialEq, Clone)]
struct Identifier {
    #[pattern(Token::Identifier(_))]
    ident: Token,
}

#[derive(SN, PartialEq, Clone)]
pub struct Type {
    #[pattern(Token::KInt)]
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

    let expected = AssignStatement{
        ty: Type{int: Token::KInt},
        ident: Identifier {
            ident: Token::Identifier("var1".to_string()),
        },
        equals_sign: Token::Assign,
        lit_int: Token::LiteralInt(1999),
    };
    assert!(Ok(expected) == result);
}
