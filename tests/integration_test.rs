use rust_book;
// import library




// They can only call public API functions however you can expose internal functions
// if you want to test against those; however this is however you want to go about it.


// notice also we dont annotate here with  #[cfg(test)]
// cargo treats the tests directory specially and compiles files in this directory

// you can also run the tests by name in any integration test file
// cart test --test integration_test
#[test]
fn it_adds_two() {
    assert_eq!(4, rust_book::add_two_other(2));
}