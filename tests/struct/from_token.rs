use astray_macro::{set_token,SN};
use astray_core::*;

set_token!(Token);

#[derive(SN, PartialEq, Debug)]
pub struct KInt {
    #[pattern(Token::KInt)]
    kint: Token
}

// impl Parsable<Token>  for KInt  {
//    fn parse(iter:&mut TokenIter<Iter>) -> Result<KInt, String> {
//      let kint = iter.parse_if_match(|tok|matches!(tok,Token::KInt))?;
//      Ok(Identifier{kint})
//    }
// }

fn main() {

    let tokens = vec![
        t!( int )
    ];
    let mut iter = TokenIter::new(tokens);
    let result = KInt::parse(&mut iter);
    
    assert!(result.unwrap().kint == Token::KInt);

}
