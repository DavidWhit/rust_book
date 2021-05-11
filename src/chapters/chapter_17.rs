pub fn characteristics_of_oop() {
    // How OOP relates to Rust 
    // Gang of four authors OOP desing patterns

    // an object packages both data and the procedure that operate on that data. 
    // The procedures are typically methods or operations.

    /*
    By the above definition Rust is object oriented: struct and enums have data and impl
    blocks provide methods on those. Even though structs and enums with methods aren't called
    objects they provide the same functionality.
    */


    /*
        *** Encapsulation that hides impl Details
            In short from way back the use of the object is through public methods
            provided by the API external code can't reach into the object and change

        Completely REVIEW but for the sake of going through the whole book.

    */

    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64
    }

    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None  => None
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }

    /* 

    Easy to see we have external methods to add and remove the last one 
    and it internally tracks average

    */ 

    let mut collection = AveragedCollection {list:  Vec::<i32>::new(), average: 0.0  };

    collection.add(32);
    collection.add(2);
    collection.add(12);
    collection.add(-2);
    collection.remove();

    println!("result {}", collection.average());


    /*
        *** Inheritance as a Type System and as Code Sharing
            A mechanism whereby an object can inherit from another objects def
            thus gaining the part objects data and behvaior without defining them again

            Gorrilla bananna problem

            If a lang must have inheritance to be an oop than rust is not one.

            Rust does similiar through traits as I already know from previous chps
            TRAITS ARE BETTER THAN INHERITANCE Defining shared behaviors

        *** Polymorphism
            To many people, polymorphism is synonymous with inheritance. But it’s actually a more general
            concept that refers to code that can work with data of multiple types. For inheritance, those 
            types are generally subclasses.

            Agreed basically
            add(f32, f32)
            add(ojbFoo, ojbBar)

            Rust instead uses generics to abstract over different possible types and trait bounds to impose
            constraints on what those types must provide. This is sometimes called bounded parametric 
            polymorphism.
            GENERICS ARE BETTER THAN POLY

            ****************************
            Inheritance has recently fallen out of favor as a programming design solution in many 
            programming languages because it’s often at risk of sharing more code than necessary. 
            Subclasses shouldn’t always share all characteristics of their parent class but will 
            do so with inheritance. This can make a program’s design less flexible. It also introduces 
            the possibility of calling methods on subclasses that don’t make sense or that cause 
            errors because the methods don’t apply to the subclass. In addition, some languages will 
            only allow a subclass to inherit from one class, further restricting the flexibility of a
            program’s design.
            ****************************


            Creating shared behavior
    */

    pub trait Draw {
        fn draw(&self);
    }

    // we dont know what draw might be other than it implements draw
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // *************** IMPORTANT DIFFERENCE
    // USING GENERICS
    // THIS would make all Screen items generic but enforce the same type 
    // on Vec<T> homogenous and if this is the only case then you should use generics
    // otherwise use the trait objects like above
    // pub struct Screen<T: Draw> {
    //     pub components: Vec<T>,
    // }

    // impl<T> Screen<T> 
    // where 
    //     T: Draw,
    // {
    //     pub fn run(&self) {
    //         for component in self.components.iter() {
    //             component.draw();
    //         }
    //     }
    // }

    // Implementing the Trait

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String, 
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>
    }

    impl Draw for Button {
        fn draw(&self) {
            // code to draw type Button
        }
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // code to draw type Button
        }
    }

    // What it looks like when we build it


    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ]
            }),
        ],
    };

    screen.run();

    /*
    Trait objects perform dynamic dispatch 

    Recall in the performance of code using generics chp 10 the disucssion on
    monomorphization process performed by th ecompiler when we use trait bounds
    on generics. 
    The compiler creates all the implementations at compile time vs dynamic dispatch

    Rust uses the pointers inside the trait object to know which method to call. This
    look up is at cost vs static dispatch and why if homogenous types are only used.
    Like above you would just then use trait objects.

    *** Object Safety Is Required for Trait Objects

    A trait is object safe if all the methods defined in the trait have the following properties

        The return type isn't Self
        There are no generic type parameters

    The Self keyword is an alias for the type we're implementing the traits or methods on.
    Trait objects must be safe because once you have used a trait object, Rust no longer knows
    the concrete type that's implementing that trait. If a trait method returns the concrete Self type,
    but a trait object forgets the same type Self is there is no way the method can be used
    for the original concrete type

    An example of a trait whose methods are not object safe is the standard lib Clone trait
    The signature for the clone method in the Clone trait looks like this:

    pub trait Clone {
        fn clone(&self) -> Self; // this breaks the first rule
    }

    // True even when we try to say it's dynamic dispatch with
    pub struct Screen {
        pub components: Vec<Box<dyn Clone>>,
    }

    */

}


// // 17.3 OOL STATE PATTERN


// // State Trait
// trait State {
//     fn request_review(self: Box<Self>)-> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     // default implementation
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }


// // DRAFT 
// struct Draft {}

// impl State for Draft{
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }


// // PENDING REVIEW 
// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>)-> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
// }

// // PUBLISHED 
// struct Published {}
// impl State for Published {
//     fn request_review(self: Box<Self>)-> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }


// //  POST 
// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }

//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }

//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//         // as_ref on the option because we want a ref to the value vs ownership
//         // also why we need a lifetime on content because we are just using a ref
//     }

//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }
// }


// the benefit is the logic related to the rules lives in the state objects rather than being
// scattered throughout Post

pub fn implementing_an_oo_design_pattern() {
    // let mut post = Post::new();

    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("",post.content());

    // post.approve();
    // assert_eq!("I ate salad for lunch today", post.content());
    

    /*
        *** Trade-offs of the State Pattern
            we've shown that Rust is capable of implementing the obj oriented state pattern to 
            encapsulate the different kinds of behavior a post should have in each state. The methods
            on post know nothing about the various behaviors. The way it's organized the code we have to 
            look in only ONE PLACE to know the different ways a published post can behave: The implementation
            of the state trait on the Published struct.

            If we were to create an alternative to the state pattern we might instead use match expressions 
            in the methods on post or even in the main code that checks the state of the post and changes
            behavior. That would mean we would have to look in several places to understand the implications
            of a post being in the published state. This would further increase the more states we added
            each match would need another arm. 

            currently the post methods and the places we use post dont need match expressions and to add a new
            state we would only need to add a new struct and implement the trait on that struct

            It's easy to extend to add more functionality

            The downside of the state pattern is that because the states implement the transisitions 
            between states of the these states are coupled to each other. 
            That is if we add another state between PendingReview and Published such as Scheduled we would
            have to change several objects.

            Another downside is that we've duplicated some logic.  
            You might think to create extra defaults that return self but this will violate our dyn State
            as a trait Object. Other duplication includes similiar implementations of the methods on Post.
            If we had a lot of methods on Post that followed this pattern we might consider defining a macro to 
            elminiate the repetition

            Furthermore by implementing the state pattern exactly as it's defined for OOL were not taking 
            full advantage of Rust's strenghts as we could. Let's Look at some changes we can make to the blog
            crate that can make invalid states and transitions into compile time errors



    */
}
// Post 
pub struct Post {
    content: String
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
// Draft 
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}
// PendingReview
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}

// Two major benefits:
// compile time checking
// no longer encapsulated on state transforms in the post implementation; although, we gain 
// that invalid states are now impossible because of the type system 
pub fn encoding_states_and_behavior_as_types() {

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}