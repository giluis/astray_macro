use astray_core::*;
use astray_macro::{set_token, SN};

set_token!(Token);

#[derive(SN, Clone, PartialEq, Debug)]
pub struct ReturnStatement {

    #[pattern(Token::KReturn)]
    k_return: Option<Token>,

    #[pattern(Token::Identifier(_))]
    ident: Token,

    #[pattern(Token::SemiColon)]
    semi: Option<Token>,
}





fn main() {
    let tokens = vec![
        t!(return),
        t!(ident "some_ident"),
        t!(;)
    ];
    let expected = ReturnStatement{
        k_return: Some(t!(return)),
        ident: t!(ident "some_ident"),
        semi: Some(t!(;))
    };

    let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    assert_eq!(Ok(expected), result);

    let tokens = vec![
        t!(ident "some_ident"),
        t!(;)
    ];
    let expected = ReturnStatement{
        k_return: None,
        ident: t!(ident "some_ident"),
        semi: Some(t!(;))
    };

    let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    assert_eq!(Ok(expected), result);

    let tokens = vec![
        t!(return),
        t!(ident "some_ident"),
    ];

    let expected = ReturnStatement{
        k_return: Some(t!(return)),
        ident: t!(ident "some_ident"),
        semi: None,
    };

    let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    assert_eq!(Ok(expected), result);
}
