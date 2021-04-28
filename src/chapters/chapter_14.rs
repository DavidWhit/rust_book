//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
pub fn cargo_chapter14() {

    // cargo has two main profiles are predefined and customizable

    // 1. dev by cargo build
    // 2. release by cargo build --release

    // cargo has default settings for each of the profiles
    // there aren't any profile.*
    //
    // If you want to customize you can override any subset
    // default in the cargo.toml
    //
    // default values for the opt-level cargo.toml
    // [profile.dev]
    // opt-level = 0


    // [profile.release]
    // opt-level = 3


    // The opt-level controls the number of optimizations Rust
    // applies slowing compilation and explains the default values.
    // opt-level has a range of 0-3


    // *** making useful documentation comments
    // IE  some src/lib.rs USE THREE forward slash
    // Somewhat like github md. files etc..

    /// Adds one to the number given.
    /// # Examples
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x:i32) -> i32 {
        x + 1
    }

    // see https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html
    // for what the above generates
    // cargo doc --open will build the html
    // Beyond examples section header
    // crate creators may include
    // Panics, Errors if fun returns result what type of errors, Safety if (unsafe is used)
    // not a must but a general idea

    // cargo test will run code examples in Example code documentation blocks

    // The below adds a documentation to the item that contains the comments
    // check top

    // *** Setting up a Crates.io account
    // cargo login "somekey"

    // *** Adding Metadata to a New Crate
    // see cargo toml

    // *** When licensing is setup just publish when ready with
    // cargo publish

    // *** Publishing a new version of an existing crate
    // you change the version value specified in your Cargo.toml file and republish
    // following semantic versioning then publish again.


    // *** Removing versions from crates.io with cargo yank.
    // To yank a version of a crate
    // cargo yank --vers 1.0.1
    // Or undo a yank
    // cargo yank --vers 1.0.1 --undo
    // ** yank doesn't delete it just keeps people from basing off that version.



    // *** Creating a Workspace

    // A workspace is a set of packages that share the same cargo.lock and output directory.
    // mkdir add  cd add
    // then in cargo.toml in the changed to dir add
    // use [workspace] instead of [package]

    // [workspace]
    /*
        members = [ "adder",]
     */

    // Then create the adder binary by running cargo new within the add directory
    // Then building the workspace by cargo build
    // then the file structure will look like
    /*
    ├── Cargo.lock
    ├── Cargo.toml
    ├── adder
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    └── target
     */


    // Create the second package in the workspace
    // updating members from above
    // members = [ "adder","add-one"]

    // Then run cargo new add-one --lib
    /*
    |── Cargo.lock
    ├── Cargo.toml
    ├── add-one
    │   ├── Cargo.toml
    │   └── src
    │       └── lib.rs
    ├── adder
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    └── target


    pub fn add_one(x: i32) -> {
        x + 1
    }

    filename: adder/cargo.toml

    [dependencies]
    add-one = {path = "../add-one"}

    filename: adder/src/main.rs
    use add_one;
    fn main() {
        let num = 10;
        println!(
            "Hello, world! {} plus one is {}!",
            num,
            add_one::add_one(num)
       );
    }

    then run cargo build in the top directory of add
    use -p to call the package you want to run
    then call cargo run -p adder
    */

    // crate dependancies are still independant of each other but
    // cargo will manage them so that only 1 is ever downloaded
    

}