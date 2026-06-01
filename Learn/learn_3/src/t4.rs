use std::collections::HashMap;

/**
.iter() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&T)
.iter_mut() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&mut T)，因此在 if let Some(v) = values_iter_mut.next() 中，v 的类型是 &mut i32，最终我们可以通过 *v = 0 的方式修改其值

into_iter 会夺走所有权
iter 是借用
iter_mut 是可变借用
*
*/
pub fn test1() {
    let arr = [1, 2, 3, 4];
    for i in arr {
        println!("{}", i);
    }

    for i in arr.into_iter() {
        println!("{}", i);
    }

    let mut arr_iterator = arr.into_iter();
    assert_eq!(arr_iterator.next(), Some(1));

    for i in arr.into_iter() {
        println!("{}", i);
    }

    for i in &arr {
        println!("{}", i);
    }

    let values = vec![
        String::from("dsada"),
        String::from("qwe"),
        String::from("xxzczxc"),
    ];
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(v) => {
                        println!("{}", v)
                    }
                    None => break,
                }
            },
        };
    }

    for i in arr.into_iter().into_iter().into_iter() {
        println!("{}", i);
    }
}

/**
消费者适配器:只要迭代器上的某个方法 A 在其内部调用了 next 方法，那么 A 就被称为消费者适配器：因为 next 方法会消耗掉迭代器上的元素，所以方法 A 的调用也会消耗掉迭代器上的元素。


其中一个例子是 sum 方法，它会拿走迭代器的所有权，然后通过不断调用 next 方法对里面的元素进行求和
*/

pub fn test2() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("{:?}", v1);

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}",v1_iter);
}

/**
迭代器适配器:既然消费者适配器是消费掉迭代器，然后返回一个值。那么迭代器适配器，顾名思义，会返回一个新的迭代器，这是实现链式方法调用的关键：v.iter().map().filter()...


与消费者适配器不同，迭代器适配器是惰性的，意味着你需要一个`消费者适配器来收尾`，最终将迭代器转换成一个具体的值
 */

pub fn test3() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // 这里map 方法是一个迭代者适配器，它是惰性的，不产生任何行为，
    // v1.iter().map(|x| x + 1);
    // 因此我们还需要一个消费者适配器进行收尾
    // 不能使用as,因为as只能转换基本类型和指针
    // let v2 = (v1.iter().map(|x| x + 1).collect()) as Vec<i32>;
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", folks);

    let v3: HashMap<_, _> = folks.iter().filter(|(k, v)| k.len() == 7).collect();
    println!("{:?}", v3);
}

struct Counter {
    counter: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { counter: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < 5 {
            self.counter += 1;
            Some(self.counter)
        } else {
            None
        }
    }
}

pub fn test4() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let sum: i32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| {
            println!("{}:{}", a, b);
            a * b
        })
        .filter(|x| {
            println! {"{}",x};
            x % 3 == 0
        })
        .take(1)
        .sum();

    println!("{}", sum);
}

pub fn test5() {
    let v = vec![1, 2, 3, 4, 5, 6];
    for (i, v) in v.iter().enumerate() {
        println!("{}:{}", i, v);
    }
}

#[cfg(target_os = "windows")]
fn get_os_info() {
    println!("这是 Windows 系统");
}

// 多个条件：任意满足一个即可（OR）
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn get_os_info() {
    println!("这是 Unix 类系统");
}

cfg_if::cfg_if! {
    if #[cfg(target_os="linux")]{
        fn get_path()->&'static str{"/usr/share"}
    }else if #[cfg(target_os="windows")]{
        fn get_path()->&'static str{"C:\\ProgramData"}
    }else {
        fn get_path()->&'static str{"/opt"}
    }
}
pub fn test6() {
    get_os_info();
    println!("{}", get_path());
}
