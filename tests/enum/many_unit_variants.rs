
use astray_core::*;
use astray_macro::{set_token, SN};

set_token!(Token);

#[derive(SN, PartialEq, Debug, Clone)]
pub enum Operator {
    #[pattern(Token::Plus)]
    Plus,

    #[pattern(Token::Minus)]
    Minus,
}

fn main() {
    plus();
    minus();
    none_of_the_variants();
}

fn plus(){
    let mut tokens = TokenIter::new(vec![t!(+)]);
    let result = Operator::parse(&mut tokens);
    match result {
        Ok(Operator::Plus) => (),
        other => panic!("Expected Operator::Plus, but got {:?}", other),
    }

}

fn none_of_the_variants(){
    let mut tokens = TokenIter::new(vec![t!(;)]);
    let result = Operator::parse(&mut tokens);
    match result {
        Ok(other) => panic!("Expected an Error, but got {:?}", other ),
        _err => (),
    }

}

fn minus() {
    let mut tokens = TokenIter::new(vec![t!(-)]);
    let result = Operator::parse(&mut tokens);
    match result {
        Ok(Operator::Minus) => (),
        other => panic!("Expected Operator::Minus, but got {:?}", other), 
    }

}