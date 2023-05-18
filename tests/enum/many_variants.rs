use astray_macro::{SN, set_token};
use astray_core::*;
use hatch_result::*; 

set_token!(Token);

#[derive(SN)]
pub enum TestEnum {
    DoubleComma(DoubleComma),

    #[from(Token::LiteralInt(_))]
    LitInt(Token),

    #[from(Token::SemiColon)]
    SemiColon(Token),
}


#[derive(SN)]
pub struct DoubleComma {
    #[from(Token::Comma)]
    comma1: Token,

    #[from(Token::Comma)]
    comma2: Token,
}


fn main() {
    let mut tokens = TokenIter::new(vec![t!(,), t!(,)]);
    let result = TestEnum::parse(&mut tokens);
    match result {
        Ok(TestEnum::DoubleComma(DoubleComma {
            comma1: Token::Comma,
            comma2: Token::Comma,
        })) => (),
        Ok(_) => panic!("Expect DoubleComma variant, but didn't get that "), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expecte Ok Result"),
    }
}
