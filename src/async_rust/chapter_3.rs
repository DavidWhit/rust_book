use futures::future::Future;

// use futures::future::Future;
// https://book.async.rs/overview/std-and-library-futures.html
pub fn async_await() {
    // *********** async/.await
    // async/await are special pieces of Rust syntax that make it possible to yield control of the
    // current thread rather than blocking, allowing other code to make progress while waiting

    // Two main ways to use them
    // async fn
    // async {} (blocks)
    // each return value implements Future trait

    async fn foo() -> u8 {5}

    fn bar() -> impl Future<Output = u8> {
        async {
            let x : u8 = foo().await;
            println!("{}", x + 5);
            x + 5
        } // statisfies a returned future where output is u8
    }



    // *********** Async Lifetimes
    // unlike traditional async fn's which take references or other non 'static arguments
    // return a Future which is bounded by the lifetime of the arguments.

    // This function
    async fn foo2(x: &u8) -> u8 { *x }
   // is same as
    fn foo2_expand<'a>(x:&'a u8) -> impl Future<Output = u8> + 'a {async  move {*x}}

    /* this means that the future returned from an async fn must be .await'ed while its non 'static
       arguments are still valid. In the common case of .await ing the future immediately after
       calling the function (as in foo(&x)) this is not an issue. However, if storing the future
       or sending it over to another task or thread, this may be an issue.

       One common work around for turning an async fn with refs as arguments into a 'static
       future is to bundle the arguments with the call to the async fn inside an async block:
     */


    // // BAD
    // fn bad() -> impl Future<Output = u8> {
    //     let x = 5;
    //     borrow_x(&x) // Error: x does not live long enough
    // }

    // fn good() -> impl Future<Output = u8> {
    //     async {
    //         let x = 5;
    //         borrow_x(&x).await
    //     }
    // }

    // *********** async move
    /*
    async blocks and closures allow the move keyword, much like normal closures. An async move block
    will take ownership of the variables it references, allowing it to outlive the current scope, but
    giving up the ability to share those variables with other code.
    */

    async fn blocks() {
        let my_string = "foo".to_string();

        let future_one = async {
           println!("{}", my_string);
        };

        let future_two = async {
            println!("{}", my_string);
        };

        // run both to completion
        let ((), ()) = futures::join!(future_one, future_two);
    }

    fn move_block() -> impl Future<Output = ()> {
        let my_string = "foo".to_string();
        async move {
            println!("{}", my_string);
        }
    }

    //


    #[derive(Debug)]
    enum Sex {
        Male,
        Female
    }

    #[derive(Debug)]
    struct Dave {
        built: u8,
        sex: Sex,
        age: Option<u8>,
        height: Option<u8>,
    }

    impl Dave {
        fn new(built: u8, sex: Sex ) -> Dave {
            Dave {
                built,sex, age: None, height: None
            }
        }

        fn set_height(mut self, height: u8) -> Self{
            self.height  = Some(height);
            self
        }
        fn set_age(&mut self, age: u8) -> &mut Self  {
            self.age  = Some(age);
            self
        }


    }

    // you can do this because it is not yet assigned to x or pre-consumption of ownership
    let x = Dave::new(8, Sex::Male).set_height(6);

    println!("{:?}", x);

}