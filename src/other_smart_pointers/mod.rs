use futures::StreamExt;

pub fn cow() {
    use std::borrow::Cow;

    fn abs_all(input: &mut Cow<[i32]>) {
        for i in 0..input.len() {
            let v = input[i];
            if v < 0 {
                input.to_mut()[i] = -v;
            }
        }
    }

    // No clone occurs because input doesn't need to be mutated
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

    // clone occurs because input needs to be mutated
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

    // no clone occurs because input is already owned
    // to be more clear see there is no reference it is directly owned
    let mut input = Cow::from(vec![-1, 0, 1]);
    abs_all(&mut input);

    // Keep a COW in struct

    struct Items<'a, X: 'a>
    where
        [X]: ToOwned<Owned = Vec<X>>,
    {
        values: Cow<'a, [X]>,
    }

    impl<'a, X: Clone + 'a> Items<'a, X>
    where
        [X]: ToOwned<Owned = Vec<X>>,
    {
        fn new(v: Cow<'a, [X]>) -> Self {
            Items { values: v }
        }
    }

    // creates a container from borrowed values of a slice
    let readonly = [1, 2];
    let borrowed = Items::new((&readonly[..]).into());
    match borrowed {
        Items {
            // checks matching struct and destructs the value out of it and used a COW
            values: Cow::Borrowed(bo),
        } => println!("borrowed {:?}", bo),
        _ => panic!("expect borrowed value"),
    }

    let mut clone_on_write = borrowed;

    // Mutates the data from slice into owned vec and pushes a new value on top
    clone_on_write.values.to_mut().push(3);
    println!("clone_on_write = {:?}", clone_on_write.values);

    // The data was mutated. Let check it out
    match clone_on_write {
        Items {
            values: Cow::Owned(_),
        } => println!("clone_on_write contains owned data"),
        _ => panic!("expect owned data"),
    }
}

pub fn trial_cow_obj_destruct_match() {
    use std::borrow::Cow;

    // working with Cow on my own
    let x = vec![3, 23, 43, 32];
    let mut y: Vec<i32> = Cow::from(&x[..2]).into();
    println!("{:?}", &y);
    y.push(10000);
    println!("x => {:?}  y = {:?}", x, y);


    // destruct value from match
    #[derive(Debug )]
    struct Thing {
        name: String,
    }

    impl Thing {
        fn new(name: String) -> Self {
            Thing { name }
        }
    }

    impl Iterator for Thing {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            Some((&self.name).parse().unwrap())
        }
    }

    let x = Thing::new(String::from("David"));

    match x {
        Thing { name: x } => println!("{}", x),
        _ => panic!("should have had value"),
    }


    // making a mutable refer and owned example
    // d is owned
    let mut d = vec![1,2,3,43,44];
    d.iter().for_each(|x| println!("{}", x + 1));

    // d is ref
    let mut d = &mut vec![1,2,3,43,44];
    d.iter().for_each(|x| println!("{}", x + 1));


    let x = vec![Thing {name: String::from("Dave")}
                                  , Thing {name: String::from("Dave")}];

    // for d in x {
    //     println!("{}", d.name)
    // }

    fn names(thing_name: Thing) -> String {
        thing_name.name
    }

    // let names: Vec<String> = x.into_iter().map(names).collect();
    // OR
    let names: Vec<String> = x.into_iter().map(|x| x.name).collect();

    println!("{:?}", names);


    let mut x = Box::new(6);

    // the value the box pointer on the stack pointing to value on the heap.
    *x= *x + 6;

    println!("{}", x);

}
