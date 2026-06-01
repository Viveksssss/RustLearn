pub fn test1() {
    let x = 5;
    let sum = |y| x + y;

    assert_eq!(32, sum(27));
}

struct Cacher<T, E>
where
    T: Fn(E) -> E,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn test2() {
    let mut cacher = Cacher::new(|x| x) as Cacher<_, i32>;
    println!("The first call:{}", cacher.value(5));
    println!("The second call:{}", cacher.value(20));
    println!("The third call:{}", cacher.value(25));
}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

pub fn test3() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());
}

use std::thread;
pub fn test4() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector:{:?}", v);
    });
    handle.join().unwrap();

    let x = 42;
    let y = "hello";
    let z = vec![1, 2, 3];

    let closure = move || {
        println!("{}", x); // 只使用了 x
        // y 和 z 没有被使用
    };

    // x 的所有权被转移了
    // println!("{}", x);  // ❌ 错误！x 已被移动

    // y 和 z 仍然可用，因为没被闭包捕获
    println!("{}", y); // ✅ 可以
    println!("{:?}", z); // ✅ 可以
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

/**
 为什么update_string没有用mut修饰却是一个可变类型的闭包？事实上，FnMut只是trait的名字，声明变量为FnMut和要不要mut没啥关系，FnMut是推导出的特征类型，mut是rust语言层面的一个修饰符，用于声明一个绑定是可变的。Rust从特征类型系统和语言修饰符两方面保障了我们的程序正确运行。
*/
pub fn test5() {
    // let mut s = String::new();
    // let mut update_string = |str| s.push_str(str);
    // update_string("hello");
    // println!("{:?}", s);

    let mut s = String::new();
    let update_string = |str| s.push_str(str);

    exec(update_string);
    // exec(update_string); // 闭包本身也是一个变量,也会被消耗转移
    println!("{:?}", s);
}

/**
所有的变量都是和数据进行"绑定"的关系,如果是引用,那么绑定的就是指针,如果转移了所有权,那么就是绑定的真实的数据.

比如let x = 5;这里实际上不是这个5不能修改,而是这个变量x是不可变绑定,绑定了5之后就不能再绑定其他的数据了.

而比如let mut x = 5;意思是,这个x可以绑定其他的变量,比如x = 10;意味着,原来这个x绑定的5,现在绑定成了10,而不是把原来的5修改成了10?
*/
pub fn test(mut s: String) {}
pub fn testt(s: String) {}
pub fn test6() {
    let s = String::new();
    // test(s);
    testt(s);
    // println!("{}",s); // 转移所有权
}

/**
一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们

这个Fn和FnMut,如果捕获的变量本是就是不可变,那么捕获的s就是&s不可变引用,然后这个闭包就是Fn,如果捕获的变量本身可变,就是&mut s,那么闭包本身就是FnMut,如果我们在闭包内部转移了所有权,那么闭包本身就是FnOnce

FnMut闭包底层实现原理中使用了&mut self,所以如果不声明为mut,无法提供&mut self参数,所以无法调用
所以一般FnMut闭包都会声明为mut.
*/
pub fn test7() {
    // 1. 捕获不可变引用 → Fn
    let s = String::from("hello");
    let closure = || println!("{}", s); // s 是 &String
    // closure: impl Fn()

    // 2. 捕获可变引用 → FnMut
    let mut s = String::from("hello");
    let mut closure = || s.push_str(" world"); // s 是 &mut String
    // closure: impl FnMut()

    // 3. 转移所有权 → FnOnce
    // 无论是否写 move，编译器都会推断为 FnOnce。加上 move → 强制转移所有权到闭包
    // let s = String::from("hello");
    // let closure = move || {
    // let _x = s;  // s 已经通过 move 被捕获了
    // };

    /*
       move 关键字强制闭包无论内部如何操作，都通过转移所有权来捕获变量，而不是借用。
    */
    let s = String::from("hello");
    let closure = || {
        println!("{}", s);
    };
    println!("{}", s); // 可以用/

    // let closure = move || {println!("{}",s);} // 强制move转移s所有权,
    // 外部的println("{}",s)不可用了.
    // closure: impl FnOnce()
}
