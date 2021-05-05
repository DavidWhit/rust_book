use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};


pub fn reference_count_smart_pointer() {
    /*
     The majority of cases, ownership is clear: you know exactly which variable owns a given value.
     However, there are cases when a single value might have multiple owners. For example, in graph
     data structures, multiple edges might point to the same node, and that node is conceptually
     owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t
     have any edges pointing to it.

     To enable multiple ownership Rust has a type called Rc<T> which is an abbreviation for
     reference counting; it keeps track the number of references to a value which determines
     whether or not a value is still in use. If the count is zero it can be cleaned up
     without any references becoming invalid.

     Imagine it as a TV in a family room. Multiple people can watch it, but when the last person
     leaves they turn it off.

     We use the ref counter when we want to allocate some data on the heap for multiple parts of our
     program to read and we cant determine at compile time which part will finish using the data last.
     If we knew which one would we could make it that part the data's owner, and the normal
     ownership rules enforced at compile time would take effect.

     Keep in mind the ref type is only for use in single-threaded scenarios. When we discuss
     concurrency in 16 we can talk about how that works.
    */

    /* Using Rc<T> to Share Data
    b = 3  points to a
    a  = 5 points to 10 points to Nil
    c = 4 points to a

    b -> 3   \
       a -> 5 --> 10 -> Nil
    c -> 4   /
   */

    // recursive type


    // the one in the current scope otherwise we would have had to prefix every Cons with List::Cons
    // and Nil as List::Nil.
    use List::{Cons, Nil};

    // errors in value already moved.
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // let's use the ref counter so we can have multiple ownership

    let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // We could have called Rc::clone() rather than Rc::clone(&a) but Rust convention is to use
    // clone in this case. The implementation of Rc::clone doesn't make a deep copy of all the
    // data like most type implementations of clone do. Rc::clone only increments the ref count.
    // Where as deep copies can obviously take a while.

    // That is if looking for performance issues disregard calls to Rc::clone
    // Showing the ref counts after
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

pub fn interior_mutability_pattern() {
    // This is a design pattern in rust that allows you to mutate data even where there are
    // immutable references to that data; normally, this action is disallowed by the borrowing
    // rules. To mutate data the pattern uses unsafe code inside a data structure to bend
    // Rusts usual rules that govern mutation and borrowing.

    /*
    We can use types that use the interior mutability pattern when we can ensure that the borrowing
    rules will be followed at runtime, even though the compiler can’t guarantee that. The unsafe
    code involved is then wrapped in a safe API, and the outer type is still immutable.

    Using the Refcell<T>

    Unlike Rc<T> the RefCell<T> types represents single ownership over the data it holds.
    So, what makes RefCell<T> different from Box<T>?

    Remember borrowing rules:
    You have have either one mutable reference or any number of immutable refs but not both.
    Refs must always be valid

    With references and Box<T> the borrowing rules invariants are enforced at compile time.
    With RefCell<T> these invariants are enforced at runtime.
    Meaning you could have runtime errors if not careful.

    ***Advantages*** of checking the borrowing rules at runtime instead is that certain memory-safe
    scenarios are then allowed, whereas they are disallowed by the compile-time checks. Static
    analysis, like the Rust compiler, is inherently conservative. Some properties of code are
    impossible to detect by analyzing the code: the most famous example is the Halting Problem,
    which is beyond the scope of this book but is an interesting topic to research.

    The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the
    compiler is unable to understand and guarantee that.

    Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.

Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable
    borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.

RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T>
    even when the RefCell<T> is immutable. Mutating the value inside an immutable value is the interior
    mutability pattern. Let’s look at a situation in which interior mutability is useful and examine
    how it’s possible.


    However, there are situations in which it would be useful for a value to mutate itself in its
    methods but appear immutable to other code. Code outside the value’s methods would not be able
    to mutate the value. Using RefCell<T> is one way to get the ability to have interior mutability.
    But RefCell<T> doesn’t get around the borrowing rules completely: the borrow checker in the
    compiler allows this interior mutability, and the borrowing rules are checked at runtime
    instead. If you violate the rules, you’ll get a panic! instead of a compiler error.

    When creating immutable and mutable references, we use the & and &mut syntax, respectively.
    With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that
    belongs to RefCell<T>. The borrow method returns the smart pointer type Ref<T>, and borrow_mut
    returns the smart pointer type RefMut<T>. Both types implement Deref, so we can treat them like
    regular references.
    */

    // let x = 5;
    // causes cannot borrow as mutable
    //let y = &mut x;

    let mut x = 5;
    let y = RefCell::new(x);
    println!("x {} y {}", x, y.borrow());
    *y.borrow_mut() = 122;

    println!("x {} y {}", x, y.borrow());

    // Just jacking around with RefCell to alter an immutable to initialize another immutable
    let d = RefCell::new(String::from("kjfaksjdfkjasdfkj"));
    let e = format!("{} {}", &(d.borrow_mut())[..5], "55555");
    println!(" d => {} ||******|| e =>  {}", e, d.borrow());

    // A Use Case for Interior Mutability: Mock Objects
    // A test double is the general programming concept for a type used in place of another type
    // during testing. Mock objects are specific types of test doubles that record what happens
    // during a test so you can assert that the correct actions took place.


    // *************************************************************************
    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    // *************************************************************************
    // Rc<T> lets you have multiple owners of some data but it only gives immutable access
    // to that data.


    use List_Rc::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("a after = {:?}", b);
    println!("a after = {:?}", c);
}


// Import part is that Messenger trait has one method called send that takes
// an immutable reference to self and the text message
// This is the interface our mock object needs to have.
// The other important part is that we want to test the behavior of the set_value
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!")
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("You are within 10% left of your available quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        // changed this
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // updated empty vec![]
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    use std::cell::{RefCell, Ref};
    // lets use the smart pointer to fix the issue

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // *** Keeping Track of Borrows at Runtime with RefCell<T>
            // self.sent_messages.borrow_mut().push(String::from(message));

            // The RefCell<T> keeps up with how many ref<t> and refmut<t>
            // are currently active and when we borrow through the functionality of the
            self.sent_messages.borrow_mut().push(String::from(message));

            // now make it fail during runtime because the refcell was already borrowed

            // you can only have 1 borrow_mut() or several immutable borrow

            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum List_Rc {
    Cons(Rc<RefCell<i32>>, Rc<List_Rc>),
    Nil,
}

// Creating another list variation to
#[derive(Debug)]
enum List_Cycle {
    Cons(i32, RefCell<Rc<List_Cycle>>),
    Nil,
}

// ref cycle
impl List_Cycle {
    fn tail(&self) -> Option<&RefCell<Rc<List_Cycle>>> {
        match self {
            List_Cycle::Cons(_, item) => Some(item),
            List_Cycle::Nil => None
        }
    }
}


pub fn reference_cycles_can_leak() {
    /*
    Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create
    memory that is never cleaned up (known as a memory leak). Preventing memory leaks entirely is
    not one of Rust’s guarantees in the same way that disallowing data races at compile time is,
    meaning memory leaks are memory safe in Rust. We can see that Rust allows memory leaks by
    using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other
    in a cycle. This creates memory leaks because the reference count of each item in the cycle
    will never reach 0, and the values will never be dropped.
    */

    // How a ref cycle can happen and how to prevent it
    // Notice that the second element of List_Cycle is RefCell<Rc<List>>
    // Meaning that instead of having the ability to modify the to.
    // We're also adding a tail method to make it convenient for us to access the second item
    // if we have a cons variant
    /*
        let a = Rc::new(List_Cycle::Cons(5, RefCell::new(Rc::new(List_Cycle::Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?} ", a.tail());

        let b = Rc::new(List_Cycle::Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a initial count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // uncomment the next line to see that we have a cycle
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());

        // ****************************************************************
        // If you ever have RefCell<T> that contain Rc<T> values or similar nested combinations
        // of types with interior mutability and reference counting you must ensure that you don't
        // create cycles; you cannot rely on Rust to catch them
        // ****************************************************************

        /* Cycle created
        a -> 5 -> 10 points back to 5
             b -> 10 points back to 5
        */

    */

    /*

    Recap calling Rc::clone increases the strong_count of an Rc<T> instance
        and an Rc<T> and an Rc<T> instance is only cleaned up if its strong_count = 0

    You can create a weak reference to the value within an Rc<T> by calling Rc::downgrade
    and passing a reference to the Rc<T>. When this is done you get a smart pointer of type Weak<T>
    instead of increasing the strong_count you increase the weak_count both of which are managed
    through the Rc<T> instance. Weak counts dont need to be 0 for it to be cleaned up.

    Strong references are how you can share ownership of an Rc<T> instance. Weak references don't
    express ownership relationship. They won't cause a ref cycle because any cycle involving weak
    refs will be broken once the strong ref count is 0

    Because of this fact any Weak<T> ref might have been dropped to do anything with the value
    that a Weak<T> is pointing to you must make sure the value still exists.

    Do this by calling the upgrade method on a Weak<T> to get an Option<Rc<T>>

    To Demonstrate rather than using a list whose items know only about the next we'll create a
    tree whose items know about their children items and their parent items.

    */

    #[derive(Debug)]
    struct Node {
        value: i32,
        // add parent
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    // we want a Node to own its own children and share that ownership with variables so we access
    // each Node directly. So we define a Vec<T> where T is Rc<Node>. We also want to modify which
    // nodes are children of another node so we have RefCell<T> in children around Vec<Rc<Node>>
    // Next use the struct to create a node named leaf with 3 and no children
    // Then another branch

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]) });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // we clone the Rc<Node> in leaf and stored that in the branch meaning the Node in leaf
    // has two owners: the leaf and branch.
    // So at the moment branch can get to the leaf but the leaf can't get to the branch
    // because it doesn't know how to get there.

    // In order to do this we need to modify the node to add the parent
    // The problem is deciding what type is the parent. We know it can't contain a Rc<T>
    // because that create a cycle where strong counts values are never 0
    // because leaf.parent points to branch
    //     and branch.children points to leaf

    // We could think about that relationship another way, a parent node should own its children.
    // If a parent node dropped so does the children
    // Child node shouldn't own its parent nor drop them.
}