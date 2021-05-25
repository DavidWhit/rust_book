use futures::executor::block_on;


pub fn Chp_1_1_Why_Async() {

    /*
    *** Async vs Other Concurrency models:
        1. OS threads don't require any changes to the programming model, which makes it very easy to express
        concurrency. However, synchronizing between threads can be difficult, and the performance
        overhead is large. Thread pools can mitigate some of these costs, but not enough to support
        massive IO-bound workloads.
        2. Event-driven programming, in conjunction with callbacks, can be very performant, but tends to
        result in a verbose, "non-linear" control flow. Data flow and error propagation is often
        hard to follow.
        3. Coroutines, like threads, don't require changes to the programming model, which makes them easy
        to use. Like async, they can also support a large number of tasks. However, they abstract away
        low-level details that are important for systems programming and custom runtime implementors.
        4. The actor model divides all concurrent computation into units called actors, which communicate
        through fallible message passing, much like in distributed systems. The actor model can
        be efficiently implemented, but it leaves many practical issues unanswered, such as
        flow control and retry logic.

        Remember by default Rust is a 1:1 with OS threads

    *** Async in Rust vs other langs
        While async is supported in many languages, some details vary across implementations.
        Rust implementation of async differs from most languages in a few ways:
            1.Futures are inert they make progress only when polled (lazy). Dropping a future stops progress
            2.Async is zero-cost without heap allocations (think Box<>) and dynamic dispatch (think <Box dyn>)
                Great for performance and can be used in embedded environments.
            3.No built-in runtime provided by Rust. Provided by the community think (tokio most popular)
                maintained crates.
            4.Both single-and multithreaded runtimes are available in Rust each with strengths and weaknesses.

    *** Async vs threads in Rust
        OS Threads are suitable for a small number of tasks and since threads come with CPU and Memory
        overhead. Spawning and switching between threads is quite expensive as even idle threads
        consume system resources. (context switching)

        Thread pooling can help mitigate some of these costs but not all.
        However threads let you reuse existing synchronous code with significant code change.
        In some OS you can even change the priority of a thread which is useful for drivers and
        and other latency sensitive applications.


        Async provides significantly reduced CPU and memory overhead, especially for workloads with a
        large amount of IO-bound tasks, such as servers and databases. All else equal, you can have
        orders of magnitude more tasks than OS thread, because an async runtime uses a small amount
        of (expensive) threads to handle a large amount of (cheap) tasks. One draw back is async Rust
        results in larger binary blobs due to the state machine generated from async functions and since
        each executable bundles an async runtime.

        One thing to note, async programming is not "better" than threads, just different. If you
        don't need async for performance reasons, threads can often be the simpler alternative.

    */

    // *** Example Concurrent downloading

    // fn get_two_sites() {
    //     let thread_one = thread::spawn(|| download("https://www.foo.com"));
    //     let thread_two = thread::spawn(|| download("https://www.bar.com"));
    // }
    //
    // // Wait for both threads to complete
    // thread_one.join().expect("one panicked");
    // thread_two.join().expect("two panicked");

    // *** downloading a web page is a small task; creating a thread for such a small amount of work
    // is quite wasteful. For a larger application, it can easily become a bottleneck. In async Rust
    // we can run these tasks concurrently without extra threads.

    // async fn get_two_sites_async() {
    //     // create two different futures
    //     let future_one = download_async("https://www.foo.com");
    //     let future_two = download_async("https://www.bar.com");
    //
    //     // run both at the same time
    //     join!(future_one, future_two);
    // }

    // above no extra threads are created. All function calls are statically dispatched
    // and there are no heap allocations. However, we need to write the code to be async
    // in the first place which this will help

    // Lastly Rust doesnt force you to choose between threads and async. You can use both models
    // within the same application, which can be useful when you have mixed threaded and async
    // dependencies. In fact you can even use a different concurrency model altogether, such
    // as event driven programming as long as you find a library that implements it.
}

