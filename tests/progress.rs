#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic/mod.rs");

    // TODO: impl stateful extraction from leafs
    t.pass("tests/struct/from_token.rs");
    t.pass("tests/struct/other_parsable.rs");
    t.pass("tests/struct/all_together.rs");

    t.pass("tests/enum/one_variant.rs");
    t.pass("tests/enum/many_variants.rs");
    t.pass("tests/enum/ref_to_enum.rs");
    t.pass("tests/enum/enum_and_struct.rs");
    t.pass("tests/enum/many_unit_variants.rs");

    t.pass("tests/std_types/optional.rs");
    t.pass("tests/std_types/vec.rs");
    t.pass("tests/std_types/box.rs");
}
