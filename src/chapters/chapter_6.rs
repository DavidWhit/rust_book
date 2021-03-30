pub fn enums() {
    // STRUCTS **************************************************
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    /*************** ENUMS ****************/
    enum IpAddrKind {
        V4,
        V6,
    }

    enum IpAddrEnum {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {}
    }
    // Recap: enumerations all you to define a type by enumerating its possible values
    // Rust also has the Option enum like scala
    // then combining enums with match

    // Also remember that enums are great and often better than structs when the data
    // can be enumerated; such as, IP address versions. Where you're certain the outcome is of
    // of enumerated field
    // enum IpAddrKind added in the enum section at the bottom
    // they can be used now as a type
    // like fn route(ip_kind: IpAddrKind) {}
    // then callable with either variant
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // Though we don't have the address leaning on structs to contain our new enum type.
    // see struct IpAddr

    let home = IpAddr {
        address: String::from("127.0.0.1"),
        kind: IpAddrKind::V4,
    };

    // * HOWEVER WE CAN GET MORE MEANING OUT OF THE ENUM *
    // to be more concise we can add the value of the address to the enum
    // rather than an enum in a struct
    // ** see enum IpAddr
    // where as if we left the struct in place we are stuck with String as a type for both
    // though with enum we can specify different concrete types that
    // IE now can define them differently

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));

    // this is common enough that the std lib defines them as such
    // struct Ipv4Addr {
    //     // --snip--
    // }

    // struct Ipv6Addr {
    //     // --snip--
    // }

    // like we just did it but enums a struct vs a struct with an enum when we first started.
    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),

    // while here if we had the IpAddr we wouldnt conflict becuase we havent imported anything

    // Alright we created a message Enum type that matches the book
    // with 4 variants

    // we could have defined 4 structs but if we used different structs

    // struct QuitMessage; // unit struct
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // tuple struct
    // struct ChangeColorMessage(i32, i32, i32); // tuple struct
    // each have there own type we couldnt easily define a function to take
    // all as we could with Message
    // Also we have a similiarity in that we can implement functions on enums
    // see below

    let m = Message::Write(String::from("hello"));
    m.call(); // does nothing as nothing was actually implemented

    // The Option enum as define by the std lib
    // enum Option<T> {
    //   Some(T),
    //   None,
    // }

    // where <T> is the generic type or some variant of the Option
    // getting an idea how to use Option (no need to import as it is included)

    let some_number = Some(5);
    let some_string = Some("a string");

    // if we use None rather than some we need to tell Rust what it is
    // as the compiler can't infer the type.
    let absent_number: Option<i32> = None;

    // none also doesnt have a display; makes sense.
    // println!("{} {} {}", some_number, some_string, absent_number);

    // how do you get the T value out of Some variant when you have a value of type Option<T>
    // so you can use that value? I'm thinking unboxing... but they haven't talked about that
    // IMPORTANT: Become fmailiar with the methods on Option<T> will be extremely useful
}

pub fn enum_and_flow_control() {
    // Match Control Flow Operator
    // Rust provides control flow operator called match to compare patterns of literal values,
    // variable names, wildcards, and many other things dicussed in chp 18

    // lets to an inner scoped enum example as the book
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    // unlike an if that requires a bool we can take any type
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            // or add a block example
            // Coin::Dime => 10,
            Coin::Dime => {
                println!("You got a dime");
                10
            }
            Coin::Quarter => 25,
        }
    }

    let penny = value_in_cents(Coin::Penny);
    let nickel = value_in_cents(Coin::Nickel);
    let dime = value_in_cents(Coin::Dime);
    let quarter = value_in_cents(Coin::Quarter);

    println!("Penny {}", penny);
    println!("Nickel {}", nickel);
    println!("Dime {}", dime);
    println!("Quarter {}", quarter);

    // This is slick, something I haven't at least seen in other langs

    #[derive(Debug)] // to view the state
    enum UsState {
        Alabama,
        Alaska, // etc
    }

    enum CoinBindValExample {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents_match_bind(coin: CoinBindValExample) -> u8 {
        match coin {
            CoinBindValExample::Penny => 1,
            CoinBindValExample::Nickel => 5,
            // or add a block example
            // Coin::Dime => 10,
            CoinBindValExample::Dime => {
                println!("You got a dime");
                10
            }
            CoinBindValExample::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let coin = value_in_cents_match_bind(CoinBindValExample::Quarter(UsState::Alabama));

    println!(
        "The quarter value is {} and show state in function print",
        coin
    );

    // Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Matching everything else use the _ placeholder
    // the () is unit value
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // nothing happens
    }

    // when you just care about one value use if let

    // consider this
    let a_value = Some(5);

    match a_value {
        Some(5) => println!("five"),
        _ => (),
    }

    // just do this

    if let Some(v) = Some(5) {
        println!("Yeah we got the value: {}", v);
    }

    let mut count = 0;

    let coin = CoinBindValExample::Quarter(UsState::Alaska);

    match coin {
        CoinBindValExample::Quarter(state) => println!("State quarter from {:#?}", state),
        _ => count += 1,
    }

    // or use if let else
    let mut count = 0;

    let coin = CoinBindValExample::Quarter(UsState::Alaska);

    if let CoinBindValExample::Quarter(state) = coin {
        println!("State quarter from {:#?}", state);
    } else {
        count += 1;
    }
}
