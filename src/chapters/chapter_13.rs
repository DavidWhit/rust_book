// like from js closures or anonymous functions
// that capture their environment
use std::io::Chain;
use std::thread;
use std::time::Duration;

// The current cacher has two problems
// 1 The Cacher instance assumes it will always get the same value for the Parameter
// 2 The Second is that it only takes a type of u32

// Then this test fails because we store the first value and never change it.
// #[test]
// fn call_with_diffferent_values() {
//     let mut c = Cacher::new(|a| a);
//     let v1  = c.value(1);
//     let v2  = c.value(2);
//
//     assert_eq!(v2, 2);
// }

struct Cacher<T>
where
    T: Fn(u32) -> u32, // we know this is a closure because of the Fn trait impl here
{
    calculation: T, // T is the closure with signature defined by trait impl
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn closures() {
    // going to create a hypothetical long (ie 3 seconds or so compute time)
    // we dont want to constantly run because we don't want the use waiting more than they should.
    fn simulated_expensive_calculation(intentsity: u32) -> u32 {
        println!("calculating slowly...");
        // commented out for future runs of chapters
        // thread::sleep(Duration::from_secs(2));
        intentsity
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        // now we call this once not twice in the first if but we call it
        // always regardless in the case of a rnd 3
        // but we now call this everytime we call the function
        // let expensive_result = simulated_expensive_calculation(intensity);

        // now instead of calling the function before the if logic below we could define
        // a function to store the closure in a variable rather than storing the result

        // if we had more than one param you separate them out like
        // |num, num1, numN|
        // we could change the calls back but we would introduce the same problem
        // as before.
        // let expensive_closure = |num| {
        //     println!("calculating slowly...");
        //     thread::sleep(Duration::from_secs(2));
        //     num
        // };

        // Common syntax begining with function
        // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
        // let add_one_v2 = |x: u32| -> u32 { x + 1 };
        // let add_one_v3 = |x|             { x + 1 };
        // let add_one_v4 = |x|               x + 1  ;

        //*** Storing closures Using Generic Parameters and the Fn Traits

        // Fortunately, another solution is available to us. We can create a struct that will hold
        // the closure and the resulting value of calling the closure. The struct will execute the
        // closure only if we need the resulting value, and it will cache the resulting value so
        // the rest of our code doesn’t have to be responsible for saving and reusing the result.
        // You may know this pattern as memoization or lazy evaluation.

        // The Fn traits are provided by teh std library. All closures implement at least one of
        // the traits: Fn, FnMut, FnOnce discussed further in "Capturing the Environment with
        // closures"

        let mut expensive_result = Cacher::new(|num| {
            println!("Calculating slowly...");
            // commented out for future runs of chapters
            // thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            // refactor call once
            // println!("Today , do {} pushups!", simulated_expensive_calculation(intensity))
            println!("Today , do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            match random_number {
                3 => {
                    println!("Take a break today! Remember to stay hydrated");
                }
                _ => {
                    println!(
                        "Today, run for {} minutes!",
                        expensive_result.value(intensity)
                    );
                }
            }
        }
    }

    fn simulated_main() {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    // notice that with the closure we only have executed it once
    // by seeing the calculating print slowly
    simulated_main();

    // *** Capturing the environment with Closures
    // Closures have an additional capability that functions don't have:
    // They can capture their environment and access variables from the scope they are defined in

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;

    assert!(equal_to_x(y));

    // if the above was a function that wouldn't work because the scope is within the func
    // However there is a cost associated with it
    // When a closure captures a value from its environment it uses memory to store the values
    // for use in the closure body because normal functions don't allow this it will never incur
    // this same overhead.

    // closures capture values from their environments in 3 ways that map the the Fn Traits
    // 1. Taking ownership     FnOnce can't take ownership of the same variable more than once
    //                         so it can only be called once.
    // 2. Borrowing mutably    FnMut
    // 3. borrowing immutably  Fn

    // Rust infers based on how the closure uses the values from the environment.
    // All closures
    // implement FnOnce because they can all be called at least once
    // Closures that dont move captured vars also implement FnMut
    // Closures that dont need mutable access to vars also implement Fn

    // IE the equal_to_x closure only needs to read the value of x so only needs the Fn Trait
    // you can however force a closure to take ownership of the values it's using by the
    // move keyword before param list
    // This technique is mostly useful when passing a closure to a new thread so it's owned
    // by the new thread. More to come in chapter 16

    // Be aware move closures may still implement fn or fnMut even though they capture variables
    // by move. This is because closures are determined by what they do with the values.
    // not how it captures them. Move only specifies the latter.

    /* Most of the time when specifying one of the Fn trait bounds, you can start with Fn and the
    compiler will tell you if you need FnMut or FnOnce based on what happens in the closure body.
    */
}

pub fn iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // iterators implement a trait called Iterator that is defined by the std library as follows
    // pub trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    //
    //     // methods with default implementations elided
    // }
    // ** new things is type Item  Self::Item
    // which are defining an associated type with this trait

    // It also requries you define an Item type and this Item type is used in the return type of
    // the next method. This item type will be the type returned from the iterator

    // *** Methods that produce other iterators

    let v1 = vec![1, 2, 3];

    // this would cause an error because iterators are lazy and do nothing unless consumed
    // v1.iter().map(|x| x + 1);

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    // *** Comparing Performance: Loops vs Iterators

    // let buffer: &mut [i32];
    // let coefficients: [i64; 12];
    // let qlp_shift: i16;

    // The compiler knows by the definition size of 12
    // it "unrolls" the loop generating repetitive code for optimization
    // it generates
    // for i in 12..buffer.len() {
    //     let prediction = coefficients.iter()
    //         .zip(&buffer[i-12..i])
    //         .map(|(&c, &s)| c * s as i64)
    //         .sum::<i64>() >> qlp_shift;
    //
    //     let delta = buffer[i];
    //     buffer[i] = prediction as i32 + delta;
    // }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// *** Using Closures that Capture their Environment
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

mod test {

    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    // *** using other iterator trait methods
    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        //               0 1 2 3 4 5
        // zip skip 1    1 2 3 4 5 None (zip is like stacking blocks in a row like a zipper)
        // map across    0 2 6 12 20  (5 and None doesnt produce)
        // filter            6 12
        // sum           18

        assert_eq!(18, sum);
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}

// *** Creating Our Own Iterators with the Iterator Trait
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// ANCHOR: here
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
// ANCHOR: end

// *** Methods that consume the iterator
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let total: i32 = v1.iter().sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
