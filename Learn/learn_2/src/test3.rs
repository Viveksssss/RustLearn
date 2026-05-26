use std::{fmt::Display, iter::once};

pub trait Summary {
    fn summary(&self) -> String {
        format!("Read more...")
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summary(&self) -> String {
        format!("文章{},作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    // fn summary(&self) -> String {
    //     format!("{}发表了微博{}", self.username, self.content)
    // }
}

pub fn test_for_impl(item: &impl Summary) {
    println!("Breaking news! {}", item.summary());
}
pub fn test_for_impl_real<T>(item: &T) -> ()
where
    T: Summary + Display,
{
    println!("Breaking news! {}", item.summary());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from("m1 max太厉害了，电脑再也不会卡"),
    }
}

pub fn test5() {
    let a: i32 = 10;
    let b: i16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred");
    }
}

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
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

#[derive(Debug, PartialEq)]
struct Points {
    x: i32,
    y: i32,
}

impl std::ops::Add for Points {
    type Output = Points;
    fn add(self, other: Points) -> Points {
        Points {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn test6() {
    println!(
        "{:?} == {:?}",
        Points { x: 1, y: 0 } + Points { x: 2, y: 3 },
        Points { x: 3, y: 3 }
    );
}
#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl std::ops::Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub fn test7() {
    println!("{:?}", Millimeters(8) + Meters(5));
}

#[derive(Debug)]
struct Container<T = i32> {
    items: Vec<T>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Container { items: vec![] }
    }

    fn add(&mut self, item: T) {
        self.items.push(item)
    }
}

fn process_container(c: &Container) -> i32 {
    c.items.iter().sum()
}

fn create_add_fill() -> Container {
    let mut c = Container::new();
    c.add(1);
    c.add(2);
    c.add(3);
    c
}

// 🆕 新代码可以使用其他类型
fn process_float_container(c: &Container<f64>) -> f64 {
    c.items.iter().sum()
}
pub fn test8() {
    let container = create_add_fill();
    println!("Sum:{}", process_container(&container));

    let mut float_container: Container<f64> = Container::<f64>::new();
    float_container.add(1.5);
    float_container.add(2.5);
    println!("{:?}", process_float_container(&float_container));
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn test9() {
    println!("A baby dog is called a {}", Dog::baby_name());

    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Pp {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Pp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl OutlinePrint for Pp {}

struct Wrapper(Vec<String>);

// 实现Deref可以将Wrapper当作Vec<String>使用,自动解包
impl std::ops::Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 在外部类型上实现外部特征(newtype)
impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.join(", "))
    }
}

pub fn test11() {
    let w = Wrapper(vec![String::from("hello"), String::from("world!")]);
    println!("{}", w);
}
