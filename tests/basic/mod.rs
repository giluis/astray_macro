use astray_core::*;
use astray_macro::{SN, set_token};

set_token!{Token}

#[derive(SN, Debug, PartialEq)]
pub struct Identifier {}

fn main() {

    // println!("Compiled");
    let tokens = vec![
        t!(ident "some_ident")
    ];
    let result = Identifier::parse(&mut TokenIter::new(tokens));
    assert!(result.is_ok())

}
