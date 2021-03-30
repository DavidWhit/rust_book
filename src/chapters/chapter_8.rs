use std::collections::HashMap;

pub fn working_with_vectors() {
    // Vectors, string, hash map
    // others mentioned https://doc.rust-lang.org/std/collections/index.html

    // declare a vec that holds type <T>
    let mut v: Vec<i32> = Vec::new();
    v.push(30);

    // declare a vec that holds the next type used
    let mut h = Vec::new();
    h.push("343");

    // or use the macro with some values
    // and make this one immutable
    let d = vec![1, 3, 2, 34, 3, 23];

    // again recall block scopes
    {
        let mut d = vec![1, 23, 23, 2, 2, 3, 2];
        for x in &mut d {
            *x += 50
        }
        for x in d {
            println!("d inner block -> {}", x);
        }
    } // shadowed d is freed

    // if you just use d it will be moved
    for x in &d {
        println!("d out of block -> {}", x);
    }

    // reading single values
    let third = &d[2];

    match d.get(2) {
        Some(third) => println!("we got something {}", third),
        None => println!("Nothing found"),
    }

    let first = &v[0];
    // first is ok to use until we change the underlying vector
    println!("first val = {}", first);
    v.push(40);
    //    println!("first val = {}", first); -- would fail here

    // Think about the above we said v was mutable but first was not
    // we could use the first var only before changes to v
    // this would break the contract of saying first was immutable.

    // Recall borrowing rules https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html

    // 1. one or more references to a resource
    // 2. exactly 1 mutable reference
}

pub fn working_with_hashmap() {
    // hashMaps
    //    use std::collections::HashMap;
    //
    //    let mut scores = HashMap::new();
    //
    //    scores.insert(String::from("Blue"), 10);
    //    scores.insert(String::from("Yellow"), 50); // Final Answer  why Rust doesnt allow indexing

    // or

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // if you plan on using scores after you need ref
    for (x, y) in &scores {
        println!("key {}, value {}", x, y)
    }

    // Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Green");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // fields are moved here

    // Accessing values in a hash Map
    if let Some(item) = map.get("Favorite color") {
        println!("{}", item);
    }

    // Update by overwriting
    // just insert with the same key

    // only insert a value if a key has no value

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    let d = scores.entry(String::from("Blue")).or_insert(50);
    *d = 60;

    println!("{:?}", scores);
}
