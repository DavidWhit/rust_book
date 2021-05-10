pub sync_and_send_traits() {
    /*
    Rust actually has very few concurrency features apart of it's std lib not the language. 
    Your options for handling concurrency are not limited to the language or the standard lib;
    You can write your own concurrency features or use those written by others.

    However, two concurrency concepts are embedded in the language: the std::marker traits sync and send;

    *** Allowing Transference of Ownership Between Threads with Send

        The send marker trait indicates ownership of values of the type implementing Send can be transferred between
        thread. Almost every Rust type is Send, but there are some exceptions including Rc<T> due to the fact if it was cloned
        to another thread both threads might update the ref count at the same time and why Rc<T> is for single thread

        All primitive types are Send, aside from raw pointers.

    
    *** Implementing Send and Sync Manually Is Unsafe
        Being types are made of Send and Sync traits are automatcially Send and Sync,
        we don't have to implement those traits manually. As marker traits they don't
        even have any methods to implement. They're just for enforcing invariants related
        to concurrency.

        Manually implementing these traits involves implementing unsafe Rust code. 
        We'll talk about using unsafe Rust code in Chp 19. The important part is that building
        new concurrent types not made up of Send and Sync parts requires careful thought to uphold the
        safety guarantees. https://doc.rust-lang.org/nomicon/index.html
        Has info about these guarantees and how to uphold them.

    *** Summary
        In chp20 will use more realistic situations than smaller examples discussed.

        Crates support most accelerated usage of concurrency IE Tokio

        The std lib does provide channels for message passing and smart pointer types
        such as Mutex<T> and Arc<T>
        
    */
    
}