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

    /*
}