#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.pass("tests/02-outer-fn.rs");
    t.pass("tests/03-mal-type.rs");
    t.pass("tests/04-rc-ref.rs");
    t.pass("tests/05-array.rs");
    t.pass("tests/06-mixed-array.rs");
    t.pass("tests/07-env.rs");
    t.pass("tests/08-attr.rs");
}
