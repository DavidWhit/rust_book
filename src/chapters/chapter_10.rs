use std::fmt::Display;

pub fn using_generic_data_types() {
    /*
    Similar to the way a function takes parameters with unknown values to run the same
    code on multiple concrete values, functions can take parameters of some generic type
    instead of a concrete type, like i32 or String. In fact, we’ve already used generics in:
        Chapter 6 with Option<T>
        Chapter 8 with Vec<T> and HashMap<K, V>
        Chapter 9 with Result<T, E>.
    In this chapter, you’ll explore how to define your own types,
    functions, and methods with generics!
    */

    // Removing Duplication by Extracting a Function
    // skipped most this is basic ABC stuff identify reusable code and wrap it in a func

    // let number_list = vec![34,50,25,100,65];
    //
    // let mut largest = number_list[0];
    //
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    //to this
    // fn largest(list: &[i32]) {
    //     let mut big = &list[0];
    //     for num in list {
    //         if num > big {
    //             big = num;
    //         }
    //     }
    //     println!("The largest is {:?}", big)
    // }

    // Generics in Function definitions

    //Now with generic T changing to return
    // Currently fails with std::comp::PartialOrder
    // This trait would need to be implemented on type T
    // fn largest<T>(list: &[T]) -> &T {
    //     let mut largest = &list[0];
    //     for x in list {
    //         if x > largest {
    //             largest = x;
    //         }
    //     }
    //     largest
    // }
    let char_list = vec!['d', 'a', 'e', 'f', 'c'];
    // let result = largest(&number_list);
    // let result2 = largest(&char_list);

    // println!("The first largest => {} and second call largest => {}", result, result2);

    // Generics In Struct Definitions
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // Notice the syntax for both functions and struct is similar  in the <T>

    // fields x and y are both of the same type thus this would fail
    // let wont_work = Point {x: 1, y: 4.0};

    // In this case we could define another that takes multiple
    // It's a good sign to restructure if you are using too many
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 }; // must change

    // Generics in Enum Definitions

    enum Option<T> {
        Some(T),
        None,
    }
    // this should make more sense now with std Result
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // Again the rule of thumb is if the same code is duplicated consider generics.

    // In Method Definitions

    struct Point3<T> {
        x: T,
        y: T,
    }

    impl<T> Point3<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point3 { x: 5, y: 10 };

    println!("{}", p.x());

    /*
    We could, for example,
    implement methods only on Point<f32>
    instances rather than on Point<T> instances with any generic type. In
     Listing 10-10 we use the concrete type f32, meaning we don’t declare any types after impl.
     */
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    /*
    Generic type parameters in a struct definition aren’t always the same as those you use in
    that struct’s method signatures. For example, Listing 10-11 defines the method mixup on the Point<T, U>
     */

    struct Point4<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point4<T, U> {
        fn mixup<V, W>(self, other: Point4<V, W>) -> Point4<T, W> {
            Point4 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let mut p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // recall just calling self in the mixup took ownership
    // so this would error
    // println!("p3.x = {}, p3.y = {}", p1.x, p1.y);

    // ********* RUST ZERO COST ON GENERICS
    // able to do this via monomorphization of the code
    // which is the red power ranger... It's the process of turning generic code into specific code
    // by filling in the concrete types that are used when compiled

    // The compiler does the opposite of the steps we used to create the generic function
    // it looks for all the generic code calls and code generates for it the concrete types the
    // generic code is called with

    // IE looking at the Option<T> enum

    let integer = Some(5);
    let float = Some(5.0);

    // when the compiler runs it performs monomorphization during that process the compiler reads
    // the values that have been used in Option<T> then identifies two kinds of Option<T>
    // one of i32 and f64 replacing the generic ones with specific ones producing the following
    // code gen

    /*
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }
     */
}

pub fn traits_defining_shared_behavior() {
    // traits tell the Rust compiler about functionality a particular type has and can share with
    // other types. They can be used to define shared behavior in an abstract way.
    // Further we can use trait bounds to specify that a generic can be any type that has certain
    // behavior

    /* EXAMPLE USE CASE

    A type’s behavior consists of the methods we can call on that type. Different types share the
    same behavior if we can call the same methods on all of those types. Trait definitions are a way
    to group method signatures together to define a set of behaviors necessary to accomplish some
    purpose.

    For example, let’s say we have multiple structs that hold various kinds and amounts of text: a
    NewsArticle struct that holds a news story filed in a particular location and a Tweet that can
    have at most 280 characters along with metadata that indicates whether it was a new tweet, a
    retweet, or a reply to another tweet.

    We want to make a media aggregator library that can display summaries of data that might be
    stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type, and
    we need to request that summary by calling a summarize method on an instance. Listing 10-12
    shows the definition of a Summary trait that expresses this behavior.


    */
    // you see pub because I'm copying the examples and exist in lib.rs
    pub trait Summary {
        fn summarize(&self) -> String;

        fn summarize_author(&self) -> String;

        // this creates a default which can call others within the trait
        fn summarize_default(&self) -> String {
            format!("(Read more {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{} {} {}", self.headline, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{} {}", self.username, self.content)
        }

        fn summarize_author(&self) -> String {
            format!("{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // One restriction to note with trait implementations is that we can implement a trait on a
    // type only if either the trait or the type is local to our crate.

    // see additional https://doc.rust-lang.org/book/ch10-02-traits.html

    // default implementations

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize_default());

    // If we had default functions in Summary trait we could assign them by implementing an
    // empty block for that type. Provided it wasn't mixed with others that require implementation

    // struct Whosawhatsit {
    //     thing: String
    // }
    //
    // impl Summary for Whosawhatsit {}

    //**** TRAITS AS PARAMETERS
    // passing any type that implements Summary trait
    pub fn notify_org(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    //*** TRAIT BOUND SYNTAX same as above
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    //The impl Trait syntax is convenient and makes for more concise code in simple cases

    //The trait bound syntax can express more complexity in other cases. For example, we can
    //have two parameters that implement Summary. Using the impl Trait syntax looks like this:

    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
    //If we wanted this function to allow item1 and item2 to have different types, using impl
    // Trait would be appropriate (as long as both types implement Summary). If we wanted to
    // force both parameters to have the same type, that’s only possible to express using a
    // trait bound, like this:

    // force same type on both
    // pub fn notify<T: Summary>(item1: &T, item2: &T) {}

    //*** SPECIFYING MULTIPLE TRAIT BOUNDS WITH THE + SYNTAX

    /* We can also specify more than one trait bound. Say we wanted notify to use display formatting
    on item as well as the summarize method: we specify in the notify definition that item must
    implement both Display and Summary. We can do so using the + syntax:
    */

    // with sugar
    pub fn notify_mult_trait(item: &(impl Summary + Display)) {}

    // traitbound
    pub fn notify_mult_trait_bound_syntax<T: Summary + Display>(item: &T) {}

    //*** Clearer Trait Bound with where Clauses

    /*
    Using too many trait bounds has its downsides. Each generic has its own trait bounds,
    so functions with multiple generic type parameters can contain lots of trait bound information
    between the function’s name and its parameter list, making the function signature hard to read.
    For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause
    after the function signature. So instead of writing this:

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        we can use a where clause, like this:


        fn some_function<T, U>(t: &T, u: &U) -> i32
            where T: Display + Clone,
                  U: Clone + Debug
        {}

    This function’s signature is less cluttered: the function name, parameter list, and return type
    are close together, similar to a function without lots of trait bounds.

    */

    // *** Returning Types that Implement Traits

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    /*

    The ability to return a type that is only specified by the trait it implements is especially
    useful in the context of closures and iterators, which we cover in Chapter 13. Closures and
    iterators create types that only the compiler knows or types that are very long to specify.
    The impl Trait syntax lets you concisely specify that a function returns some type that
    implements the Iterator trait without needing to write out a very long type.

    However, you can only use impl Trait if you’re returning a single type. For example, this code
    that returns either a NewsArticle or a Tweet with the return type specified as impl Summary
    wouldn’t work:

    Doesn't work because how impl Trait sytax is implemented in the compiler
    This is later discussed in chapter 17 "using trait objects that allows values of different
    types

    fn returns_summarizable2(switch: bool) -> Box<dyn Summary> {
        let x = if switch {
            NewsArticle {
                headline: String::from(
                    "Penguins win the Stanley Cup Championship!",
                ),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
                ),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            }
        }
    }

    Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl
    Trait syntax is implemented in the compiler. We’ll cover how to write a function with this
    behavior in the “Using Trait Objects That Allow for Values of Different Types” section of
    Chapter 17.
    */
    // *** Fixing the largest Function with Trait Bounds

    
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![2, 32, 43, 11, 23, 2];
    let char_list = vec!['a', 'z', 'x', 'A', 'd'];

    let number_result = largest(&number_list);
    let char_result = largest(&char_list);

    println!(
        "The largest in number_list ===> {} and the largest in char is ===> {}",
        number_result, char_result
    );

    struct Pair<T> {
        x: T,
        y: T,
    }

    // *** Using Trait Bound to Conditionally Implement Methods

    /* By using a trait bound with an impl block that uses generic type parameters, we can
      implement methods conditionally for types that implement the specified traits. For example,
      the type Pair<T> in Listing 10-16 always implements the new function. But Pair<T> only
      implements the cmp_display method if its inner type T implements the PartialOrd trait that
      enables comparison and the Display trait that enables printing.
    */
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // only if the type T implements display and PartialOrd otherwise the
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmd_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let x = Pair::new(3, 40);
    let y = Pair::new('s', 'a');
    x.cmd_display();
    y.cmd_display();

    /*
    Traits and trait bounds let us write code that uses generic type parameters to reduce
    duplication but also specify to the compiler that we want the generic type to have particular
    behavior. The compiler can then use the trait bound information to check that all the concrete
    types used with our code provide the correct behavior. In dynamically typed languages, we would
    get an error at runtime if we called a method on a type which didn’t define the method.
    But Rust moves these errors to compile time so we’re forced to fix the problems before our
    code is even able to run. Additionally, we don’t have to write code that checks for behavior
    at runtime because we’ve already checked at compile time. Doing so improves performance without
    having to give up the flexibility of generics.

    Another kind of generic that we’ve already been using is called lifetimes. Rather than ensuring
    that a type has the behavior we want, lifetimes ensure that references are valid as long as we
    need them to be. Let’s look at how lifetimes do that.
    */
}

pub fn validating_references_with_lifetimes() {
    /*
    One detail we didn’t discuss in the “References and Borrowing” section in Chapter 4 is that
    every reference in Rust has a lifetime, which is the scope for which that reference is valid.
    Most of the time, lifetimes are implicit and inferred, just like most of the time, types are
    inferred. We must annotate types when multiple types are possible. In a similar way, we must
    annotate lifetimes when the lifetimes of references could be related in a few different ways.
    Rust requires us to annotate the relationships using generic lifetime parameters to ensure the
    actual references used at runtime will definitely be valid.

    The concept of lifetimes is somewhat different from tools in other programming languages,
    arguably making lifetimes Rust’s most distinctive feature. Although we won’t cover lifetimes in
    their entirety in this chapter, we’ll discuss common ways you might encounter lifetime syntax
    so you can become familiar with the concepts.
    */

    // *** Preventing Dangling References with Lifetimes

    // fn main() {
    //     {
    //         let r;                // ---------+-- 'a
    //                                           |
    //         {                     //          |
    //             let x = 5;        // -+-- 'b  |
    //             r = &x;           //  |       |
    //         }                     // -+       |
    //                                           |
    //         println!("r: {}", r); //          |
    //     }                         // ---------+
    // }

    // From the above we can see a lives longer than b so we should get a compiler warning

    // re writing as follows
    {
        let x = 5; // ----------+-- 'b
                   //           |
        let r = &x; // --+-- 'a  |
                    //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    } // ----------+

    // Not sure on how the scopes are defined 'a or 'b in regards to scope...
    // The book doesn't make clear the outter scope is xzy and inner scope is zyx

    // *** Generic lifetimes in Functions

    // Currently results in an error due to:
    // Rust can’t tell whether the reference being returned refers to x or y. Actually, we don’t
    // know either, because the if block in the body of this function returns a reference to x and
    // the else block returns a reference to y!
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // THE FIX!!!
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // *** Lifetime Annotation Syntax
    /*
    Lifetime annotations don’t change how long any of the references live. Just as functions can
    accept any type when the signature specifies a generic type parameter, functions can accept
    references with any lifetime by specifying a generic lifetime parameter. Lifetime annotations
    describe the relationships of the lifetimes of multiple references to each other without
    affecting the lifetimes.

    Code style:
    must start with an apostrophe (') and are usually all lowercase and very short, like generic
    types. Most people use the name 'a. We place lifetime parameter annotations after the & of a
    reference, using a space to separate the annotation from the reference’s type.

    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime

    One lifetime annotation by itself doesn’t have much meaning, because the annotations are
    meant to tell Rust how generic lifetime parameters of multiple references relate to each other.
    For example, let’s say we have a function with the parameter first that is a reference to an
    i32 with lifetime 'a. The function also has another parameter named second that is another
    reference to an i32 that also has the lifetime 'a. The lifetime annotations indicate that
    the references first and second must both live as long as that generic lifetime.
    */

    // Another example
    let string3 = String::from("long string is long");
    let result;

    // inner block scope
    {
        let string4 = String::from("xyz");
        result = longest(string3.as_str(), string4.as_str());
    }
    // outer scope
    // this would fail because the values exist at the same level of scope of the func call
    // and not beyond it.
    // println!("The longest string is {}", result);

    // *** Thinking in terms of lifetimes.
    /*
    The way in which you need to specify lifetime parameters depends on what your function is doing.
    For example, if we changed the implementation of the longest function to always return the first
    parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y
    parameter. The following code will compile:
    */
    fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // however this would fail will fail to compile because the return value lifetime is not
    // related to the lifetime of the parameters; the x param
    // fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    //     let result = String::from("something string");
    //     result.as_str()
    // }

    // In this case, the best fix would be to return an owned data type rather than a reference
    // so the calling function is then responsible for cleaning up the value.

    // *** Lifetime Annotations in Struct Definitions

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a ','");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("The part {}", i.part);

    // This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds
    // in its part field.

    // *** Lifetime Ellision
    /*
    You’ve learned that every reference has a lifetime and that you need to specify lifetime
    parameters for functions or structs that use references. However, in Chapter 4 we had a
    function in Listing 4-9, which is shown again in Listing 10-26, that compiled without lifetime
    annotations.


    How did this then compile?
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    Instead of having a signature of:
    fn first_word<'a>(s: &'a str) -> &'a str {}

    The Rust team found there were a lot of writing the same annotations in deterministic ways
    that could be elided

    Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return
     values are called output lifetimes.

    There are a couple of rules when no lifetimes are defined.
    1. Each parameter that is a reference gets its own lifetime
    2. If there is only 1 input lifetime all params get the same output lifetime
    3. If there are multiple input params but one of them is &self or &mut self
    the lifetime of self is assigned to all output lifetime parameters.
    */

    // ***  Lifetime Annotations in Method definitions
    /*
    Where we declare and use the lifetime parameters depends on whether they’re related to the
    struct fields or the method parameters and return values. Lifetime names for struct fields
    always need to be declared after the impl keyword and then used after the struct’s name,
    because those lifetimes are part of the struct’s type. In method signatures inside the impl
    block, references might be tied to the lifetime of references in the struct’s fields, or they
    might be independent. In addition, the lifetime elision rules often make it so that lifetime
    annotations aren’t necessary in method signatures. Let’s look at some examples using the struct
    named ImportantExcerpt that we defined in Listing 10-25.
     */

    // let's update the original above renamed to keep the other in place

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        // we’re not required to annotate the lifetime of the reference to self because of the first
        // elision rule.

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
        // There are two input lifetimes, so Rust applies the first lifetime elision rule and gives
        // both &self and announcement their own lifetimes. Then, because one of the parameters is
        // &self, the return type gets the lifetime of &self, and all lifetimes have been accounted
        // for
    }

    // *** The Static Lifetime

    let s: &'static str = "I have a static lifetime.";

    /*
    You might see suggestions to use the 'static lifetime in error messages. But before specifying
    'static as the lifetime for a reference, think about whether the reference you have actually
    lives the entire lifetime of your program or not. You might consider whether you want it to
    live that long, even if it could. Most of the time, the problem results from attempting to
    create a dangling reference or a mismatch of the available lifetimes. In such cases, the
    solution is fixing those problems, not specifying the 'static lifetime.
    */

    // *** Generic Type Parameters, Trait Bound, and Lifetimes Together

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let x = longest_with_an_announcement("kasjdfkasjd", "Asdflkjasdlkfj",
                                         String::from("SHIIIZ"));

    println!("longest {}", x);
}
