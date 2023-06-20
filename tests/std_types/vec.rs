use astray_core::*;
use astray_macro::{set_token, SN};

set_token!(Token);

#[derive(SN, Clone, PartialEq, Debug)]
pub struct ReturnStatement {
    #[pattern((Token::LiteralInt(_),Token::SemiColon))]
    k_return: Vec<(Token,Token)>,
}


fn main() {
    empty();
    one_out();
    exactly_right();
}

fn empty() {

    let tokens = vec![t!(ident "Hello")];

    let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    let expected = ReturnStatement {
        k_return: vec![]
    };
    assert_eq!(Ok(expected), result);
}

fn one_out() {
    let tokens = vec![
        t!(litint 4),
        t!(;),
        t!(litint 3),
        t!(;),
        t!(litint 2),
    ];

    let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    let expected = ReturnStatement {
        k_return: vec![
            (t!(litint 4),t!(;)),
            (t!(litint 3),t!(;)),
        ]
    };
    assert_eq!(Ok(expected), result);
}

fn exactly_right() {
    let tokens = vec![
        t!(litint 4),
        t!(;),
        t!(litint 3),
        t!(;),
    ];

    let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    let expected = ReturnStatement {
        k_return: vec![
            (t!(litint 4),t!(;)),
            (t!(litint 3),t!(;)),
        ]
    };
    assert_eq!(Ok(expected), result);
}
