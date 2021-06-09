#[derive(Debug)]
struct Name<'a> {
    owner: &'a str,
}

// struct NameCollection {
//     names: Vec<Name>
// }

// impl FromIterator<Name> for NameCollection {
//     fn from_iter<I: IntoIterator<Item=Name>>(iter: I) -> Self {
//         let mut c: Vec<Name> = vec![];
//         for i in iter {
//             c.push(Name { owner: i.owner});
//         } 

//         NameCollection { names: c}
//     }
// }

fn update_vector<'a>(value: &'a str, name_vec: Vec<Name<'a>>) -> Vec<Name<'a>> {
    let mut x = vec![];
    for mut vals in name_vec {
        if vals.owner == "Case" {
            vals.owner = value;
        }
        x.push(vals)
    }

    x
}

pub fn trial_iterators() {
    // annotate x with Vec<Name> or use collect 'turbofish' collect::<Vec<name>>()
    let mut x: Vec<Name> = vec![ Name { owner: "Case"}
    , Name { owner: "E"}
    , Name { owner: "Dave"}];

    // one way to change values of the struct
    for d in x.iter_mut() {
        if d.owner == "Case" {
            d.owner = "Replaced";
        }
    }

    println!("{:?}", x);

    // another way taking ownership calling fn and returning another
    // we don't use duplicate memory because Vec implement the drop trait 
    // Added in lifetimes for complexity 
    // and we consumed it in the function by taking ownership
    // also why y is immutable  
    let y: Vec<Name> = vec![ Name { owner: "Case"}
    , Name { owner: "E"}
    , Name { owner: "Dave"}];

    let a = update_vector("Lifetime fn mod Vec<T>", y);
    // y is inaccessible from fn taking ownership
    println!("{:?}", a);
    println!("{:?}", x);

    // *** A third way 
    let x: Vec<Name> = vec![ Name { owner: "Case"}
    , Name { owner: "E"}
    , Name { owner: "Dave"}];

    // spent so long on this trying to implement FromIter when the type return was just wrong
    // in the map make sure the type is the same and it will work.
    let h: Vec<Name> = x.into_iter().map( |mut v| { 
        if v.owner == "Case" {
            v.owner = "REPLACED"
        }
        v
    }).collect();
    // x is consumed // moved

    println!("{:?}", h);

}
