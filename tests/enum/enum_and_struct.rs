use astray_macro::{SN, set_token};
use astray_core::*;

set_token!{Token}

#[derive(SN, Debug, PartialEq)]
pub struct FnCall {
    #[from(Token::Identifier(_))]
    ident: Token,
    args: Args,
}

#[derive(SN, Debug, PartialEq)]
pub enum Args {
    EmptyArgs(EmptyArgs),
    FullArgs(FullArgs),
}

#[derive(SN, Debug, PartialEq)]
pub struct EmptyArgs {
    #[from(Token::LParen)]
    l_paren: Token,

    #[from(Token::RParen)]
    r_paren: Token,
}

#[derive(SN, Debug, PartialEq)]
pub struct FullArgs {
    #[from(Token::LParen)]
    l_paren: Token,

    ty1: ArgType,

    #[from(Token::Identifier(_))]
    ident1: Token,

    #[from(Token::RParen)]
    r_paren: Token,
}

#[derive(SN, Debug, PartialEq)]
pub enum ArgType {
    #[from(Token::KInt)]
    KInt(Token),

    #[from(Token::KFloat)]
    KFloat(Token),
}


fn main() {
    test_empty_args();
    test_full_args();
}

fn test_full_args() {
    let expected_fn_ident1 = "fn1";
    let expected_arg_ident1 = "arg1";
    let tokens = vec![
        t!(ident expected_fn_ident1), 
        t!(l_paren), 
        t!(float),
        t!(ident expected_arg_ident1),
        t!(r_paren)];

    let mut token_iter = TokenIter::new(tokens);
    let result = token_iter.parse::<FnCall>();
    let expected = FnCall {
        ident: Token::Identifier(expected_fn_ident1.to_string()),
        args: Args::FullArgs(FullArgs {
            l_paren: t!(l_paren),
            ty1: ArgType::KFloat(Token::KFloat),
            ident1: Token::Identifier(expected_arg_ident1.to_string()),
            r_paren: t!(r_paren),
        }),
    };
    assert_eq!(Ok(expected), result);
}

fn test_empty_args() {
    let expected_fn_ident = "fn1";
    let tokens = vec![t!(ident expected_fn_ident), t!(l_paren), t!(r_paren)];
    let mut token_iter = TokenIter::new(tokens);
    let result = token_iter.parse::<FnCall>();
    let expected = FnCall {
        ident: Token::Identifier(expected_fn_ident.to_string()),
        args: Args::EmptyArgs(EmptyArgs {
            l_paren: t!(l_paren),
            r_paren: t!(r_paren),
        }),
    };
    assert_eq!(Ok(expected), result);
}