/*
Putting to use concepts learned in chapters
7,8,9,10,11
 */

use std::env;
use std::process;

use rust_book::chapter12_lib::{Config, run};

/*
The args Function and Invalid Unicode
Note that std::env::args will panic if any argument contains invalid Unicode. If your program needs
to accept arguments containing invalid Unicode, use std::env::args_os instead. That function
returns an iterator that produces OsString values instead of String values. We’ve chosen to use
std::env::args here for simplicity, because OsString values differ per platform and are more
complex to work with than String values.
 */

/*

Separation of Concerns for Binary Projects
The organizational problem of allocating responsibility for multiple tasks to the main function is
common to many binary projects. As a result, the Rust community has developed a process to use as
a guideline for splitting the separate concerns of a binary program when main starts getting large.
The process has the following steps:

Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
As long as your command line parsing logic is small, it can remain in main.rs.
When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
The responsibilities that remain in the main function after this process should be limited to the following:

Calling the command line parsing logic with the argument values
Setting up any other configuration
Calling a run function in lib.rs
Handling the error if run returns an error
 */




/*
The Trade-Offs of Using clone
There’s a tendency among many Rustaceans to avoid using clone to fix ownership problems because of
its runtime cost. In Chapter 13, you’ll learn how to use more efficient methods in this type of
situation. But for now, it’s okay to copy a few strings to continue making progress because you’ll
make these copies only once and your filename and query string are very small. It’s better to have
a working program that’s a bit inefficient than to try to hyperoptimize code on your first pass.
As you become more experienced with Rust, it’ll be easier to start with the most efficient solution
, but for now, it’s perfectly acceptable to call clone.
 */

/*
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {
        query, filename
    }
}
 This is moved into a constructor
*/



// pretending this is our main
pub fn project() {
    let args: Vec<String> = env::args().collect();

    // here we need to know the ok or err value
    // so we used unwrap_or_else with the closure of the err message.

    // eprintln! prints to stderr
    // write with run > output.txt
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // now we can just use a block on if let
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}