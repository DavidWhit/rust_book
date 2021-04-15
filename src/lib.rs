pub mod trial;
pub mod trial2;


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value:i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess values must be between 1 and 100, got {}.", value )
        }
        Guess {value}
    }
}

fn add_two(a:i32) -> Result< i32,String> {
    Ok(a+2)
}

fn greeting(name: &str) -> String {
   format!("Hello! {}", name)
}

#[cfg(test)]
mod tests {
   use super::*; // bring in outer mod scope into inner mod

    #[test]
    fn exploration() {
        assert_eq!(2+2, 4);
    }
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
    // #[test]
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
    fn add_to_err() -> Result<i32, String> {
        Ok(add_two(5)?)
    }
}