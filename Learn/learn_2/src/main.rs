// use num::complex::Complex;

// fn main_1() {
//     let a = Complex { re: 2.1, im: -1.2 };
//     let b = Complex::new(11.1, 22.2);
//     let result = a + b;

//     println!("{} + {}i", result.re, result.im)
// }
// fn add_with_extra(x: i32, y: i32) -> i32 {
//     let x = x + 1;
//     let y = y + 1;
//     x + y
// }

// fn resc(x: i32) -> i32 {
//     let x = if x > 1 { 25 } else { 255 };
//     x
// }

// fn main_2() {
//     let res = add_with_extra(5, 6);
//     // println!("{}", res);
//     let y = {
//         let k = 20;
//         let c = 300;
//         let w = (c << 8) + k;
//         w
//     };
//     println!("{}：{}", res, y);

//     let k: i32 = resc(56);
//     println!("{}", k);
//     println!("{}", resc(-2));
// }

// fn func1(x: i32, y: i32) {
//     return ();
// }

// fn func2(x: i32) -> i32 {
//     if x > 5 {
//         return x - 5;
//     }
//     return x;
// }

// fn func3(x: i32) -> i32 {
//     if x > 5 {
//         return x;
//     }
//     x << 8
// }

// fn dead_end() -> i32 {
//     // panic!("end");
//     loop {
//         return 5;
//     }
// }

// fn main_3() {
//     // dead_end();
//     let e: i32;
//     e = 23;
//     // func1(5, 23);
//     println!("{},{}", func2(23), e);
//     println!("{}", func3(5));
// }

// fn main_4() {
//     let mut s = String::from("hello");
//     s.push_str(",world");
//     println!("{}", s);

//     let c = s.clone();
//     println!("{}", c);
//     println!("{}", s);

//     let c = &s;
//     // s.push_str("string");
//     println!("{}", c);

//     let x: &str = "ASdasdasd";
//     let x2 = x;
//     println!("{},{}", x, x2);

//     let mut xx: &str = "asdsa";
//     let xx2 = xx;
//     xx = "$#534534534534";
//     println!("{},{}", xx, xx2);

//     let mut s: String = String::from("hell");
//     let ss = &mut s;
//     ss.push_str("o");
//     println!("{}", ss);

