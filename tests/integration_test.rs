use rust_book;
// import library


// can have separate testing modules
mod test_integrate_mod;

//Files in subdirectories of the tests directory don’t get compiled as separate crates or have
//sections in the test output.
// That is:
/*
To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create
tests/common/mod.rs. This is an alternate naming convention that Rust also understands. Naming the
file this way tells Rust not to treat the common module as an integration test file. When we move
the setup function code into tests/common/mod.rs and delete the tests/common.rs file, the section
in the test output will no longer appear. Files in subdirectories of the tests directory don’t get
compiled as separate crates or have sections in the test output.
 */


// You can't create integrations without having a src/lib.rs
// to import your bin crate.


// They can only call public API functions however you can expose internal functions
// if you want to test against those; however this is however you want to go about it.


// notice also we dont annotate here with  #[cfg(test)]
// cargo treats the tests directory specially and compiles files in this directory

// you can also run the tests by name in any integration test file
// cart test --test integration_test
#[test]
fn it_adds_two() {
    test_integrate_mod::integrate_funcs();
    assert_eq!(4, rust_book::add_two_other(2));
}
