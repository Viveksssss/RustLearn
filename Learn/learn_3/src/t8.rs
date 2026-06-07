// 智能指针
pub fn test1() {
    let mut a = Box::new(3);
    println!("{}", a);
    let b = a.clone();
    println!("{}", a);
    println!("{}", b);

    let arr = [0; 1000];
    let arr1 = arr;
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let aa = vec![String::from("213"); 1000];
    let bb = aa;
    // println!("{:?}",aa); // aa has been boorrowed
    println!("{:?}", bb);
}

// recursive
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// Box的大小是固定的
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 异质数组1
trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{}", self.id);
    }
}

struct Select {
    s: String,
}

impl Draw for Select {
    fn draw(&self) {
        println!("{}", self.s)
    }
}

pub fn test2() {
    let elems: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { id: 1 }),
        Box::new(Select {
            s: String::from("sss"),
        }),
    ];

    for e in &elems {
        e.draw();
    }
}

// 异质数组2
use std::any::Any;
pub fn test3() {
    let arr: [Box<dyn Any>; 3] = [Box::new(42), Box::new(22.5), Box::new("23232")];

    for e in &arr {
        if let Some(v) = e.downcast_ref::<i32>() {
            println!("i32:{}", v);
        } else if let Some(v) = e.downcast_ref::<f64>() {
            println!("f64:{}", v);
        } else if let Some(v) = e.downcast_ref::<&str>() {
            println!("str:{}", v);
        } else {
            println!("any");
        }
    }

    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    println!("{:p}:{:p}", *first, *second);
    let sum = **first + **second;
    println!("{}", sum);

    println!("{}", generate_static_str());
}

fn generate_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("helloworld");
    println!("{}", &s);

    Box::leak(s.into_boxed_str())
}

// Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
// Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::ops::DerefMut;

// 要实现 DerefMut 必须要先实现 Deref 特征：pub trait DerefMut: Deref
// T: DerefMut<Target=U> 解读：将 &mut T 类型通过 DerefMut 特征的方法转换为 &mut U 类型，对应上例中，就是将 &mut MyBox<String> 转换为 &mut String
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn display(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}

pub fn test4() {
    let y = MyBox::new(5);
    // 没有为MyBox实现Deref之前:报错
    // assert_eq!(5,*y);
    // 实现之后
    assert_eq!(5, *y); //自动解引用:*(y.deref())
    // let a : i32 = y;  // 不会自动隐式转换手动:*y
    // 只有特定的地方（方法调用、运算符等）才会触发 Deref。
    let s = MyBox::new(String::from("hello"));
    println!("{}", s.len()); // 自动 deref 为 &String，然后调用 len()

    let s = MyBox::new(String::from("hello, world"));
    let s1: &str = &s;
    let s2: String = s.to_string();

    let mut s = MyBox::new(String::from("hello,"));
    display(&mut s);
}

struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!");
    }
}

pub fn test5() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let _foo = Foo;
    println!("Running!");
}

pub fn test6() {
    let foo = Foo;
    // foo.drop();
    // println!("Running!:{:?}", foo);

    // error[E0040]: explicit use of destructor method
    //   --> src/main.rs:37:9
    //    |
    // 37 |     foo.drop();
    //    |     ----^^^^--
    //    |     |   |
    //    |     |   explicit destructor calls not allowed
    //    |     help: consider using `drop` function: `drop(foo)`
    drop(foo);
    // 事实上，能被显式调用的drop(_x)函数只是个空函数，在拿走目标值的所有权后没有任何操作。而由于其持有目标值的所有权，在drop(_x)函数结束之际，编译器会执行_x真正的析构函数，从而完成释放资源的操作。换句话说，drop(_x)函数只是帮助目标值的所有者提前离开了作用域。

    /*
    标准库中 drop 的实际定义
    pub fn drop<T>(_x: T) {
        // 什么都不做！
    }
    */
}
use std::rc::Rc;

