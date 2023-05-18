use astray_macro::{SN, set_token};
use astray_core::*;

set_token!{Token}

#[derive(SN, Debug, PartialEq)]
pub enum Punct {
    #[from(Token::Assign)]
    EqualSign(Token), 
    SemiOrComma(SemiOrComma),
}

#[derive(SN, Debug, PartialEq)]
pub enum SemiOrComma {
    #[from(Token::Comma)]
    Comma(Token),

    #[from(Token::SemiColon)]
    Semi(Token),
}

fn main() {
    let mut token_iter = TokenIter::new(vec![
        t!(=),
        ]);
    
    let result = token_iter.parse::<Punct>();
    match result {
        Ok(Punct::EqualSign(Token::Assign)) => (),
        Ok(other) => panic!("Expect Punct::EqualSign variant, but got {:?}", other), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expecte Ok(Punct::EqualSign(Token::Assign))(Punct::EqualSign(Token::Assign)) Result, but got error"),
    }

    let mut token_iter = TokenIter::new( vec![
        t!(,),
        ] );
    let result = token_iter.parse::<Punct>();
    match result {
        Ok(Punct::SemiOrComma(SemiOrComma::Comma(Token::Comma))) => (),
        Ok(other) => panic!("Expected Punct::SemiOrComma(SemiOrComma::Semi(Token::Comma)) variant, but got {:?}", other), // internal error: token should always be t! (int) when result is OK
        Err(msg) => panic!("Expected Ok(Punct::SemiOrComma(SemiOrComma::Comma(Token::Comma))) Result, but got error: {:?}", msg),
    }
    // assert!(currentBefore + 1 == iter.current );

    let mut token_iter =TokenIter::new(  vec![
        t!(;),
        ] );

    let result = token_iter.parse::<Punct>();
    match result {
        Ok(Punct::SemiOrComma(SemiOrComma::Semi(Token::SemiColon))) => (),
        Ok(other) => panic!("Expected Punct Punct::SemiOrComma(SemiOrComma::Semi(Token::SemiColon)) variant, but got {:?}", other), // internal error: token should always be t! (int) when result is OK
        Err(msg) => panic!("Expected Ok Result, but got error: {:?}", msg),
    }
}
