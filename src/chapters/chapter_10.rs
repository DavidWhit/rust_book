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

    let number_list = vec![34,50,25,100,65];
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
    // let char_list = vec!['d','a','e','f','c'];
    // let result = largest(&number_list);
    // let result2 = largest(&char_list);

    // println!("The first largest => {} and second call largest => {}", result, result2);


    // Generics In Struct Definitions
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    // Notice the syntax for both functions and struct is similar  in the <T>

    // fields x and y are both of the same type thus this would fail
    // let wont_work = Point {x: 1, y: 4.0};


    // In this case we could define another that takes multiple
    // It's a good sign to restructure if you are using too many
    struct Point2<T,U> {
        x: T,
        y: U
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
    enum Result<T,E> {
        Ok(T),
        Err(E)
    }

    // Again the rule of thumb is if the same code is duplicated consider generics.

    // In Method Definitions

    struct Point3<T> {
        x: T,
        y: T
    }

    impl<T> Point3<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point3 { x: 5, y: 10};

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
        y: U
    }

    impl <T, U>  Point4<T,U> {
       fn mixup<V,W>(self, other: Point4<V,W>) -> Point4<T,W> {
            Point4 {
                x: self.x,
                y: other.y
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
        pub content: String
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{} {} {}", self.headline, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            format!("{}",self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{} {}", self.username, self.content)
        }

        fn summarize_author(&self) -> String {
            format!("{}",self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
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
    pub fn notify_mult_trait(item: &(impl Summary + Display)) {};

    // traitbound
    pub fn notify_mult_trait_bound_syntax<T:Summary + Display>(item: &T) {};
}