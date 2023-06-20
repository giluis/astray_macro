use astray_macro::{SN, set_token};
use astray_core::*;
use hatch_result::*; 

set_token!(Token);

#[derive(SN, PartialEq, Clone)]
pub enum TestEnum {
    DoubleComma(DoubleComma),

    #[pattern(Token::LiteralInt(_))]
    LitInt(Token),

    #[pattern(Token::SemiColon)]
    SemiColon(Token),
}


#[derive(SN, PartialEq, Clone)]
pub struct DoubleComma {
    #[pattern(Token::Comma)]
    comma1: Token,

    #[pattern(Token::Comma)]
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
