use num::integer;

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        // Circle {
        //     x: x,
        //     y: y,
        //     radius: radius,
        // }
        Self { x, y, radius }
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn x(&self) -> f64 {
        return self.x;
    }

    pub fn y(&self) -> f64 {
        return self.y;
    }
}

#[derive(Debug)]
pub enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// 为 TrafficLightColor 实现所需的方法
impl TrafficLightColor {
    pub fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}

pub fn test() {
    let t = Circle::new(5 as f64, 6 as f64, 7 as f64);
    let t2 = Circle {
        x: 4 as f64,
        y: 5 as f64,
        radius: 6 as f64,
    };
    println!("{}", t.radius);
    (&t).x();
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn test2() {
    let a = 5;
    let b = 10;
    println!("{}+{}={}", a, b, add::<i32>(a, b));

    let b: i32 = 100.into();
    println!("{}", b);
}

#[derive(Debug)]
struct Points<T> {
    x: T,
    y: T,
}

pub fn test3() {
    let integer = Points { x: 5, y: 7 };
    let float = Points { x: 1.0, y: 4.0 };
    println!("{:?},{:?}", integer, float);
    println!("{}", integer.x());
    println!("{}", integer.xx());
}

impl<T> Points<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Points<i32> {
    fn xx(&self) -> i32 {
        self.x * 33
    }
}
// 数组[T;2]和数组[T;3]是完全不同的类型,好比int和String
// 所以如果直接(arr:[T]),[T]在编译时大小不确定!
// 而且[T;1]....[T:N]都是不同类型,没办法分配空间也就报错
// 所以只能是切片引用&[T],是一个胖指针,包含了长度,运行时确定.
fn display_array1<T: std::fmt::Display + std::fmt::Debug>(arr: &[T]) {
    for i in arr {
        println!("{}", i);
    }
}
// 如果在某些场景下引用不适宜用或者干脆不能用呢？
fn display_array2<T: std::fmt::Display + std::fmt::Debug, const N: usize>(arr: [T; N]) {
    for i in arr {
        println!("{}", i);
    }
}

pub enum Assert<const CHECK: bool> {
    //
}

pub trait IsTrue {
    //
}

impl IsTrue for Assert<true> {
    //
}

pub fn test4() {
    let arr: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 4] = [1, 2, 3, 4];
    display_array1(&arr);
    display_array1(&arr2);
    display_array2(arr);
    display_array2(arr2);

    println!("{:?}:{:?}", arr, arr2);

    const SIZE: usize = compute_buffer_size(4);
    let buffer = Buffer::<SIZE> { data: [0; SIZE] };
    println!("Buffer size: {} bytes", buffer.data.len());
}

struct Buffer<const N: usize> {
    data: [u8; N],
}

const fn compute_buffer_size(factor: usize) -> usize {
    factor * 1024
}
