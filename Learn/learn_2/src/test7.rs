use core::fmt;

pub fn test1() -> () {
    println!("hello");
    println!("hello ,{}", "world");
    println!("The number is {}", 1);
    println!("{:?}", (3, 4));
    println!("{value}", value = 4);
    println!("{},{}", 1, 2);
    println!("{:04}", 42);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "牛不牛啊,我是{},我tm{}了!", self.name, self.age)
    }
}

pub fn test2() -> () {
    let s = format!(
        "Person:{:?}",
        Person {
            name: "hhh".to_string(),
            age: 23
        }
    );

    println!("{s}");
    println!(
        "{}",
        Person {
            name: String::from("asdsad"),
            age: 23
        }
    );
}

struct Array(Vec<i32>);
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Array:{:?}", self.0)
    }
}

pub fn test3() -> () {
    let vec = Array(vec![1, 2, 657, 234, 42345, 65, 656]);
    println!("{}", vec);
}

pub fn test4() -> () {
    println!("{},{}", 1, 2);
    println!("{1},{0}", 1, 2);
    println!("{0},this is {1},{1},this is {0}", "Allice", "Bob");
    println!("{1}{}{0}{}", 1, 2);
    println!("{:z>1$}", "x", 10);
    println!("{:+}", 5);
    println!("{:05}", 5);
    println!("{:05}", -5);
    println!("{:^1$}", "#$%345", 20);
    println!("{:^8}", "#$544");
    println!("{:+}", -5);
    println!("hello,{:.*}", 3, "2354234");
    println!("{:#b}", 12345);
    println!("{:#o}", 12345);
    println!("{:#x}", 12345);
    println!("{:x}", 12345);
    println!("{:010b}", 23);

    println!("{:2e}", 1000000);
    println!("{:2E}", 1000000);

    let v = vec![1, 2, 3];
    println!("{:p}", v.as_ptr());
}
