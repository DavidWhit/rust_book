use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

pub fn recoverable_errors_with_result() {
    // you can unwind the stack which is costly or
    // you can set abort immediately and let the OS clean up
    // via cargo.toml

    /* [profile.release]
       panic='abort'
    */

    // recall chapter 2 enum Result<T,E> {
    //     Ok(T),
    //     Err(E)
    // }
    // where T is the type and E is the type of error

    // opens but will not have available write permissions
    // let f = File::open("hello.txt");

    // Follow the builder pattern
    let y = OpenOptions::new().write(true).open("hello.txt");
    //https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open
    // ** NOTE you wouldn't do this and then call match also it would cause a move error in the latter match
    // for demonstraition commenting this out
    // check for error specifically
    // if let Err(r) = f {
    //     println!("{:?}",r);
    // }

    // or match and do something in either case
    // prefix _ to the variable because we know that it will not be used
    // in this case.
    // match f {
    //     Ok(_file) => println!("Do something with the file"),
    //     Err(e) => println!("{:?}",e)
    // }

    // Matching on Different Errors
    let mut y = match y {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

    // Actually blocked by Norton Antivirus from writing
    if let Err(x) = y.write_all(b"some byte array") {
        println!("{:?}", x.kind());
    }

    // force a permission panic on f
    // first unwrap expects the file to exist
    // second expects the write to succeed
    // f.unwrap().write_all(b"askdfjskldjf").unwrap();
    // expect will throw message into panic! result
    //f.unwrap().write_all(b"askdfjskldjf").expect("Failed to write to file");

    // shortcuts for propagating errors via ? operator

    // ********** consider this example ***********
    fn read_username_from_file() -> Result<String, Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // Changed to this example using ? to propagate
    fn read_username_from_file_short() -> Result<String, Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // And even shorter
    fn read_username_from_file_shorter() -> Result<String, Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    // And even shortest
    fn read_username_from_file_shortest() -> Result<String, Error> {
        std::fs::read_to_string("hello.txt")
    }

    if let Ok(result) = read_username_from_file() {
        println!("success {}", result);
    }
    if let Ok(result) = read_username_from_file_short() {
        println!("success {}", result);
    }
    if let Ok(result) = read_username_from_file_shorter() {
        println!("success {}", result);
    }
    if let Ok(result) = read_username_from_file_shortest() {
        println!("success {}", result);
    }

    // if you tried to call this in the main function
    // it would error because it isn't a return type
    // of result or std::ops::Try

    // One valid return type for main is () and conveniently another
    // valid return type is Result<T,E>
    //
    // fn main() -> Result<(), Box<dyn Error>> { Ok(())}

    /*
    Box<dyn Error> type is called a trait object discussed in Chapter 17
    For now think of it as any type of Error. Then using that is type is allowed.


    Panic or unwrap() expect() are ways to mark tests for things that shouldn't
    happen. unwrap() is usually a good indicator that something needs to be more robust
    later; however, it would also be appropriate to call unwrap when you have some other logic
    that ensure the Result will have an Ok value. That is whatever operation you're calling has the
    possibility of failing in general, even though it's logically impossible in your
    particular situation.

    If you can ensure by manually inspect the code that you'll never have an Err variant
    it's perfectly acceptable to call unwrap IE

    use std::net::IpAddr;

    we already know that to be valid so calling unwrap() without anything catching is
    considered acceptable

    let home: IpAddr = "127.0.0.1".parse().unwrap();
    however having a hardcoded valid string doesnt change the return type of the parse method
    we still get a Result value and the compiler will still enforce handling it as a Result wiht
    an Error variant


    It is advisable to have the code panic when the code could end up in a bad state.
    By bad state: when some assumption, guarante, contract, or invariant has been broken.
    These are such as values, contradictory values, missing values are passed to the code.

    plus one or more of the following:
        The bad state is not something that’s expected to happen occasionally.
        Your code after this point needs to rely on not being in this bad state.
        There’s not a good way to encode this information in the types you use.


    */
}