pub fn test7() {
    let s = String::from("!helloworld");
    // let a = Box::new(s);
    // let b = Box::new(s); // err,a has been moved
    // println!("{}",b);

    let a = Rc::new(String::from("www"));
    let b = Rc::clone(&a);
    let c = a.clone();
    println!("{}{}{}", a, b, c);
    println!("Rc 指针地址:");
    println!("{:p}\n{:p}\n{:p}", &*a, &*b, &*c);
    println!("\n内部 String 的地址:");
    println!("{:p}\n{:p}\n{:p}", &&*a, &&*b, &&*c);
    println!("\nRc 变量地址:");
    println!("{:p}", &a);
    println!("{:p}", &b);
    println!("{:p}", &c);
    println!("----------------");
    println!("{:p}\n{:p}\n{:p}\n{:p}", &a, &&a, &&&a, &&&&a);
    println!("{:p}\n{:p}\n{:p}\n{:p}", &*a, &&*a, &&&*a, &&&&*a);
    println!("{}", Rc::strong_count(&a));
}
use std::sync::Arc;
use std::thread;

pub fn test8() {
    // Rc不可使用在多线程,不安全
    // let s = Rc::new(String::from("多线程漫游者"));
    // for _ in 0..10 {
    //     let s = Rc::clone(&s);
    //     let handle = thread::spawn(move || {
    //        println!("{}", s)
    //     });
    // }

    let s = Arc::new(String::from("多线程漫游者"));
    let mut handles = vec![];
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || println!("{}", s));
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

use std::cell::Cell;
use std::cell::RefCell;

// 由于 Rust 的 mutable 特性，一个结构体中的字段，要么全都是 immutable，要么全部是 mutable，不支持针对部分字段进行设置。比如，在一个 struct 中，可能只有个别的字段需要修改，而其他字段并不需要修改，为了一个字段而将整个 struct 变为 &mut 也是不合理的。

// 所以，实现 内部可变性 的 Cell 和 RefCell 正是为了解决诸如这类问题存在的，通过它们可以实现 struct 部分字段可变，而不用将整个 struct 设置为 mutable。

pub fn test9() {
    // 由于 Cell 类型针对的是实现了 Copy 特征的值类型，因此在实际开发中，Cell 使用的并不多，因为我们要解决的往往是可变、不可变引用共存导致的问题，此时就需要借助于 RefCell 来达成目的。
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{},{}", one, two);

    // 可以看出，Rc/Arc 和 RefCell 合在一起，解决了 Rust 中严苛的所有权和借用规则带来的某些场景下难使用的问题。但是它们并不是银弹，例如 RefCell 实际上并没有解决可变引用和引用可以共存的问题，只是将报错从编译期推迟到运行时，从编译器错误变成了 panic 异常：
    let s = RefCell::new(String::from("hello, world"));
    {
        let s1 = s.borrow();
        println!("{}", s1);
    }
    // let s2 = s.borrow_mut();
    match s.try_borrow_mut() {
        Ok(mut writer) => {
            *writer = String::from("SAdasd");
            println!("修改成功");
        }
        Err(_) => {
            println!("当前有人在使用，稍后重试");
            // 可以稍后重试或使用其他策略
        }
    }
    println!("{}", s.borrow());
    // 不会报错,但是运行时panic
}

pub fn test10() {
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    let f = x.clone();
    let ss = x.get();
    println!("{}", ss);
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());
    println!("{}", f.get());
}

// Rc + RefCell 组合使用
pub fn test11() {
    // 前者可以实现一个数据拥有多个所有者，后者可以实现数据的可变性
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    // 内存损耗
    // struct Wrapper<T> {
    //     // Rc
    //     strong_count: usize,
    //     weak_count: usize,

    //     // Refcell
    //     borrow_count: isize,

    //     // 包裹的数据
    //     item: T,
    // }

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s1.borrow_mut().push_str(":hello!");
    s2.borrow_mut().push_str(", oh yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

use std::cell::Cell;

pub fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}
