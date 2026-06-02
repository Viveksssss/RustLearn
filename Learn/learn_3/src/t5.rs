pub fn test1() {
    println!("{}", i8::MAX);
    println!("{}", 'a' as i8);

    let mut x = 42;
    let ptr: *mut i32 = &mut x; // 指向 x 的可变裸指针,指向的内容可变

    let a = 10;
    let b = 20;
    let mut ptr: *const i32 = &a; // 指针本身可变

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 10;
    }
    println!("{} {} {}", p1 as usize, first_address, second_address);
    println!("{}", values[1]);
}

use std::{collections::btree_map::Values, convert::TryInto, mem, rc::Rc, sync::Arc};

pub fn test2() {
    let a: u8 = 10;
    let b: u16 = 1500;
    let b_: u8 = match b.try_into() {
        Ok(k1) => k1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    if a < b_ {
        println!("Ten iss less than one hundard");
    }
}

struct Foo {
    x: u32,
    y: u16,
}

struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}
//Rust 的规则：即使 T 能转成 U，Something<T> 也不会自动转成 Something<U>。

//当你写 value.foo() 时，Rust 编译器会按顺序尝试 4 种方式 来让这个调用合法：
// 直接值调用：T::foo(value)
// 自动引用：<&T>::foo(value) 或 <&mut T>::foo(value)
// 自动解引用：U::foo(*value)（如果 T 可以解引用成 U）
// 定长变不定长：比如 [i32; 2] 变成 [i32]

pub fn test3() {
    let array: Rc<Box<[i32; 3]>> = Rc::new(Box::new([1, 2, 3]));
    let first_entry = array[0];
    println!("{}", first_entry);
}
/*
    这里T实现了Copy特征,会先调用fn clone(&T)->T,进行值调用
*/
fn do_stuff<T: Clone + std::fmt::Display>(value: &T) {
    let cloned = value.clone();

    // 0x7ffe4150a240
    // 0x7ffe4150a288
    // 0x7ffe4150a2c0
    // 0x7ffe4150a300
    // 0x7ffe4150a348
    // 0x7ffe4150a240
    // 0x7ffe4150a4a0
    // 0x7ffe4150a4a0
    println!("{:p}", &cloned);
    println!("{:p}", &&cloned);
    println!("{:p}", &&&cloned);
    println!("{:p}", &&&&&cloned);
    println!("{:p}", &&&&&&cloned);
    println!("{:p}", &cloned as *const T);
    println!("{:p}", value);
    println!("{:p}", value as *const T);
    println!("{}", *value);
}

fn do_stuff2<T>(value: &T) {
    let cloned = value.clone();
    // 0x7ffe4150a4a0
    // 0x7ffe4150a428
    println!("{:p}", cloned);
    println!("{:p}", &cloned);
}

pub fn test4() {
    let value = String::from("123");
    do_stuff(&value);
    do_stuff2(&value);
}

#[derive(Clone)]
struct Container<T>(Arc<T>);

fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_cloned = foo.clone();
    let bar_cloned = bar.clone();
}

pub fn test5() {
    let a: f32 = 5.7;
    let b: i32 = unsafe { mem::transmute(a) };
    println!("{}", a);
    println!("{}", b);
}

fn foo() -> i32 {
    129
}

pub fn test6() {
    let pointer = foo as *const ();
    let function = unsafe { std::mem::transmute::<*const (), fn() -> i32>(pointer) };
    println!("{}", function());
}

// 演长生命周期
struct R<'a>(&'a i32);

unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    std::mem::transmute::<R<'b>, R<'static>>(r)
}

unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
    std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
}

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|x| x.trim())
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
pub fn test7() {
    let p = Point::from_str("(3, 4)");
    assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

    println!("Success!")
}
