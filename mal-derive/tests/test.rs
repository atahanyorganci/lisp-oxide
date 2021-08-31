#[test]
fn builtin_func_test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/builtin_func/01-parse.rs");
    t.pass("tests/builtin_func/02-outer-fn.rs");
    t.pass("tests/builtin_func/03-mal-type.rs");
    t.pass("tests/builtin_func/04-rc-ref.rs");
    t.pass("tests/builtin_func/05-array.rs");
    t.pass("tests/builtin_func/06-mixed-array.rs");
    t.pass("tests/builtin_func/07-env.rs");
    t.pass("tests/builtin_func/08-attr.rs");
    t.compile_fail("tests/builtin_func/09-attr-fail.rs");
    t.pass("tests/builtin_func/10-option.rs");
}
