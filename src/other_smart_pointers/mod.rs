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
