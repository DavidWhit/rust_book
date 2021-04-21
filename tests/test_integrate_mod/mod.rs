use rust_book;


pub fn integrate_funcs() {

    let greet = rust_book::greeting("David");
    let result = format!("{} {}", greet, rust_book::add_two_other(3));

    assert_eq!("Hello! David 5".to_string(), result)
}