//     let mut xx = String::from("asdas");
//     let yy = &mut xx;
//     println!("{}", yy);
//     let cc = &mut xx;
//     println!("{}", cc);
//     // println!("{:p}:{:p}", &xx, yy);
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email:String,username :String) -> User{
//     User{
//         email,
//         username,
//         active:true,
//         sign_in_count:1
//     }
// }
// fn main() {
//     // let a = [1,2,3,4,5];
//     // let slice = &a[1..3];
//     // assert_eq!(slice, &[2, 3]);
//     // let mut name = String::from("中国人");
//     // let hello = &name[0..3];
//     // println!("{}", hello);
//     // println!("{}", &name[..]);
//     // name.clear();

//     // let tup = (500,6.4,1);
//     // let x= tup.2;
//     // println!("{}",x);

//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user3 = User{
//         sign_in_count:2,
//         ..user1
//     };

//     let mut user2 = build_user(String::from("sadas"),String::from("asdsad"));
//     user2.active = false;
//     println!("{:#?}:{:#?}",user1.active,user2.active);
// }

// use crate::List::*;

// enum List {
//     // Cons: Tuple struct that wraps an element and a pointer to the next node
//     Cons(u32, Box<List>),
//     // Nil: A node that signifies the end of the linked list
//     Nil,
// }

// // Methods can be attached to an enum
// impl List {
//     // Create an empty list
//     fn new() -> List {
//         // `Nil` has type `List`
//         Nil
//     }

//     // Consume a list, and return the same list with a new element at its front
//     fn prepend(self, elem: u32) -> List {
//         // `Cons` also has type List
//         Cons(elem, Box::new(self))
//     }

//     // Return the length of the list
//     fn len(&self) -> u32 {
//         // `self` has to be matched, because the behavior of this method
//         // depends on the variant of `self`
//         // `self` has type `&List`, and `*self` has type `List`, matching on a
//         // concrete type `T` is preferred over a match on a reference `&T`
//         // after Rust 2018 you can use self here and tail (with no ref) below as well,
//         // rust will infer &s and ref tail.
//         // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
//         match *self {
//             // Can't take ownership of the tail, because `self` is borrowed;
//             // instead take a reference to the tail
//             Cons(_, ref tail) => 1 + tail.len(),
//             // Base Case: An empty list has zero length
//             Nil => 0
//         }
//     }

//     // Return representation of the list as a (heap allocated) string
//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {
//                 // `format!` is similar to `print!`, but returns a heap
//                 // allocated string instead of printing to the console
//                 format!("{}, {}", head, tail.stringify())
//             }
//             Nil => {
//                 format!("Nil")
//             }
//         }
//     }
// }

// fn main() {
//     // Create an empty linked list
//     let mut list = List::new();

//     // Prepend some elements
//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);

//     // Show the final state of the list
//     println!("linked list has length: {}", list.len());
//     println!("{}", list.stringify());
// }

// use std::ops::IndexMut;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
// }

// fn main() {
//     // 编译器自动推导出one的类型
//     let one = [1, 2, 3];
//     print_type_of(&one);
//     println!("{}", one.get(2).unwrap());

//     for (i, v) in one.iter().enumerate() {
//         println!("{},{}", i, v);
//     }

//     let mut n = 0;
//     loop {
//         if n > 5 {
//             break;
//         }

//         println!("{}", n);
//         n += 1;
//     }

//     let mut count = 0;
//     'outer: loop {
//         'inner: loop {
//             count += 1;
//             if count == 20 {
//                 break 'inner;
//             }
//         }

//         'innner: loop {
//             count -= 1;
//             if count == 10 {
//                 break 'outer;
//             }
//         }
//     }
//     // 显式类型标注
//     // let two: [u8; 3] = [1, 2, 3];
//     // let blank1 = [0; 3];
//     // let blank2: [u8; 3] = [0; 3];

//     // // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
//     // let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

//     // print_type_of(&arrays[0]);

//     // 借用arrays的元素用作循环中
//     //     for a in &arrays {
//     //         print!("{:?}: ", a);
//     //         // 将a变成一个迭代器，用于循环
//     //         // 你也可以直接用for n in a {}来进行循环
//     //         for n in a.iter() {
//     //             print!("\t{} + 10 = {}\t", n, n + 10);
//     //         }

//     //         let mut sum = 0;
//     //         // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
//     //         for i in 0..a.len() {
//     //             sum += a[i];
//     //         }
//     //         println!("\t({:?} = {})", a, sum);
//     //     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     if let Some(v) = six {
//         println!("{}", v);
//     }
// }

// fn main() {
//     let mut stack = Vec::new();
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);
//     stack.push(4);
//     stack.push(5);
//     stack.push(6);

//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }

//     let a = stack.pop();
//     println!("{}", a.unwrap_or(4));
// }

// fn main() {
//     let mut stack = Vec::new();
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);
//     stack.push(4);
//     stack.push(5);
//     stack.push(6);

//     for (index, value) in stack.iter().enumerate() {
//         println!("{}:{}", index, value);
//     }
// }

// fn get_item_from_str(s: &str) -> (u64, &str) {
//     let mut it = s.split(" ");
//     let (Some(_size), Some(item)) = (it.next(), it.next()) else {
//         panic!("Can't parse integer:'{s}'");
//     };
//     let size: u64 = item.len() as u64;
//     (size, item)
// }
// fn main() {
//     let s = "hhh vivek";
//     let (size, s) = get_item_from_str(s);
//     println!("{size}:{s}");
// }

// use crate::test3::Summary;

// mod test1;
// mod test2;
// mod test3;

// fn main() {
//     let post = test3::Post {
//         title: "Rust语言简介".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust棒极了!".to_string(),
//     };
//     let weibo = test3::Weibo {
//         username: "sunface".to_string(),
//         content: "好像微博没Tweet好用".to_string(),
//     };

//     println!("{}", post.summary());
//     println!("{}", weibo.summary());
//     test3::test_for_impl(&post);
//     test3::test_for_impl(&weibo);

//     test3::test5();
// }

#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

mod test3;
mod test4;
mod test5;
mod test6;
mod test7;

use std::alloc::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::{self, Write};
fn main() {
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("读取失败");
    // let numbers: Vec<_> = input
    //     .trim()
    //     .split_whitespace()
    //     .map(|s| s.parse::<i64>().expect("请输入有效整数"))
    //     .collect();
    // println!("what you input is :{:?}", &numbers);
    test7::test1();
    test7::test2();
    test7::test3();
    test7::test4();
}
