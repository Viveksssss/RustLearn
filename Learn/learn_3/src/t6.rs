use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn test1() {
    let w = Wrapper(vec![String::from("ASdsad"), String::from("Asdsadasdasd")]);
    println!("{}", w);

    // 类型标注复杂
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
        Box::new(|| {})
    }

    // 使用别名:清晰
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type2(f: Thunk) {
        // --snip--
    }

    fn returns_long_type2() -> Thunk {
        // --snip--
        Box::new(|| {})
    }
}

pub fn test2() {
    // error
    // Rust 需要明确地知道一个特定类型的值占据了多少内存空间，同时该类型的所有值都必须使用相同大小的内存。如果 Rust 允许我们使用这种动态类型，那么这两个 str 值就需要占用同样大小的内存，这显然是不现实的: s1 占用了 12 字节，s2 占用了 15 字节，总不至于为了满足同样的内存大小，用空白字符去填补字符串吧？
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // ok
    let s3: &str = "on?";

    // fn foobar_1(thing: &dyn MyThing) {}     // OK
    // fn foobar_2(thing: Box<dyn MyThing>) {} // OK
    // fn foobar_3(thing: MyThing) {}          // ERROR!
}

/**
trait MyThing {
    fn do_something(&self);
}

struct Small {
    data: u8,
}
impl MyThing for Small {
    fn do_something(&self) {}
}

struct Large {
    data: [u8; 1000],  // 1000 字节
}
impl MyThing for Large {
    fn do_something(&self) {}
}

// 如果这个允许编译：
fn foobar_3(thing: MyThing) {
    // thing 应该占用多少栈空间？
    // 如果传入 Small：1 字节
    // 如果传入 Large：1000 字节
    // 编译器无法确定！
}
*/
pub fn test3() {
    // 先，Box<str> 使用了一个引用来指向 str，嗯，满足了第一个条件。但是第二个条件呢？Box 中有该 str 的长度信息吗？显然是 No。那为什么特征就可以变成特征对象？其实这个还蛮复杂的，简单来说，对于特征对象，编译器无需知道它具体是什么类型，只要知道它能调用哪几个方法即可，因此编译器帮我们实现了剩下的一切。
    // let s1: Box<str> = Box::new("Hello there!" as str);

    let s2: Box<&str> = Box::new("hello world");
    println!("{}", s2);
    println!("{}", &*s2);
    println!("{}", &**s2);
    println!("{:p}", &*s2 as *const &str);
    println!("{:p}", &**s2 as *const str);
    println!("{:p}", &**s2 as *const str);

    let s3: &str = &*s2;
    let s4: &str = &s2;
    let s5: &str = &s2;
    println!("{}", s3);
    println!("{}", s4);
    println!("{}", s5);

    let tmp = &**s2 as *const str;
    let (addr, len) = unsafe {
        let s: &str = &*tmp;
        println!("{}", s);
        println!("{}", s.len());
        println!("{:p}", s.as_ptr());

        let (addr, len) = std::mem::transmute::<*const str, (usize, usize)>(tmp);
        (addr, len)
    };

    unsafe {
        let ptr: *const str = std::ptr::slice_from_raw_parts(addr as *const u8, len) as *const str;
        println!("{}", &*ptr);
    }
}
