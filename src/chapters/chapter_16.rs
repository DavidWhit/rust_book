
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;

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

    The example will create a program that has one thread to generate values and send them 
    down a channel and thread that will receive the values and print them out. 

    We'll be sending simple values between threads using a channels 
    */

    // mpsc == multi producer single consumer
    // returns a tuple transmitter, receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // closure has ownership of tx via move and then send takes ownership
        // println!("Got: {}", val);  val is no longer available
    });

    let received = rx.recv().unwrap(); // rx.recv blocks the main thread and returns Result<T,E>
    // try_recv is useful if the thread has other work to do while waiting for messages
    println!("Got: {}", received);

    //***  Channels and Ownership Transference
    // make two producers
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you")
        ];

        for val in vals {
            tx_clone.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });


    // The sends from above are going to make 
    // again the main thread is blocked until all messages are received
    for received in rx {
        println!("Got: {}", received);
    }


    /* *** Shared-State Concurrency
    msg passing is a fine way handling concurrency but its not the only one

    what would communicating by sharing memory look like?

    channels in any programming language are similar to single ownership, because once
    you transfer a value down a channel you should no longer use that value.
    shared memory is having multiple ownership where mult threads can access the same 
    memory location this is more difficult because multiple owners need managing
    Rusts ownership and rules greatly assit in getting this management correct.


    *** Using Mutexes to Allow Access to Data from One Thread at a Time

    Mutex is an abbreviation for mutual exclusion it only allows one thread to access
    some data at any given time. To access teh data in a mutex a thread must first signal
    that it wants access by asking to aquire the mutex lock. The lock is a data structure
    that is part of the mutex that keeps track of who currently has exclusive access to the
    data. Therefore, the mutex is described as guarding the data it holds via the locking
    system.

    Mutexes have a reputation for being difficult to use because you have to remember two rules:
    1. You have to attempt to acquire the lock before using the data
    2. When you're done with the data the mutex guards you must unlock the data so other
        threads can acquire it.

    Management of mutexes can be incredibly tricky to get right which is why so many people
    are proponents of channels. However, thanks to Rust you can't get locking and unlocking
    wrong.


    *** The API of Mutex<T>

    */

    // First lets use a mutex in a single thread context
    // Mut<T> is a smart pointer
    let m = Mutex::new(5);

    {
        // lock will block the current thread (this case the main thread)
        // the call to lock would fail if another thread holding the lock panicked
        // the call to lock returns a type MutexGuard
        // it implements Deref, Drop to point to the inner data 
        let mut num = m.lock().unwrap();
        *num = 6;
    }// lock is dropped after scope

    println!("m = {:?}", m);

}