use std::{slice::from_raw_parts, str::from_utf8_unchecked};

pub fn get_memory_location() -> (usize, usize) {
    let string = "HelloWorld!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

pub fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

pub fn test1() {
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!("The {} bytes at 0x{:X} stored:{}", length, pointer, message);
}

/**
局部变量 t 的生命周期很短，为什么它的类型 i32 能叫 'static？
关键在于：'static 约束检查的是类型的"结构"，而不是值的实际存活时间。
如果 Rust 把 'static 改名为 Owned 或 NoBorrow，可能更容易理解：
'static 不是说"这个值要活到程序结束"，而是说"这个类型不包含借用，所以这个值可以独立存活任意久"。
*/
use std::fmt::Debug;
pub fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is : {:?}", input);
}
pub fn printt<T>(input: &'static T)
where
    T: Debug + ?Sized,
{
    println!("'static value passed in is : {:?}", input);
}

/**
这里的 T: 'static 意味着：

类型 T 可以安全地被持有任意长时间

T 不包含任何生命周期短于 'static 的引用

为什么 i32 满足 'static
i32 是一个拥有所有权的类型，它不包含任何引用，因此自动满足 'static 约束：
*/
pub fn print_it2<T: Debug + 'static>(input: T) {
    println!("'static value passed in is : {:?}", input);
}

// 这里的 T: 'static 约束的是类型 T 本身，而不是引用 &T 的生命周期。
pub fn print_it2_with_ref<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is : {:?}", input);
}

pub fn test2() {
    let t = 5;
    let s = String::from("asdsadas");
    // printt(t);
    print_it("asdasd");
    print_it(t);
    print_it(s);
    print_it2("asdasd");
    print_it2_with_ref(&t);
}

pub fn test3() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 当 `static_string` 超出作用域时，该引用不能再被使用，但是数据依然会存在于 binary 所占用的内存中
    }

    // println!("static_string reference remains alive: {}", static_string);
}

// &'static 要求被指向的数据真的能活到程序结束，而 T: 'static 只要求类型不包含借用。

/**
举个例子,上述的print_it和print_it2()实际上都是要求T:'static,所以i32虽然不能真的活的和程序一样久,但是满足不包含借用所以直接传入不报错.

假如使用&'static,那么就需要你传入的参数是引用,而且真的能和程序一样活的久,比如printt()
*/

pub fn test4() {
    let p = 5;
    print_it(p);
    // printt(&p); // 报错
    let p2: &'static i32 = &532;
    let p3: &'static str = "532";
    printt(p2); // 没问题
    printt(p3); // 没问题
}
