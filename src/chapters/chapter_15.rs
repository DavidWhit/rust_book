pub fn smart_pointers() {
   /*
   Recall from C++ days a pointer is just that it points to a location in memory
   via the address of which it belongs.

   In Rust it's indicated by the & symbol and borrow the value they point to.
   There is no overhead and the most used.

   Smart pointers are data structures that act as a pointer but include metadata.
   While the concept isn't new these pointers provided by std library provides the
   added functionality beyond that provided by references.

   One example is the reference counting smart pointer type.
   This pointer tracks multiple owners of data that when none exist cleaning up the data.

   In Rust the concept of ownership and borrowing and additional difference between
   references and smart pointers is that refs are pointers that only borrow data in contrast
   smart pointers own the data they point to.

   Some of those already encountered are:
   String
   Vec<T>
   That's because they own some memory and allow you to manipulate it.
   They also have metadata and extra capabilities ( such as with String ensuring its data will
   always be a valid UTF-8)

   Some smart pointers are implemented using structs
   The characteristic that distinguishes the two is that
   Smart Pointers implement the Deref and Drop traits.

   The Deref trait allows and instance of the smart pointer struct to behave like a reference
   so you can write code that works with either references or smart pointers.

   The Drop trait allows you to customize the code that is run when an instance of the smart pointer
   goes out of scope.

   Given that the smart pointer pattern is a general design patter used frequently in Rust
   the doc can't cover them all, but will cover the most common smart pointers in the std lib

   Box<T> for allocating values on the heap
   Rc<T> a reference counting type that enables multiple ownership
   Ref<T> and RefMut<T> accessed through RefCell<T> a type that enforces the borrowing
   rules at runtime instead of compile time.

   and in addition cover the "interior mutability" pattern where an immutable type exposes an API
   for mutating an interior value and discuss "reference cycles": how they can leak memory and how
   to prevent them.


   */

    // *** Using Box<T> to point to Data on the heap

    // Allows you to put data on the heap vs the stack and the stack has a pointer back to the
    // heap

    // Boxes don't have performance overhead other than storing their data on the heap
    // But they don;t have many extra capabilities either.

    /* Most often used in the following situations.
    cases:
    1.When you have a type whose size can't be known at compile time and you want to use a value
    of that type in a context that requires an exact size.

    2.When  you have a large amount of data and you want to transfer ownership but ensure the
    data won't be copied when you do so.

    3.When you want to own a value and you care only that it's a type that implements a particular
    trait rather than being of a specific type.

    FOR 1. Demonstrate the first example in "Enabling Recursive Types with Boxes

    FOR 2. trans ownership of a large amount of data can take a long time because
    the data is copied around the stack.If we offload that to the heap in a box then a small
    pointer can be copied around on the stack

    FOR 3. The 3rd case is known as a trait object that chp17 covers more in detail
    known as "Using trait objects that allow for values of different types"

    */

    // *** Using as Box<T> to store Data on the Heap

    pub fn pseudo_main() {
        // b exists on the stack points to a box that has 5 in it on the heap
        // just like any other owned value when the box goes out of scope so does
        // does the b smart pointer at the end of main

        // this is obviously a contrived example but makes the point what the box is doing.
       let b = Box::new(5);
        println!("b = {}", b);

        // looking at recursive types with boxes
        // we have a cons function short for construct function derived from cons list data
        // structure in Lisp

        // typically in FP "to cons x onto y" means to construct a new container instance by
        // putting element x at the start of this new container followed by the container y

        // Each item in a cons list contains two elements the current val and the next item.
        // The last item in the list contains a value called Nil

        // This is common base case for recursion
        // although fp langs use cons lists frequently the cons list isn't common in Rust.
        // most often Vec<T>



        enum List {
            Cons(i32, Box<List>), // add box here
            Nil
        }

        use List::*;

        // Now because box is in our cons list we need to prefix the second param with a new box
        let list =  Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(List::Nil) )))));

        // How Rust computes the size of a non-recursive type

        // Example

        enum Message {
            Quit,
            Move { x: i32, y: i32},
            Write(String),
            ChangeColor(i32, i32,i32)
        }

        // To determine the space rust goes through each of the variants to see which variant
        // needs the most space. Then uses the vairant with the most needed space


        // in contract with recursive type rust tries to determine how much space a recursive type
        // like the List enum compiler starts looking at the Cons variant which holds a value of
        // i32

        let x = Message::Write(String::from("sadfjklsjdklfj"));

        if let Message::Write(y) = x {
            println!(" message {}", y)
        };
    }

    pseudo_main();
}