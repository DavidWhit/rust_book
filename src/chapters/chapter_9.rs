use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::io::prelude::*;

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
    let f = File::open("hello.txt");

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
        }
    };

    y.write_all(b"askdfjskldjf").unwrap();

    // force a permission panic on f
    // first unwrap expects the file to exist
    // second expects the write to succeed
    // f.unwrap().write_all(b"askdfjskldjf").unwrap();
    
    // expect will throw message into panic! result
    f.unwrap().write_all(b"askdfjskldjf").expect("Failed to write to file");


    
}

