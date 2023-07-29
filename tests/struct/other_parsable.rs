// Resources:
//
//   - The Quote crate for putting together output from a macro:
//     https://github.com/dtolnay/quote
//
//   - Joining together the type name + "Builder" to make the builder's name:
//     https://docs.rs/syn/1.0/syn/struct.Ident.html

use astray_macro::{SN,set_token};
use astray_core::*;

set_token!(Token);

#[derive(SN, Debug, PartialEq, Clone)]
pub struct AssignStatement {
    ty: Type,
}

#[derive(SN, Debug, PartialEq, Clone)]
pub struct Type {
    #[pattern(Token::KInt)]
    int: Token,
}

fn main() {
    let mut iter = TokenIter::new(vec![t!( int )]);
    let result = iter.parse::<AssignStatement>();
    let expected = AssignStatement{ty: Type{int: Token::KInt}};
    assert!(Ok(expected) == result);
}
