
fn main() {
//    Chapter 3 variables

    // LET vars ****************

    // the type is inferred from the right side
    // intellij shows the inferred type here
    let a = 5;

    // default values are immutable thus
    // this would fail to compile a = 8;

    // you need to prefix the var mut to make it mutable
    let mut y = 10;
    y = 15; // This is ok

    // Constant vars *************

    // Constants cant be mut
    // Constants by always be type annotated
    const MAX_POINTS: u32 = 100_000;

    // Note that we can use _ for readability

    // Shadowing vars **********

    // lets you reassign immutable replacing it
    // in effect transforming the same value into a new immutable one.
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("the value of x is: {}",x);

    // some other key differences
    // because we are technically creating a new variable
    // using let different types are allowed

    let y = "Some string";
    let y = y.len();

    println!("the value of y is: {}",y);

    // ** you could not do the above with mut
    // because your not creating a new value

    // 3.2 Data types

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
    let float_32 : f32 = 3.0;

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
    let tup: (i32,f64,u8) = (500,6.4,1);
    // can be accessed by destructuring
    let (_,e,_) = tup;
    // or access by position begins with 0
    println!("{} {}",e, tup.2);

    //Arrays
    // Great for allocating data on the stack
    // vs the heap for known const data or when you want to ensure
    // you have a fixed number of elements.
    // literal
    let a = [1,2,3,4,5];
    // creat an array with 3, 5times
    let a = [3;5];
    // accessing an index out of range will not be caught at compilation
    // however during runtime it will panic preventing invalid memory being accessed

    // 3.3 Functions
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
        println!("x val inside block is: {}", x );

        let x = 3;
        // this is left with the ; off because we are saying this is an
        // expression and not a statement.
        // recall if we added do x+1; recall statements don't return anything
        // causing an error.
        x + 1
    };

    // we haven't shadowed x outside the block since the last 10
    // because y block {} created a new scope so x is 10 not 3
    println!("x val outside block is: {}", x );
    println!("the value of y is {}", y);

    // 3.4 Comments DUH begin with //
    // 3.5 Control flow

    // Basic control structures like other langs
    let number = 9;

    // condition must be a boolean expression
    // exits the block on first true condition
    // you can add this as a statement by adding;
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
            break counter * 2
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
    let a = [10,2,1,23,22,33,44];

    for val in a.iter() {
        println!("{}", val);
    }

    // range example -- in rust this is inclusive at the begin and
    // not at the end that is 1..4 is 1-3 inclusive
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    //Chapter 4.1 Ownership
    // Each value in rust has a variable that's called its owner.
    // there can only be one owner at a time
    // when the owner goes out of scope the value will be dropped.

    // variable scope.
    {  // begin scope
        let s = "hello block"; // s is valid
        println!("{}",s);
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

}


// 3.3 function declaration
// uses snake case
// can be at the top or bottom outside of main
// example
fn another_function (x: i32, y: i32) -> i32 {
    x + y
}