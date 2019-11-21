// Paths for referring to an item in the Module Tree

// A path can take two forms:

// An absolute path starts from a crate root by using a crate name or a literal crate.
// A relative path starts from the current module as uses self, super, or an identifier in the
// current module.

// Both absolute and relative paths are followed by one or more identifiers separated by
// double colons ::

// Modules are useful for organization and define Rust's Privacy boundaries.
//
// All functions, methods, strucs, enums, modules, and constants are private
// by default.

// Though children can use ancestors parents can't use children.

// front of house is sibling to eat_at_restaurant so they are callable without pub
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Staring relative paths with super keyword like the .. syntax in cmd

fn serve_order() {}

mod back_of_house {
    fn fix_inncorrect_order() {
        cook_order();
        super::serve_order(); //currently in back_of_house so super is crate level
    }

    fn cook_order() {}
}

// Structs example
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// Enums the same

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Bringing Paths into Scrop with the use keyword

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// idiomatic use paths

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


// creating aliases with the as keyword

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {

}


fn function2() -> IoResult<()> {

}

// Re-exporting names with pub use
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// combined pu use here so it is now exported at hosting level
// that is we would have access to hosting externally
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// basic packages in cargo toml
// add dependancy 

// no longer need extern crate
// IE

// cargo.toml

[dependencies] 
rand = "0.5.5"

use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);
}

use std::collections::HashMap;

//  Or clean up use paths with nested paths

use std::io;
use std::cmp::Ordering;

// vs

use std::{cmp::Ordering, io};

// Glob operator 
// brings in everything into scope

use std::collections::*;

// also used as part of the prelude pattern as described in the std library docs


// Separating Modules into Different Files

// THIS block is assumed to be in src/lib.rs
mod front_of_house; // mod xyz ; where the ; tells rust to load from another file witht he same name.

pub use create::front_of_house::hosting; // notice this is made public 

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// This is assumed to be in src/front_of_house.rs

pub mod hosting {
    pub fn add_to_waitlist() {};
}

