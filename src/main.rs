/*

  http://doc.rust-lang.org/rust-by-example/mod/split.html

  *** For mods to appear in cargo docs they need to be defined in lib.rs ***
  It is also much cleaner to have them all in lib or use workspaces.

// ***************************************************************
// when you use crate leading keyword that can see any MOD in main and lib
// files.
// The binary name root in this case rust_book:: can see any modules outside the main and lib
// This effects what cargo doc will show as well
// ***************************************************************

examples of different types of imports
mod chapters;

A path can take two forms:
1.An absolute path starts from a crate root by using a crate name or a literal crate.
2.A relative path starts from the current module and uses self, super, or an identifier in
    the current module.

-- I feel like you can do this in one of two ways that make sense.
   1. If making a lib add all mods you intend to import to it.
   2. Just for code splitting in main use mod mod_name then nest or expand just like lib
      similar to trial2

this will pull in files or mod.rs under the directory matching the name
 */
mod mod_test;
//src dir file points to mod1 matching the name *** This shouldn't be common ***
mod mod_test2;
// directory matching name with mod.rs
mod mod_test3;
// directory matching name with mod.rs that points to inner mod
mod async_rust; // directory matching name with mod.rs

/*
 If the module is defined in lib then you can use cargo toml package_name to import directly:
 IE -- rust_book::mod_name::mod_func_etc

 This one isn't necessary because we can import directly with use because it exists in lib.
 mod trial2;
*/

/*
*** note I couldn't say rust_book::mod_test3 because it's not in main or lib
    thus I had to declare the mod for scope above and then use it.

    For all mod tests you could just say mod and then prefix everything
    but for most cases you would have several imports so to avoid all the prefixes use what
    you are going to.
*/
use mod_test::mod1::test_string;
use mod_test3::blah::blah;
// import as alias
use mod_test2::test_string as ch2test;
// additional learning https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html
use async_rust::chapter_1::chp_1_3_async_await_primer;

// all rust_book mods exists in lib
use rust_book::trial::some_included_lib_function;
use rust_book::trial2::chapters::chapters_9::nesting_mods;
use rust_book::trial2::{another_mod_test, call_it};
// import all from chapters mod
use rust_book::chapters::*;

use chapter_10::{
    traits_defining_shared_behavior, using_generic_data_types, validating_references_with_lifetimes,
};
use chapter_3::chapter3_1::vars_and_mutability;
use chapter_3::{control_flow, data_types, functions};
use chapter_4::*;
use chapter_5::{defining_and_init_structs, using_structs_and_method_syntax};
use chapter_6::{enum_and_flow_control, enums};
use chapter_8::{working_with_hashmap, working_with_vectors};
use chapter_9::recoverable_errors_with_result;
// Chapter 11 is all included in lib.rs
use chapter_12::project;
use chapter_13::{closures, iterators};
// 14 was read only
use chapter_15::{
    running_code_on_cleanup_with_drop_trait, smart_pointers, treating_sp_as_reg_refs_with_deref,
};
use chapter_15_4_thru_6::{
    interior_mutability_pattern, reference_count_smart_pointer, reference_cycles_can_leak,
};
use chapter_16::{fearless_concurrency, using_msg_passing_to_trns_data_btwn_threads};
use chapter_17::{characteristics_of_oop, encoding_states_and_behavior_as_types};
use chapter_18::patterns_and_matching;
use chapter_19::advanced_features;
use rust_book::other_smart_pointers::{cow, trial_cow_obj_destruct_match};

fn block_print_chap(title: &str, chapter: &str) {
    println!("\n*****************************************************************************");
    println!("CHAPTER {} \n{}", chapter, title);
    println!("*****************************************************************************\n");
}

fn main() {
    let x: &str = test_string();
    let y: &str = ch2test();
    some_included_lib_function();
    another_mod_test();
    call_it();
    println!("{}, {}", x, y);

    // Chapter 3
    block_print_chap("Common Programming Concepts", "3");
    vars_and_mutability();
    data_types();
    functions();
    control_flow();

    // Chapter 4
    block_print_chap("Understanding Ownership", "4");
    ownership();
    references_and_borrowing();
    slice_type();

    // Chapter 5
    block_print_chap("Using Structs to Structure Related Date", "5");
    defining_and_init_structs();
    using_structs_and_method_syntax();

    // Chapter 6
    block_print_chap("Enums and Pattern Matching", "6");
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
    block_print_chap("Enums and Pattern Matching", "8");
    working_with_vectors();
    working_with_hashmap();

    // CHAPTER 9 ERROR Handling
    block_print_chap("Error handling", "9");
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
    block_print_chap("Generic Types, Traits, and Lifetimes", "10");
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

    // Chapter 12 project
    block_print_chap("An I/O Project: Building a Command Line Program", "12");
    // turned this off to avoid passing search string and file name
    // project();

    // Chapter 13
    block_print_chap("Functional Language Features: Iterators and Closures", "13");
    closures();
    iterators();

    //Chapter 14
    // changed to a mod just read

    //Chapter 15
    block_print_chap("Smart Pointers", "15");
    smart_pointers();
    treating_sp_as_reg_refs_with_deref();
    running_code_on_cleanup_with_drop_trait();
    reference_count_smart_pointer();
    interior_mutability_pattern();
    reference_cycles_can_leak();

    //Chapter 16
    block_print_chap("Fearless Concurrency", "16");
    fearless_concurrency();
    using_msg_passing_to_trns_data_btwn_threads();

    //Chapter 17
    block_print_chap("OOP Features of Rust", "17");
    characteristics_of_oop();
    encoding_states_and_behavior_as_types();

    //Chapter 18
    block_print_chap("Patterns and Matching", "18");
    patterns_and_matching();

    //Chapter 19
    block_print_chap("Advanced Features", "19");
    advanced_features();

    // Async Primer
    chp_1_3_async_await_primer();

    // other smart pointers
    cow();
    trial_cow_obj_destruct_match();


}
