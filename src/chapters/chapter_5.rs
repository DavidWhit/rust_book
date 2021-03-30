pub fn defining_and_init_structs() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 5.1
    // will create a user with defaults and takes email and username
    fn build_default_user(email: String, username: String) -> User {
        User {
            // Note repeating the fields is tedious so you can do
            // because it makes sense to name the params the same a the field
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // CHAPTER 5 Defining and Instantiating Structs

    // note that everything has to be mutable Rust doesn't allow specific fields to be mutable.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusernamehere"),
        active: true,
        sign_in_count: 1,
    };

    // uses dot notation like everything else

    user1.email = String::from("anotheremail");

    // creating instances from others with struct update syntax

    let user2 = User {
        email: String::from("another@exam2ple.com"),
        username: String::from("anotherusername5647"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // we coul also create an instance and populate the remaining like so
    let user3 = User {
        email: String::from("another@exa3mple.com"),
        username: String::from("anotherusername5637"),
        ..user1
    };

    // Using tuple stucts without named Fields to create different types.
    // See struct section
    // although they have the same data types as inputs they are different my name
    // that is a Color is not a Point and vice versa
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like Structs are Structs without any fields
    // this is because thye behave similarly to () the unit type
    // Unit-like structs can be useful in situations in which you need to
    // implement a trait on some type but don't have any data that you want to store in the type
    // itself MORE IN CHAPTER 10 traits
    // Ownership of Struct Data
    //
    // in the examples we used the owned String type rather than the slice type &str
    // this was to make sure we own all of its data and for that data to be valid for as long
    // as the entire struct is valid.

    // However, it is possible for structs to store refs to data owned by something else but to do so
    // requires use of lifetime
}

pub fn using_structs_and_method_syntax() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    // FUNCTIONS ********************************
    fn area_struct(r: &Rectangle) -> u32 {
        r.height * r.width
    }

    fn area_tuple(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    // Incorp functions
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // refactored with tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // adding more meaning
    // added Rectangle Struct in struct section below

    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    // Adding useful functionality with derived traits

    // struct types dont implement std::fmt::Display
    // nor debug
    // Rust does include functionality to print out debugging info, but we have to explicitly opt in to
    // make that functionality avialable. To do that we add an annotation
    // #[derive(Debug)] on the line before the struct definition
    // no you should be able to access it with the debug trait
    println!("{:?}", rect1);
    // order add # for pretty print
    println!("{:#?}", rect1);

    // Implementing methods
    // methods are simply functions defined within the contect of a struct, enum, or trait object
    // and their first param is always self which is the instance of the struct in the method being called
    // see imp below
    // using the same Rectangle instance

    println!("{:#?}", rect1.area());

    // where is the -> operator
    // Here’s how it works: when you call a method with object.something(),
    // Rust automatically adds in &, &mut, or * so object matches the signature
    // of the method. In other words, the following are the same:

    // p1.distance(&p2);
    // (&p1).distance(&p2);

    // methods with more parameters

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated Functions (similiar to static functions)
    // often used as
    // these are not considered methods because they don't have an instance
    // like String::from is an associated method
    // add square function to Rectangle as an associated function
    let some_square = Rectangle::square(5);

    println!("Some square {:#?}", some_square);

    // you are also allowed to use multiple imp blocks though this will be covered more in depth in
    // chapter 10.
}
