pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess(i32);

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100,got {}", value);
        }
        Guess(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // 该包仅能用于测试
    #[test]
    fn it_works() {
        println!("it workes");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works2() {
        assert_eq!(5, 5);
    }

    #[test]
    fn another() {
        // panic!("Make the test fail");
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("gouxiong");
        assert!(
            result.contains("gouxiong"),
            "您的问候中并没有包含目标姓名 {},你的问候是 {}",
            "狗熊",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_workes3() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    #[ignore]
    fn it_works() {
        println!("it workes");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn addtwo(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod test3 {
    use super::*;
    #[test]
    fn internal() {
        addtwo(2, 3);
    }
}
