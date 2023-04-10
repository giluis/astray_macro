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

#[derive(SN, PartialEq, Clone)]
pub struct AssignStatement {
    ty: Type,

    // #[stateful_leaf( Token::Identifier )]
    // ident: String,

    // #[leaf( Token::Assign )]
    // equals_sign: String,


    // #[leaf( Token::LiteralInt )]
    // value: u32

    // omitted expression (fearing recursion)
}

#[derive(SN, PartialEq,Clone)]
pub struct Type {
    #[from(Token::KInt)]
    int: Token,
}



// impl Parsable for Type  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//      let int = match iter.expect_token(Token::KInt) ? {
//          Token::KInt(int) => int,
//          _ => panic!("Internal error: Ok result for iter.expect should always yield token of the same kind as input "),
//      }
//
//      Ok(Type {
//          int
//      })
//    }
// }
//


fn main() {
    let mut iter = TokenIter::new(vec![t!( int )]);

    let result = iter.parse::<AssignStatement>();
    let expected = AssignStatement::new(Type::new(Token::KInt));
    assert!(Ok(expected) == result);
}
