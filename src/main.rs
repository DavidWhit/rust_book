// examples of different types of imports
mod chapters;
mod mod_test;
mod mod_test2;
mod mod_test3;
// brings them into scope or you could use package root
// IE remove the mod scope import
// then rust_book::mod_name::mod_func_etc
// mod trial;
mod trial2;

//pulling mod into scope with use otherwise as described below we must
//prefix the mod to every call see mod_test3 below
// import as alias
use mod_test2::test_string as ch2test;
// just use the mod or use a part of it use "use" to bring it into scope
// so that you dont have to lead every call with mod::{some imports}
// use test3::test_string as ch3test;
use chapters::chapter_3::chapter3_1::vars_and_mutability;
use chapters::chapter_3::{control_flow, data_types, functions};
use mod_test::mod1::test_string;
use rust_book::trial::some_included_lib_function;
use trial2::{another_mod_test, call_it };
// import all
use chapters::chapter_10::{
    traits_defining_shared_behavior, using_generic_data_types, validating_references_with_lifetimes,
};
use chapters::chapter_4::*;
use chapters::chapter_5::{defining_and_init_structs, using_structs_and_method_syntax};
use chapters::chapter_6::{enum_and_flow_control, enums};
use chapters::chapter_8::{working_with_hashmap, working_with_vectors};
use chapters::chapter_9::recoverable_errors_with_result;

fn main() {
    //http://doc.rust-lang.org/rust-by-example/mod/split.html
    let x: &str = test_string();
    let y: &str = ch2test();
    let z: &str = mod_test3::test_string();
    some_included_lib_function();
    another_mod_test();
    call_it();
    println!("{}, {}, {}", x, y, z);

    // Chapter 3
    vars_and_mutability();
    data_types();
    functions();
    control_flow();

    // Chatper 4
    ownership();
    references_and_borrowing();
    slice_type();

    // Chapter 5
    defining_and_init_structs();
    using_structs_and_method_syntax();

    // Chapter 6
    enums();
    enum_and_flow_control();

    // **************************** Chapter 7 ************************************

    // crates either binaries or libs
    // A CRATE:
    // root is a source file that the rust compiler starts from and makes up the root
    // module of your crate.

    // A PACKAGE:
    // Is one or more crates that provides a set of functionality. A package contains a cargo.toml
    // file that describes how to build those crates

    // Cargo is name specific
    // That is when we call cargo new some_project
    // tt creates and application with a cargo.toml and a src dir
    // Also if the package directory contains src/lib.rs Cargo passes the crate root files to rustc
    // to build the lib or binary

    // Modules:
    // Allows code to be organized within a crate
    // Controls privacy of items public or private
    // Example

    // continued in restaurant project chapter 7 modules lib

    // CHAPTER 8
    working_with_vectors();
    working_with_hashmap();

    // CHAPTER 9 ERROR Handling
    // panic! macro

    // By default the panic! macro will undwind the stack walking back up the stack
    // cleaning up allocated memory; however, this is a lot of work.
    // The alternative is to just immediately abort the application making the OS clean up memory.
    // This can be done by adding [profile] section to cargo.toml
    // For example if you wanted to abort in release mode you would set
    //
    //    [profile.release]
    //    panic = 'abort'

    // in the cargo.toml file.

    // We can also force a panic via the macro
    // panic!("Crash and Burn Bro");

    // Rust helps to prevent buffer over read, as in, trying to read data in a vec that is out of bounds.

    // If we wanted to read from the backtrace we can setup the cargo command to do
    // RUST_BACKTRACE=1 cargo run
    recoverable_errors_with_result();

    // Chapter 10
    using_generic_data_types();
    traits_defining_shared_behavior();
    validating_references_with_lifetimes();

    // Chapter 11
    /*
    See lib.rs
    The bodies of tests functions typically perform these three actions.

    1. Set up any needed data or state
    2. Run the code you want to tests.
    3. assert the results are what you expect.

    */

    // *** Anatomy of a tests function
    // to change a function into a tests function add the #[tests] attribute before fn
    // run with cargo tests


    // the tests module will already be created when creating a lib
    // IE
    // cargo new adder --lib
    // cd adder

}
