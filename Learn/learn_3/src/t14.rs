#[macro_export]
macro_rules! vec{
    ($( $x:expr ),+$(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

use my_marco::Describe;
use my_marco::MyDefault;

pub fn test1() {
    let v = vec![23, 12312];
    for i in &v {
        println!("{}", i);
    }
}

pub fn test2() {
    #[derive(Describe)]
    struct Person {
        name: String,
        age: u32,
        email: String,
    }

    #[derive(Describe)]
    struct Point(i32, i32, i32);

    #[derive(Describe)]
    struct EmptyStruct;

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    person.describe();

    println!("\n---\n");

    let point = Point(1, 2, 3);
    point.describe();

    println!("\n---\n");

    let empty = EmptyStruct;
    empty.describe();
}

pub fn test3() {
    #[derive(MyDefault, Debug)]
    struct SomeData(u32, String);

    #[derive(MyDefault, Debug)]
    struct User {
        name: String,
        data: SomeData,
    }
    println!("{:?}", SomeData::default());
    println!("{:?}", User::default());
}


