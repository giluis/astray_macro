## TODO
-[X] Box
-[X] Vec
-[ ] Functional Aproach
-[ ] Enums with fieldless variants (check Operator @ tests/std_types/box.rs)
-[ ] Proper documentation



## What is generated
// /**
//  *  impl Parsable<#Token> for #Type {
//  *      fn parse(iter: &mut TokenIter) -> Result<#Type, ParseError<#Token>> {
//  *          (
//  *          // in case it is a struct Node
//  *          let #field_name = iter.parse().map_err(|err| ParseError::from_conjunct_error(err))?;
//  *          // in case it is an enum Node
//  *          let #field_name ## _err = iter.parse()?.map(|result: #field_type |#Type::#field_name(result)).hatch()?;
//  *          ) * // repeat for each field
//  *
//  *          // if struct Node
//  *          Ok(#Type {#(#field_name)*})
//  *          // else if enum Node
//  *          Err(ParseError::from_disjunct_errors(#(#field_name##_err)*))
//  *      }
//  * }
// */