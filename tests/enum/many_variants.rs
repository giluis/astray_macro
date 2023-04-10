use astray_macro::{SN, set_token};
use astray_core::*;
use hatch_result::HatchResultExt;

set_token!(Token);

#[derive(SN)]
pub enum TestEnum {
    DoubleComma(DoubleComma),

    #[from(Token::LiteralInt(ANY))]
    LitInt(Token),

    #[from(Token::SemiColon)]
    SemiColon(Token),
}

impl TestEnum {
    fn pparse(iter: &mut TokenIter) -> TestEnum {
        let semi_err = if let Ok(result) = iter.expect(Token::SemiColon) && matches!(result, Token::SemiColon) {
            return TestEnum::LitInt(result)
        } else {
            Err("Could not parse smiecolon")

        };
        
        let litint_err = if let Ok(result) = iter.expect(Token::LiteralInt(())) && matches!(result, Token::SemiColon) {
            return TestEnum::LitInt(result)
        } else {
            Err("Could not parse smiecolon")

        };

    }

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
