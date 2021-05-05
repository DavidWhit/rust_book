pub fn fearless_concurrency() {
    /*
    I already know this but recapping:
    Concurrent programming = where different parts of a program execute independantly
    Parallel programming =  where different parts of a program execute at the same time

    Ever more important because cpus dont get faster just more cores

    Rust devs found that ownership and type systems are powerful set of tools to help manage memory
    saftey and concurrency problems!
    
    This allows catching a lot of bugs before being release to production by getting catch by the 
    compiler; thus, dubbed fearless

    For the sake of simplicity the rest of the chapter refs problems as concurrent rather than being more
    percise saying concurrent and/or parallel.
    So mentally substitute concurrent for both cases.

    Discusses:
    How to create threads to run multiple pieces of code at the same time
    Message-passing concurrency, where channels send messages between threads
    Shared-state concurrency, where multiple threads have access to some piece of data.
    The Sync and Send traits, which extend Rust's concurrency guarantees to user-defined types as
        well as types provided by the standard library.
    */



    /*
    * In most programs executed code runs in a process and the OS manages those at once.
    * Within your program you can also have independent parts that run simultaneously.
    * This ability is imparted by threads

      remember in school days discussions of hardware threads and software threads.

      splitting the computation in your program into multiple threads can improve performance because
      the program does multiple tasks at the same time, but it also adds complexity. Threads can run
      simultaneously, there's no inherent guarantee about the order in which parts of your code run on
      different threads. This causes problems like:
        Race conditions, 
        Dead locks
        Bugs -- Something that is hard to replicate in a threaded environment

      Rust trys to mitigate the negative use of threads but programming a multi threaded context still takes
      careful thought and requries a code structure that is different from that in programs running  a single thread.

      Rust implements threads in a few different ways. 
      Many OS's provide API for creating new threads often called 1:1 meaning one OS thread per language thread

      Then programming languages provide their own special implementation of threads. 
      Programming language provided threads are known as green threads and run in the context of a different number of 
      operating system threds. For this reason green-threaded model is called the M:N model there are (M) green threads
      per (N) operating system threads where M and N are not necessarily the same number 

      Each model has it's own advantages and disadvantages and the disadvantage most important to Rust is 
      runtime support. Runtime is confusing as it can have different meanings in different contexts.

      In this context by runtime we mean code that is included by the language in every binary; larger or small. 
      Every non-assembly language will have some amoutn of runtime code. Colloquially when people say a language has 
      "no runtime" they often mean "samm runtime" Smaller runtimes = fewer features but result in smaller binaries
      which can combine with other languages in more contexts. Rust neds to have nearly no runtime and cannot compromise on
      being able to call into C to maintain performance.

      The green-threaded M:N model requires larger language run times to manage threads as such Rust std::lib onlyr provides
      implementation of 1:1 trheading. Though there exists crates that implement M:N threading for the cost of overhead. 
      For aspects such as more control over which threads run when and lower costs of context switching,
    */

    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if here the thread would complete first before moving on.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // notice from the above that the main thread can close before the spawned thread completes
    // if the handle code is removed.
    // to resolve this by saving the handle added above and joining it back to the main thread.
    // thread::spawn return type is a JoinHandle which is an owned value 
    // HOWEVER this causes the code to wait for the spawned thread
    // IF positioned after the spawned thread which doesn't make a lot of sense unless you 
    //      to wait for all threads to finish independantly join at the end
    //      then the time is the slowest thread

    // *** using move closures with threads
    // the move closure is often used alongside thread::spawn because it allows you to use data 
    // from another thread.

    // in chapter 13 we mentioned we can use the move before the param list of a closure to
    // force the closure to take ownership of the values it uses in the environment. 
    // especially used 

    let v = vec![1,2,3];

    let handle = thread::spawn( move || {
        // grants us certainity that it cant be used again outside the spawned thread
        // check drop(v)
        println!("here's a vector: {:?}", v);
    });

    // drop(v);

    // Rust infers how to capture v, and because println! only needs a reference to v, the closure 
    // tries to borrow v. However, there’s a problem: Rust can’t tell how long the spawned thread 
    // will run, so it doesn’t know if the reference to v will always be valid.
    // *** add move to the closure to force it take ownership of v

    handle.join().unwrap();

}

pub fn using_msg_passing_to_trns_data_btwn_threads() {
    /*
    An increasingly popular approach to ensure safe concurrency is message passing
    where threads or actors communicate by sending each other messages containing data. 
    The idea slogan from "Go lang"  "Do not communicate by sharing memory; instead
    share memory by commmunicating."

    The major tool Rust has for accomplishing message-sending concurrency is the channel
    a concept that the std lib provides. 

    Think of it as a channel of water such as a stream or a river. If you put something
    like a rubber duck or boat into a stream, it will travel downstream to the end of the
    waterway.

    A channel has two halves a transmitter and a receiver
    The transmitter half is upstream sending them 
    The reciever half is downstream  receiving 
    The channel is closed if either the transmitter or receiver half is dropped.

    The example will create a 
    */
}