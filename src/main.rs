fn main() {
    // +++++++++++++++++++++ Chapter 3 variables and Mutability +++++++++++++++++++++

    // LET vars ****************

    // the type is inferred from the right side
    // intellij shows the inferred type here
    let _a = 5;

    // default values are immutable thus
    // this would fail to compile a = 8;

    // number defaults to i32 and a prefix _ to the var says ignore that its not used elsewhere
    // you need to prefix the var mut to make it mutable
    let mut _y = 10;
    _y = 15; // This is ok

    // Constant vars *************

    // Constants cant be mut
    // Constants must always be type annotated
    const MAX_POINTS: u32 = 100_000;

    println!("Numbers with underscores for read {}", MAX_POINTS);

    // Note that we can use _ for readability

    // Shadowing vars **********

    // lets you reassign immutable replacing it
    // in effect transforming the same value into a new immutable one.
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("the value of x is: {}", x);

    // some other key differences
    // because we are technically creating a new variable
    // using let different types are allowed

    let y = "Some string";
    let y = y.len();

    println!("the value of y is: {}", y);

    // ** you could not do the above with mut
    // because your not creating a new value

    // +++++++++++++++++++++ 3.2 Data types +++++++++++++++++++++

    // scalar types Rust has 4 primary
    // integers
    // floats
    // booleans
    // chars

    // Integers
    //        signed | unsigned variants
    // 8-bit   i8       u8
    // 16-bit  i16	    u16
    // 32-bit  i32	    u32
    // 64-bit  i64	    u64
    // 128-bit i128	    u128
    // arch	   isize	usize

    // ** the arch stands for architecture
    // so 32bit or 64bit
    // The primary situation you would use these
    //  is when indexing some sort of collection.

    //Integer literals
    //    Number literals	Example
    //    Decimal	        98_222
    //    Hex	            0xff
    //    Octal	            0o77
    //    Binary	        0b1111_0000
    //    Byte (u8 only)	b'A'

    // Compiler notes on integer overflow
    // when compiling without the --release option
    // and say a u8 is trying to assign 256 which is over 0-255
    // then integer overflows that type and panics

    // if you are in --release mode rust doesn't
    // include checks for integer overflow that cause panics.
    // instead if it occurs Rust does twos complement wrapping
    // IE 257 becomes 1 and so on. This will give wrong results
    // clearly; but will prevent the program from panics

    // Floats
    // Basically two types
    // f32 and f64
    // f32 has single precision
    // f64 has double precision

    // unlike integers floats default to f64
    let float_64 = 3.4;
    let float_32: f32 = 3.0;

    // Numeric Operations
    // standard
    // + - * / and Mod %

    // Boolean
    // 1 byte in size;
    // default is true;

    // Character
    // 4 byte size
    // wrapped in single quotes like other langs
    // unicode supported

    // Compound types
    // Tuples and Arrays

    // Tuples
    // offer a way to group multiple types into one type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // can be accessed by destructuring
    let (_, e, _) = tup;
    // or access by position begins with 0
    println!("{} {}", e, tup.2);

    //Arrays
    // Great for allocating data on the stack
    // vs the heap for known const data or when you want to ensure
    // you have a fixed number of elements.
    // literal
    let a = [1, 2, 3, 4, 5];
    // creat an array with 3, 5times
    let a = [3; 5];
    // accessing an index out of range will not be caught at compilation
    // however during runtime it will panic preventing invalid memory being accessed

    // +++++++++++++++++++++ 3.3 Functions +++++++++++++++++++++
    // no support for default function arguments
    let result = another_function(32, 89);
    println!("result is: {}", result);

    // Functions contain statements and expressions
    // and understanding the difference
    // Statements are instructions that perform some action
    //       and do not return a value.
    // Expressions evaluate to a resulting value
    let y = 6; // this is a statement
               // calling a function
               // calling a macro
               // use {} to create a block are all 3 expressions
               // expressions also do not end in ;

    let x = 10;
    let y = {
        // a block was created making a new scope but x is still 10 here
        println!("x val inside block is: {}", x);

        let x = 3;
        // this is left with the ; off because we are saying this is an
        // expression and not a statement.
        // recall if we added do x+1; recall statements don't return anything
        // causing an error.
        x + 1
    };

    // we haven't shadowed x outside the block since the last 10
    // because y block {} created a new scope so x is 10 not 3
    println!("x val outside block is: {}", x);
    println!("the value of y is {}", y);

    // 3.4 +++++++++++++++++++++ Comments DUH begin with // +++++++++++++++++++++
    // 3.5 +++++++++++++++++++++ Control flow +++++++++++++++++++++

    // Basic control structures like other langs
    let number = 9;

    // condition must be a boolean expression
    // exits the block on first true condition
    // you can add this as a statement by adding ';'
    // so you could assign it to a let var
    // however each if case would need to be of the same type.
    let result = if number < 5 {
        "the number was less than 5"
    } else if number > 5 {
        "the number was > than 5"
    } else if number > 6 {
        "the number was > than 6"
    } else {
        "the number is 5"
    };

    println!("The resulted if was:   {}", result);

    // Basic looping
    // loop -- forever or until control statement IE break

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // sorta confused here as this works
            // as an expression or statement.
            break counter * 2;
        }
    };

    println!("The resulted is 20:   {}", result);

    // likewise with a while loop allows for cleaner control vs
    // loop and break but infinite loops have purposes.
    counter = 3;

    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }

    // for keyword
    // obviously a better mechanic than using length and index counter
    let a = [10, 2, 1, 23, 22, 33, 44];

    for val in a.iter() {
        println!("{}", val);
    }

    // range example -- in rust this is inclusive at the begin and
    // not at the end that is 1..4 is 1-3 inclusive
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // +++++++++++++++++++++ Chapter 4.1 Ownership +++++++++++++++++++++

    // Each value in rust has a variable that's called its owner.
    // there can only be one owner at a time
    // when the owner goes out of scope the value will be dropped.

    // variable scope.
    {
        // begin scope
        let s = "hello block"; // s is valid
        println!("{}", s);
    } // scoped ended s is no longer valid

    // All previously covered data types are stored on the stack
    // when their scope is over they are popped off the stack.

    // But we want to look at data that is stored on the heap
    // and explore how rust knows when to clean up that data.

    // str literals hardcoded are immutable so we need to reach out
    // to the more complex String type which is heap allocated.

    // :: operator that allows us to namespace this particular function "from"
    // under the String type
    let s = String::from("hello");

    // this type of string can be mutated.
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{}", s);

    // these above two types (str literal "", String) deal with memory differently
    // in the case of the literal we know the contents at compile time so the text
    // is hardcoded directly into the final executable.

    // There is a natural point at which we can return the memory our String needs
    // to the operating system when s goes out of scope. Rust calls a special
    // function for us called drop and is where the author of String can put the code
    // to return the memory. Rust calls drop automatically at the closing curly bracket.

    // This is similiar to C++ patter of deallocating resources at the end
    // of an item's lifetime is sometimes called Resource Acquisition Is Initialization or
    // RAII

    /*

    This pattern has a profound impact on the way Rust code is written. It may seem simple right
    now, but the behavior of code can be unexpected in more complicated situations when we want to
    have multiple variables use the data we’ve allocated on the heap. Let’s explore some of those
    situations now.

    */

    /***** exploring this ******/

    let s1 = String::from("hello");
    let s2 = s1;

    // we would assume a copy of the value in s1 is bound to s2
    // but this isn't what happens.

    // A String is made up of three parts, shown on the left
    // stored on the stack         contents on the right is on the heap
    // s1                               index    value
    // 1. pointer    --------------->   0        h
    // 2. length     5                  1        e
    // 3. capacity   5                  2        l
    //                                  3        l
    //                                  4        o

    // length is how much memory in bytes the contents of String is currently using
    // capacity is the memory in bytes created by the OS
    // when we assign s1 to s2 the String data is copied meaning that copied
    // the stack values which includes the address location
    // meaning s1 and s2 point to the same string

    // If we didnt copy the stack and copied the heap you can see why the
    // runtime performance could be very expensive.

    // How this impacts scope if s2 or s1 goes out of scope and rust
    // calls drop what happens to the other ?
    // This is known as double free error

    // In Rust this is handled by invalidating the original s1.
    // So now Rust doesn't do any clean up if s1 goes out of scope.
    // Also why this is called a mov from s1 to s2 and s1 is killed

    // This errors because it is invalid -- borrowed after moved
    // println!("{}",s1);

    // Typically understood as a shallow copy ** what we just did ** (of the stack)  and a deep copy (complete copy)
    // Though because rust invalidates the old one it's call a move
    // we would say s1 moved to s2 and s1 is no longer used.

    // The Rust language design choice is that it will never auto create "deep copies"
    // So if we wanted a deep copy we would use .clone()

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {} | s2 = {}", s1, s2);

    // Stack-Only Data: Copy
    let x = 5;
    let y = x;

    // This seems to contradict what was just said but that is because these values are on
    // the stack and these values are known at compile time. So there isn't a reason
    // to drop x after creating y because there is no difference in a deep or shallow
    // copy so calling clone wouldnt do anything different.

    // Rust has a special annotation called the copy trait where we can place types
    // like integers on the stack ** see chapter 10
    // mutually exclusive -- if you have a drop trait you cant have a copy trait
    // drop traits are on heap
    // copy traits are on stack

    // General Rule
    // any group of simple scalar values can be Copy and nothing that requires allocation
    // or is some form of resource is Copy. Here are some of the types that are Copy

    // All integers, boolean, floats, char, tuples *if they only contain Copy trait
    // implemented types
    println!("x = {}, y = {}", x, y);

    // ************* Ownership and Functions *************

    // The same semantics for passing a value to a function are similar to those
    // for assiging a value to a variable. That is...
    // passing a variable with a drop trait will cause a move otherwise copy

    let s = String::from("hello");
    takes_ownership(s);

    // s is not out of scope because it was moved
    // println!("trying to call S will boom! {}", s);

    let x = 5;

    makes_copy(x);

    println!("X is still here in scope: {}", x);

    // x is still in scope because it was copied.

    // ************* Return Values and Scope *************

    //
    let s1 = gives_onwership(); // function gave ownership via return val
    let s2 = String::from("hello"); // set s2 on heap
    println!("s2 -> {} original address: {:p}", s2, &s2);
    let s2 = takes_and_gives_back(s2); // sent s2 to be modified and return back with concat
    println!("s2 -> {} address: {:p}", s2, &s2); // you can see this is a new address
    let s3 = takes_and_gives_back(s2); // moved s2
                                       // s2 is now out of scope
                                       // s3 holds final transform of s2
                                       // no mutation has occured because you are returning a new value
                                       // seen by the address changes
    println!("s3 -> {} address: {:p}", s3, &s3);

    // recap the ownership of a variable follows the same pattern every time
    // assiging a value to another variable moves it
    // when a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop
    // unless the data has been moved by another variable.

    // This is obviously extremely tedious
    // you could do things like tuple returns but references are best

    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);

    println!("s1 {} and length {}", s1, len);

    // ************* 4.2 References and Borrowing *************

    // As we just witnessed in the calculate_length fn above we had to use another variable
    // in order to get the length otherwise a borrow after move error would occur
    // WHY though?
    // focus on the error it said "borrow after move"
    // with the current signature return is a tuple -> (String, usize)
    //
    // ok that must mean values are being assigned in a left to right order. So the sString has already moved
    // the value s1 and we next are trying to access on it via s.len()
    // this must mean that if we reversed the return to get the length we would have done it successfully
    // that is to say get the length first then move it.
    // change fn calculate_length return to be -> (usize, String) and see that it is fixed.
    // again extremely tedious

    // the alternative and I should say idiomatic way is to use references and borrowing

    // you must allow a mut if you want to mutate a reference passed to a function
    let mut s1 = String::from("Hello");
    // s1 wasn't moved but borrowed by accessing the reference pointing to the location in memory
    // on the heap.
    // The below fn now has param &s that points to s1 that points to a location on the heap
    // fn param &s points -> s1 points -> heap location
    // notice the different addresses between s1 and the interal s local in the function.
    let len = calculate_length_borrowing(&mut s1); // this is a mutable borrowing(&s1);
                                                   // s1 still in scope; printing
                                                   // because the function is holding a reference when it goes out of scope the value it points to will not.
    println!("address of s1 {:p} {} {}", &s1, len, s1);

    // Basic pointer symbols in most langs
    // reference    &
    // dereference  *

    // one big restriction
    // You can only have one mutable reference to particular piece of data in a particular scope.
    // This would fail:

    // let mut S = String::from("Hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // ^ this makes a bit of sense I should have only one thing referencing a value I will change
    // within any given scope.

    // This comes with the prevention of a lot of issues involving data races.
    // Typical conditions in which data races occur.
    //    Two or more pointers access the same data at the same time.
    //    At least one of the pointers is being used to write to the data.
    //    There’s no mechanism being used to synchronize access to the data. IE mutex

    // We could simply mitigate the error if we added a simple block scope

    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    } // r1 dropped

    let r2 = &mut s;

    // however

    let r1 = &s; // no problem immutable borrow
    let r2 = &s; // no problem
                 // let r3 = &mut s; // big problem mutable borrow

    // doesn't allow to mix non mutable with a immutable ref.
    // in basic terms I can have multiple readers but I cannot have readers and writers point to the same data in the same scope.

    // important to keep in mind the scope is also until the last time the reference was used
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // just need to make sure are readers are used before we make an immutable ref
                                   // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // ******************** dangling references (hints to lifetimes in chapter 10) ********************

    // Rust compiler ensures that dangling refs will not happen
    // by not allowing the data to go out of scope before the reference to the data does

    let reference_to_nothing = dangle();
    // with the fn fixed we dont error

    println!("{}", reference_to_nothing);

    // recap
    // At any given time you can have either one mutable reference or any number of immutable references.
    // references must always be valid

    // ++++++++++++ 4.3 The slice type ++++++++++++

    // Doesn't have ownership
    // they are refs to a continguous sequence of elements in a collection rather than the whole collection.

    // making a function that returns the first word in a string.
}

// 4.3
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter() returns each element in the collection
    // enumerate() wraps the result of iter and returns each element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len();
}

// ************* 3.3 function declaration *************
// uses snake case
// can be at the top or bottom outside of main
// example
fn another_function(x: i32, y: i32) -> i32 {
    x + y
}

fn takes_ownership(some_sting: String) {
    println!("{}", some_sting);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_onwership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string + " with more"
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_borrowing(s: &mut String) -> usize {
    println!("address of &s -> {:p}", &s);
    s.len()
} // s is out of scope

fn dangle() -> String {
    String::from("hello")
} // now s is out of scope and dropped the easy way to resolve this is to move it
