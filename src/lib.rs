//! # Art
//!
//! A library for modeling artistic concepts.

pub mod trial;
pub mod trial2;
pub mod chapters;


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(dead_code)]
struct Guess {
    value: i32
}


#[allow(dead_code)]
impl Guess {
    pub fn new(value:i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess values must be between 1 and 100, got {}.", value )
        }
        Guess {value}
    }
}


#[allow(dead_code)]
fn add_two(a:i32) -> Result< i32,String> {
    Ok(a+2)
}

#[allow(dead_code)]
pub fn greeting(name: &str) -> String {
   format!("Hello! {}", name)
}


pub fn add_two_other(a:i32) -> i32 {
    internal_adder(a,2)
}

// This is obviously private
#[allow(dead_code)]
fn internal_adder(a:i32, b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
   use super::*; // bring in outer mod scope into inner mod allowing private

    #[test]
    fn exploration() {
        assert_eq!(2+2, 4);
    }
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
    // #[tests]
    // fn hit_the_fan() {
    //     panic!("BLOW IT UP");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7};
        let smaller = Rectangle { width: 5, height: 1};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7};
        let smaller = Rectangle { width: 5, height: 1};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!( Ok(4), add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        )
    }


    #[test]
    #[should_panic(expected = "Guess values must be between 1 and 100, got 300.")]
    fn greater_than_100() {
        Guess::new(300);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
       if 2 + 2 ==4 {
           Ok(())
       } else {
           Err(String::from("two plus two != 4"))
       }
    }

    #[test]
    // You can also exclude tests with #[ignore]
    // The use case is they are still tests just those that are time consuming
    // so you can skip them to run them individually later or otherwise
    // then run with cargo tests -- --ignored
    fn add_to_err() -> Result<(), String> {
        if  add_two(2)? == 4 {
            Ok(())
        } else {
            Err(String::from("Broken"))
        }
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2,2));
    }
}


// chapter 14 example of docs
// re-exports
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here
        SecondaryColor::Orange
        // ANCHOR: here
    }
}