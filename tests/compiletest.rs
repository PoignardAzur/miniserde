#[rustversion::attr(not(nightly), ignore)]
#[cfg_attr(miri, ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[rustversion::attr(not(nightly), ignore)]
#[cfg_attr(miri, ignore)]
#[test]
fn ui_venial() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui-venial/*.rs");
}
