pub fn ownership() {
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
}

pub fn references_and_borrowing() {
    fn calculate_length_borrowing(s: &mut String) -> usize {
        println!("address of &s -> {:p}", &s);
        s.len()
    } // s is out of scope

    fn dangle() -> String {
        String::from("hello")
    } // now s is out of scope and dropped the easy way to resolve this is to move it

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
}

pub fn slice_type() {
    // 4.3

    fn first_word_redo(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }

    // extremely brittle and error prone thus slices are introduced to avoid logic that trys to maintain sync
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        // iter() returns each element in the collection
        // enumerate() wraps the result of iter and returns each element tuple of (index, ref) (mainly for getting the index)
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                // byte literal syntax
                return i;
            }
        }

        s.len()
    }
    // ++++++++++++ 4.3 The slice type ++++++++++++

    // Doesn't have ownership
    // they are refs to a continguous sequence of elements in a collection rather than the whole collection.

    // making a function that returns the first word in a string.
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // s is now empty but word still holds 5

    // compiles still but there is no lenght of 5 in s
    // changing the return signature to usize, usize for start and end is even more error prone and requires more
    // to keep in sync

    // Enter Slices
    // string slices are reference to part of a string

    let s = String::from("Hello World");

    let hello = &s[0..5]; // zero index and index based noninclusive for the top value 0-4
    let world = &s[6..11];

    let slice = &s[0..2]; // starting at zero \__  same result
    let slice = &s[..2]; //  starting at zero /

    // likewise to get the end you would start a x index &s[5..];
    // or even take the whole string as &s[..]; or 0-length

    /*
    Note: String slice range indices must occur at valid UTF-8 character boundaries.
    If you attempt to create a string slice in the middle of a multibyte character,
    your program will exit with an error. For the purposes of introducing string
    slices, we are assuming ASCII only in this section; a more thorough discussion
    of UTF-8 handling is in the “Storing UTF-8 Encoded Text with Strings” section
    of Chapter 8.
    */

    // with this let's rewrite the first_word function
    // the logic works because we are tied to the underlying data
    let x = first_word_redo(&s);

    // now the values are maintained because we are ref the underlying value
    // x is a pointer to the slice
    // &s is a potiner to the String
    println!("{}, {:p}, {:p}", s.len(), x, &s);

    let s = "";

    println!("{}", s.len());

    // recalling immutable ref to something we cannot also take a mutable ref

    let mut s = String::from("hello world");
    let word = first_word_redo(&s);
    // s.clear(); // error!
    println!("the first word is: {}", word);

    //     error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //    --> src\main.rs:557:5
    //     |
    // 556 |     let word = first_word_redo(&s);
    //     |                                -- immutable borrow occurs here
    // 557 |     s.clear(); // error!
    //     |     ^^^^^^^^^ mutable borrow occurs here
    // 558 |     println!("the first word is: {}", word);
    //     |                                       ---- immutable borrow later used here

    // String literals are slices
    let s = "Hello, world!";

    // the type of s is &str and why string literals are immutable &str is an immutable reference

    // String slices as params

    // because of this we know can rewrite the first_word to handle both literals and strings
    // by changing first_word_redo(s: &str) -> &str
    // then you pass it the slice of the string or string literal (String - str)

    // Other slices
    let a: [i64; 5] = [3, 3, 2, 3, 4];

    let slice_a = &a[2..5];
}
