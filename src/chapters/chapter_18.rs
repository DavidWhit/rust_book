pub fn patterns_and_matching() {
    /* 

    match arms:

    Formally match expressions are defined as keyword match 
    and one or more match arms that consist of a pattern and an 
    expression.

    One requirement for match is they need to be exhaustive in that all
    possibilites for teh value in the match are accounted for.

    Though a particular pattern is to match anything else as a deafult by
    using _. It's often used in the last match arm. 

    IE
    */

    enum KindOfPeople {
        Good,
        Evil
    }

    let evil = KindOfPeople::Evil;

    // the _ makes it match for everything else but be careful
    // not to miss the Good match otherwise if you didnt use _ 
    // the compiler would force you match both
    match evil {
       KindOfPeople::Evil => { println!("evil kind of people")} 
       _ => { println!("everyone else")} 
    }


    // Conditional if let expressions 
    /*
    if let was a shorter way to write teh equivalent of a match that only 
    matches one case. Optionally if let can have a corresponding else 
    containing code to run if the pattern in the if let doesn't match.

    you can mix and match if let, else if and else if let.
    */

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your fav color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("purple");
        } else {
            println!("orange");
        }
    } else {
        println!("blue");
    }

    // the downside of if lets is they don't check exhaustiveness

    // ***  while let conditional loops

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
   
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }


    // *** for loops

    let v = vec!['a','b','c'];

    // we can use pattern after for to destruct values out of enumerate()
    for ( index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


    // *** let statements are even patterns
    let x = 5;
    // let pattern = expression;

    let (x,y,z) = (1,2,3);

    // this would error
    // let (x,y) = (1,2,3);

    // both of these work I prefer _ syntax unless number range
    let (x,y, _) = (1,2,3);
    let (x,y, ..) = (1,2,3);

    // function parameters can also be patterns
    // fn foo(x: i32) {
    // }

    fn print_coordinates(&(x,y): &(i32,i32)) {
        println!("Current location: ({}, {})", x,y );
    }

    let point = (3,5);
    print_coordinates(&point);

    /*
     Patterns come in two forms:
     refutable
     and irrefutable

     let x = 5  because x matches anything therefore cannot fail to match
     Patterns that can fail to match for some possible value are refutable. 
     An example would be Some(x) if None it will fail

     you could do if let x = 5 but the compiler will tell you that's dumb

    */

    // matching named variables

    let x = Some(5); 
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), 
        // we created and used y in scope (shadowing the outer scope) so in this case y is 5
        _ => println!("Default case, x = {:?}", x),
    }

    // multiple patterns

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything else") 
    }

    // matching ranges

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("a through j"),
         _ => println!("something else")
    }


    // Destructuring Structs 

    struct Point {
        x: i32, 
        y: i32, 
    }

    let p = Point {x: 0, y: 7};
    let Point { x: _, y: b} = p;
    // assert_eq!(0,a); 
    assert_eq!(7,b); 

    // Destructuring Enums

    enum Message {
        Quit, 
        Move { x:i32, y:i32},
        Write(String),
        ChangeColor(Color)
    }

    let msg = Message::ChangeColor(Color::Rgb(0,160,255));

    let move_message = Message::Move { x: 4 , y: 32};

    // You can't do this
    // let requires a irrefutable pattern
    // let Message::Move { x: a, y: b} = move_message;


    match msg {
        Message::ChangeColor(Color::Rgb(r,g,b)) => println!("values {} {} {}", r,g,b), 
        // do similiar things for the others 
        // Message::Move { x, y }  => 
        // Message::Quit => 
        // Message::Write (text) =>
        _ => println!("something else")
    }

    // Destructing nested Enums

    enum Color {
        Rgb( i32, i32, i32),
        Hsv( i32,i32,i32)
    }
    // with above message changed Color to new enum above
    // check the new match


    // Destructing Structs and Tuples
    let ((feet,inches), Point {x, y}) = ((3,10), Point { x:3, y:-10});

    println!("{} {} {} {}", feet, inches, x, y);

    // Ignore pattern with _ 
    // or unused with _x    *** THIS STILL BINDS *** where _ doesn't

    let _x = 6; // binds

    let s = Some(String::from("tst"));
    if let Some(_s) = s {
        println!("found string");
    }
      
    // This would error because it's moved by the bind  Some(_s) = s
    // println!("{:?}", s);

    let numbers = (2,4,8,16,32);

    match numbers {
        (first, _ , third,_, fifth) => println!("Some numbers: {} {} {} ", first, third, fifth)
    }


    // Ignoring multiple with ..

    struct Point2 {
        x: i32,
        y: i32,
        z: i32
    }

    let origin = Point2 {x: 0, y: 0, z:0};

    match origin {
        Point2 { x, ..} => println!("x is {}", x)
    }

    let numbers = (2, 4, 8, 16, 32);

    // Can only be used once in a pattern
    match numbers {
        (first, .., last) => println!("Some numbers: {}, {}", first, last)
        // Can only be used once in a pattern this would fail
        // 
        // (.., second, ..) => println!("Some numbers: {}", second)
    }

    // Extra Conditionals with Match Guards

    let num = Some(4);
    let y = 20;

    match num {
        // you can check against vals created out of the match scope
        // we just can't use  
        Some(x) if x < y => println!("less than {}: {}",y, x),
        Some(x) => println!("{}", x), 
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        // precedence (4|5|6) first then if 
        4 | 5| 6 if y => println!("yes"),
        _ => println!("no")
    }


    // @ bindings

    /*
        The @ operator lets us create a variable that holds a value at the same 
        time we're testing that value to see if it matches a pattern.

        IE we want to test that a Message:Hello id field is with the range 3..=7
        but we also want to bind the value to the variable id_variable so we can use it in 
        code associated to the arm. we could name this variable id, the same as the field, but for this exmaple
        we'll use a different name.

    */

    enum Message2 {
        Hello {id: i32}
    }

    let msg = Message2::Hello {id: 5};


    // basically you can bind the variable that's within the range 
    // without knowing what it was other than it matched and we get the value
    // that was in range back
    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7, 
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello {id: 10..=12} => println!("Found in another range"),
        Message2::Hello {id } => println!("Found some other id: {}", id),
    }


}