pub fn Chp_1_2_State_of_Async_Rust(){
    /*

    *** The State of Async Rust

        Parts of async rust are supported with the same stability guarantees as aysnc Rust.
        Other parts are still maturing AND WILL CHANGE OVER TIME. You can expect async Rust:
        1.Outstanding runtime performance for typical concurrent workloads.
        2.More frequent interaction with advanced language features such as lifetimes and pinning.
        3.Some compatibility constraints, both between sync and async, and between different async
            runtimes.
        4.Higher maintenance burden, due to the ongoing evolution of async runtimes and language
            support.

    *** Language and library support
        While asynchronous programming is supported by Rust itself, most async applications depend
        on functionality provided by community crates (again tokio is the goto atm) As such, you need
        to rely on a mixture of lang and library support.

        1. The most fundamental traits, types and functions, such as the Future trait are provided
           by the standard library.
        2. The async/await syntax is supported directly by the Rust compiler
        3. Many utility types, macros and functions are provided by the futures crate. They can be
           in any async Rust application.
        4. Execution of async code IO and task spawning ar provided by "async runtimes"
        (Tokio and async-std). Most async apps and some aysnc crates depend on a specific runtime
        see the ecosystem here https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html
        which is chapter 8

    ***  Compiling and debugging
        For the most part, compiler and runtime errors in async Rust work the same was as they have
        always. There are a few notable differences.
            compilation errors:
                same high standards as synchronous Rust, but since async Rust often depends on more
                complex lang features again (lifetimes and pinning) you may encounter these types
                of errors more frequently.
            Runtime errors:
                Whenever the compiler encounters an async function, it generates a state machine
                under the hood. Stack traces in async Rust typically contain details from these
                state machine along with calls from the runtime. As such,
            New Failure:
                A few novel failures modes are possible with async Rust for instance if you call
                a blocking function from an async context or if you implement the Future trait
                incorrectly. Such errors CAN siletly pass both the compiler and sometimes even
                uint tests. Having firm understanding of the underlying concepts which this books
                aims to give.
    *** Compatibility considerations
        Asynchronous and synchronous code cannot always be combined freely. For instance you
        cant directly call an async function from a sync function. Sync and async code also tend
        to promote different design patterns, which can make it difficult to compose code
        intended for the different environments.

        even async code cannot always be combined freely some crates depend on specific async
        runtimes and what crates you may need early. Once you have settled you wont have to worry
        much about compatibility.

    *** Performance characteristics
        The async performance depends on the implementation of the async runtime you're using
        Even though runtimes that power async. Rust apps are relatively new they perform
        exceptionally well for most practical workloads.

        That said most async ecosystem assumes a multi threaded runtime. This makes it difficult to
        enjoy the theoretical performance benefits of single-threaded async applications namely
        cheaper synchronization.
        Another over looked use-case is latency sensitive tasks which are important for drivers
        GUI applications and so on. Such task depend on runtime and/or OS support in order to be
        scheduled appropriately.
    */
}

pub fn chp_1_3_async_await_primer(){
   /*
   async/.await is rusts built in tool for writing asynchronous functions that look like
   synchronous code.
   prefixing async to the function transforms the block of code into a state machine that implements
   a trait called Future.

   ***  Where as calling a blocking function in a synchronous method would block the whole thread, blocked
   Future/s will yield control of the thread allowing other Future/s to run.
   ***

   adding futures dependency cargo.toml
   [dependencies]
   futures="0.3"
   */

    async fn hello_future() {
        println!("hello, future!");
    }

    // the value returned by an async fn is a Future that in turn needs to run on an executor
    // because they are inherently lazy


    // block_on blocks the current thread until the provided future has run to completion.
    // Other executors provide more complex behavior, like scheduling multiples futures onto
    // the same thread.
    let future = hello_future();
    block_on(future); // future is ran now because it was polled as they are inherit lazy

    /*
    Inside an async fn you can use .await (like js) to wait for the completion of another type
    that implements Future trait such as the output of async fn. Unlike block_on await
    doesn't block the current thread but instead async waits for the future to complete
    allowing other tasks to run if the future is currently unable to make progress.

    For example imagine that we have three async fn:
    IE:
    async fn lear_song
    async fn sing_song
    async fn dance
    */
    // one way to do learn, sing and dance would be to block on each of these individually

    /*
    fn main() {
        let song = block_on(learn_song());
        block_on(sing_song(song));
        block_on(dance());
    }

    obviously blocking on each isn't the best performance as things are happening in sync order
    We have to learn the song before we can sing it but it's possible to dance at the same time
    as learning and signing the song. To do this we can create two separate async fn which can be
    run concurrently

    async fn learn_and_sing() {
        // wait till the song is learned before singing it
        let song = learn_song().await;
        sing_song(song).await;
    }

   async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1,f2);

     // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.

   }

   fn main() {
    block_on(async_main());

    // if we blocked on learn_song in learn_and_sing the thread wouldn't be able to do anything
    else while learn_song was running; making it impossible to dance at the same time.
   }
   */

}