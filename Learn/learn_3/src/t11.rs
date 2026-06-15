// const MAX_ID: usize = usize::MAX / 2;
static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);

use std::sync::atomic::{AtomicUsize, Ordering};
pub fn test1() {
    // println!("maxid : {}", MAX_ID);
    // unsafe {
    //     REQUEST_REEV += 1;
    //     let local = REQUEST_REEV;
    //     println!("{}", local);
    // }
    for i in 0..100 {
        REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    }
    println!("{:?}", REQUEST_RECV);
}

// 全局计数器
struct Factory {
    factory_id: usize,
}

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

fn generate_id() -> usize {
    let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overfalowed");
    }
    next_id
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }
}

/*
运行期初始化:lazy_static

以上的静态初始化有一个致命的问题：无法用函数进行静态初始化，例如你如果想声明一个全局的Mutex锁：

但你又必须在声明时就对NAMES进行初始化，此时就陷入了两难的境地。好在天无绝人之路，我们可以使用lazy_static包来解决这个问题。

*/
use lazy_static::lazy_static;
use std::sync::Mutex;

// static NAMES: Mutex<String> = Mutex::new(String::from("generate from compilation period")); // Mutex::new 虽然是 const fn，但它的参数需要在编译期求值，而 String::from 涉及堆分配，只能在运行时进行。
lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("generate from compilation period"));
}

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        println!("{}", "initialization");
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}
pub fn test2() {
    // 首次访问`HASHMAP`的同时对其进行初始化
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // 后续的访问仅仅获取值，再不会进行任何初始化操作
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG: Option<&mut Config> = None;

pub fn test3() {
    // unsafe {
    //     // temporary value dropped while borrowed
    //     // creates a temporary value which is freed while still in use
    //     CONFIG = Some(&mut Config {
    //         a: "A".to_string(),
    //         b: "B".to_string(),
    //     });
    // }

    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        // 将c从内存中泄漏变成'static生命周期
        CONFIG = Some(Box::leak(c));
        // static mut 不允许直接共享引用,所以采用先取地址显然解引用的方法打印
        println!("{:?}", *std::ptr::addr_of!(CONFIG));
    }

    // 从函数中返回全局变量
    fn init() -> Option<&'static mut Config> {
        let c = Box::new(Config {
            a: "A".to_string(),
            b: "B".to_string(),
        });
        Some(Box::leak(c))
    }
    unsafe {
        CONFIG = init();
        println!("{:?}", *std::ptr::addr_of!(CONFIG));
    }
}

use std::sync::{Once, OnceLock};
#[derive(Debug)]
struct Logger;

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    fn global() -> &'static Logger {
        LOGGER.get_or_init(|| {
            println!("Logger is being created...");
            Logger
        })
    }

    fn log(&self, message: String) {
        println!("{}", message);
    }
}

pub fn test4() {
    let handle = std::thread::spawn(|| {
        let logger = Logger::global();
        logger.log("Thread message".to_string());
    });

    let logger = Logger::global();
    logger.log("some message".to_string());

    let logger2 = Logger::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}

use std::{sync::LazyLock, thread};

pub fn test5() {
    #[derive(Debug)]
    struct Logger;

    static LOGGER: LazyLock<Logger> = LazyLock::new(Logger::new);

    impl Logger {
        fn new() -> Logger {
            println!("Logger is being created...");
            Logger
        }

        fn log(&self, message: String) {
            println!("{}", message)
        }
    }

    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = &LOGGER;
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = &LOGGER;
    logger.log("some message".to_string());

    let logger2 = &LOGGER;
    logger2.log("other message".to_string());

    handle.join().unwrap();
}
