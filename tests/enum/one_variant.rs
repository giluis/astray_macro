use astray_macro::*;
use astray_core::*;
use hatch_result::ResultHatchExt;

set_token!(Token);

#[derive(SN, Clone, PartialEq)]
pub enum Type {
    #[pattern( Token::KInt )]
    KInt(Token),
}

fn main() {
    let tokens = vec![
        t!( int ),
        t!( ident "some_function" )
    ];
    let result = Type::parse(&mut TokenIter::new(tokens));
    match result {
        Ok(Type::KInt(Token::KInt)) => (),
        Ok(_) => panic!("There is only one variant to this enum"), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Should return Ok")
    }
    // assert!(currentBefore + 1 == iter.current );
}
