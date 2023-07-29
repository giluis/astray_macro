use astray_core::*;
use astray_macro::{set_token, SN};

set_token!(Token);

#[derive(SN, Debug, PartialEq)]
pub struct ArithExpr {
    left: Term,
    #[pattern((Operator::Add(_) | Operator::Sub(_), _))]
    others: Vec<(Operator, Term)>,
}

#[derive(SN, Debug, PartialEq)]
pub struct Term {
    left: Factor,
    #[pattern((Operator::Mult(_) | Operator::Div(_), _))]
    others: Vec<(Operator, Factor)>,
}

#[derive(SN, Debug, PartialEq)]
pub enum Factor {
    Expression(Box<ParenthesizedExpression>),
    #[pattern(Token::LiteralInt(_))]
    Number(Token),
    #[pattern(Token::Identifier(_))]
    Identifier(Token),
}

#[derive(Debug, PartialEq)]
pub struct ParenthesizedExpression {
    expr: ArithExpr,
}

impl Parsable<Token> for ParenthesizedExpression {
    type ApplyMatchTo = Self;

    fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>> {
        let _r: Token =
            iter.parse_if_match(|tok| matches!(tok, Token::LParen), Some("Token::LParen"))?;
        let expr = iter.parse()?;
        let _l: Token =
            iter.parse_if_match(|tok| matches!(tok, Token::RParen), Some("Token::RParen"))?;
        Ok(ParenthesizedExpression { expr })
    }
}

#[derive(SN, Debug, PartialEq)]
pub enum Operator {
    #[pattern(Token::Plus)]
    Add(Token),
    #[pattern(Token::Minus)]
    Sub(Token),
    #[pattern(Token::Mult)]
    Mult(Token),
    #[pattern(Token::Div)]
    Div(Token),
}

fn main() {
    let tokens = vec![
        t!(l_paren),
        t!(litint 1),
        t!(+),
        t!(litint 2),
        t!(r_paren),
        t!(*),
        t!(litint 3),
    ];
    let result = ArithExpr::parse(&mut TokenIter::new(tokens));
    let expected = ArithExpr {
        left: Term {
            left: Factor::Expression(Box::new(ParenthesizedExpression {
                expr: ArithExpr {
                    left: Term {
                        left: Factor::Number(t!(litint 1)),
                        others: vec![],
                    },
                    others: vec![(
                        Operator::Add(t!(+)),
                        Term {
                            left: Factor::Number(t!(litint 2)),
                            others: vec![],
                        },
                    )],
                },
            })),
            others: vec![(Operator::Mult(t!(*)), Factor::Number(t!(litint 3)))],
        },
        others: vec![],
    };
    assert_eq!(Ok(expected), result);
}
