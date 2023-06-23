use astray_core::*;
use astray_macro::{set_token, SN};

set_token!(Token);

#[derive(SN, PartialEq, Debug, Clone)]
pub enum TestEnum {
    DoubleComma(DoubleComma),

    #[pattern(Token::LiteralInt(_))]
    LitInt(Token),

    #[pattern(Token::SemiColon)]
    SemiColon,
}

#[derive(SN, PartialEq, Debug, Clone)]
pub struct DoubleComma {
    #[pattern(Token::Comma)]
    comma1: Token,

    #[pattern(Token::Comma)]
    comma2: Token,
}

fn main() {
    semi();
    lit_int();
    double_comma();
    none_of_the_variants();
}

fn none_of_the_variants() {
    let mut tokens = TokenIter::new(vec![t!(ident "hello")]);
    let result = TestEnum::parse(&mut tokens);
    match result {
        Ok(_) => panic!("Should not be ok, since no rule expects ident hello"),
        Err(_) => (), // passed
    }
}

fn semi() {
    let mut tokens = TokenIter::new(vec![t!(;)]);
    let result = TestEnum::parse(&mut tokens);
    match result {
        Ok(TestEnum::SemiColon) => (),
        Ok(other) => panic!("Expect SemiColon variant, but didn't get that {:?}", other), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expecte Ok Result"),
    }
}

fn lit_int() {
    let mut tokens = TokenIter::new(vec![t!(litint 3)]);
    let result = TestEnum::parse(&mut tokens);
    match result {
        Ok(TestEnum::LitInt(t!(litint 3))) => (),
        Ok(other) => panic!("Expect LitInt, but got {:?}", other), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expecte Ok Result"),
    }
}

fn double_comma() {
    let mut tokens = TokenIter::new(vec![t!(,), t!(,)]);
    let result = TestEnum::parse(&mut tokens);
    match result {
        Ok(TestEnum::DoubleComma(DoubleComma {
            comma1: Token::Comma,
            comma2: Token::Comma,
        })) => (),
        Ok(other) => panic!("Expect DoubleComma variant, but got {:?}", other), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expecte Ok Result"),
    }
}
