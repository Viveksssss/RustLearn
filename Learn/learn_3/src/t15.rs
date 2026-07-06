use std::{
    io,
    pin::{Pin, pin},
    time::Duration,
};

use futures::{SinkExt, Stream, StreamExt, channel::mpsc, executor::block_on};

async fn do_something() {
    do_before().await;
    println!("go go");
}

async fn do_before() {
    println!("do_before");
}

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "大胖熊".to_string(),
        name: String::from("hhh"),
    }
}

async fn sing_song(song: Song) {
    println!("为大家献上一首由{}创作的歌曲{}", song.author, song.name);
}

async fn dance() {
    println!("唱歌情深处,跳起舞来");
}

async fn learn_and_song() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_song();
    let f2 = dance();
    futures::join!(f1, f2);
}

pub fn test1() {
    let futures = async_main();
    block_on(futures);
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

pub fn test2() {
    use std::collections::VecDeque;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    use std::thread;
    use std::time::{Duration, Instant};

    // ===== 1. 任务结构 =====

    struct Task {
        future: Pin<Box<dyn Future<Output = ()> + Send>>,
        waker: Option<Waker>,
    }

    impl Task {
        fn new<F>(future: F) -> Self
        where
            F: Future<Output = ()> + Send + 'static,
        {
            Task {
                future: Box::pin(future),
                waker: None,
            }
        }
    }

    // ===== 2. 单线程执行器 =====

    struct SimpleExecutor {
        tasks: VecDeque<Arc<Mutex<Task>>>,
        running: bool,
    }

    impl SimpleExecutor {
        fn new() -> Self {
            SimpleExecutor {
                tasks: VecDeque::new(),
                running: true,
            }
        }

        // 添加任务
        fn spawn<F>(&mut self, future: F)
        where
            F: Future<Output = ()> + Send + 'static,
        {
            let task = Arc::new(Mutex::new(Task::new(future)));
            self.tasks.push_back(task);
        }

        // 运行执行器
        fn run(&mut self) {
            println!("🚀 执行器启动，管理 {} 个任务", self.tasks.len());

            while self.running && !self.tasks.is_empty() {
                // 取出一个任务
                if let Some(task) = self.tasks.pop_front() {
                    let mut task_guard = task.lock().unwrap();

                    // 创建 Waker - 把任务放回队列
                    let task_clone = task.clone();
                    let waker = create_waker(move || {
                        // 当任务被唤醒时，重新加入队列
                        println!("💡 任务被唤醒，重新加入队列");
                        // 注意：这里无法直接访问 executor 的 tasks
                        // 实际实现中，需要把任务放回队列
                        // 这里简化演示，不实现了
                    });

                    let mut cx = Context::from_waker(&waker);

                    // 轮询任务
                    match task_guard.future.as_mut().poll(&mut cx) {
                        Poll::Ready(()) => {
                            println!("✅ 任务完成");
                            // 任务完成，不重新加入队列
                        }
                        Poll::Pending => {
                            println!("⏳ 任务挂起，等待唤醒");
                            task_guard.waker = Some(waker);
                        }
                    }
                }

                // 模拟处理其他事件
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    // ===== 3. 修复：正确的 Waker 创建 =====

    // ✅ 定义包装结构体
    struct WakerData {
        callback: Box<dyn Fn() + Send + Sync>,
    }

    fn create_waker<F>(wake_fn: F) -> Waker
    where
        F: Fn() + Send + Sync + 'static,
    {
        // 创建包装结构体
        let data = WakerData {
            callback: Box::new(wake_fn),
        };
        let arc = Arc::new(data);
        let ptr = Arc::into_raw(arc) as *const ();

        unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
            // ✅ 使用 WakerData 类型
            let arc = unsafe { Arc::from_raw(ptr as *const WakerData) };
            let cloned = arc.clone();
            std::mem::forget(arc);
            let new_ptr = Arc::into_raw(cloned) as *const ();
            RawWaker::new(new_ptr, &VTABLE)
        }

        unsafe fn wake_waker(ptr: *const ()) {
            // ✅ 使用 WakerData 类型
            let arc = unsafe { Arc::from_raw(ptr as *const WakerData) };
            (arc.callback)(); // 调用闭包
            // arc 在这里被 drop，减少引用计数
        }

        unsafe fn wake_by_ref_waker(ptr: *const ()) {
            // ✅ 使用 WakerData 类型
            let arc = unsafe { Arc::from_raw(ptr as *const WakerData) };
            (arc.callback)();
            std::mem::forget(arc); // 保持引用计数
        }

        unsafe fn drop_waker(ptr: *const ()) {
            // ✅ 使用 WakerData 类型
            drop(unsafe { Arc::from_raw(ptr as *const WakerData) });
        }

        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(clone_waker, wake_waker, wake_by_ref_waker, drop_waker);

        let raw_waker = RawWaker::new(ptr, &VTABLE);
        unsafe { Waker::from_raw(raw_waker) }
    }
    // ===== 4. 测试：创建 1000 个定时器任务 =====

    struct SimpleTimer {
        start: Instant,
        duration: Duration,
        waker: Option<Waker>,
    }

    impl SimpleTimer {
        fn new(duration: Duration) -> Self {
            SimpleTimer {
                start: Instant::now(),
                duration,
                waker: None,
            }
        }
    }

    impl Future for SimpleTimer {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.start.elapsed() >= self.duration {
                Poll::Ready(())
            } else {
                let waker = Some(cx.waker().clone());
                // 注意：这里的定时器永远不会被唤醒
                // 因为实际实现中，需要一个线程在定时器到达时调用 waker.wake()
                // 为了演示，我们会在执行器中模拟唤醒
                Poll::Pending
            }
        }
    }

    let mut executor = SimpleExecutor::new();

    // 创建 1000 个任务
    for i in 0..1000 {
        let timer = SimpleTimer::new(Duration::from_millis(1));
        executor.spawn(async move {
            timer.await;
            println!("任务 {} 完成！", i);
        });
    }

    // 运行执行器
    executor.run();
}

pub fn test3() {
    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            let self_ref: *const String = &self.a;
            self.b = self_ref;
        }

        fn a(&self) -> &str {
            &self.a
        }

        fn b(&self) -> &String {
            // String::from(&self.a)
            unsafe { &*self.b }
        }
    }

    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b());
    std::mem::swap(&mut test1, &mut test2);
    println!("a: {}, b: {}", test2.a(), test2.b());
}

pub fn test4() {
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            }
        }

        fn init(self: Pin<&mut Self>) {
            let self_ref: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ref;
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            unsafe { &*(self.b) }
        }
    }

    let mut test1 = Test::new("test1");
    // 新的`test1`由于使用了`Pin`，因此无法再被移动，这里的声明会将之前的`test1`遮蔽掉(shadow)
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    let mut test3 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    // Test::init(test2.as_mut());
    test2.as_mut().init();

    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );
    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
    std::mem::swap(&mut test1, &mut test2);
    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );
    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
}

pub fn test5() {
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Pin<Box<Self>> {
            let t = Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            };
            let mut boxed = Box::pin(t);
            let self_ptr: *const String = &boxed.as_ref().a;
            unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
            boxed
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            unsafe { &*(self.b) }
        }
    }
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");

    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );
    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
    std::mem::swap(&mut test1, &mut test2);
    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );
    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
}

//  报错,因为x的生命周期持续到bad末尾,但是我们却将x引用传入到borrow_x,future活的更久.
//  关键在于 async 块并不是直接返回 borrow_x 的 Future，而是返回一个“包含 x 数据的新的 Future”。
//  1. 捕获并存储：当 Rust 编译器遇到 async { ... } 块时，它会将这个块编译成一个匿名的结构体（也就是生成的 Future 实现）。这个结构体里会包含块中使用的所有局部变量（比如 x）作为它的成员字段。
//  2. 延后执行：good 函数返回的是这个“匿名结构体 Future”。此时，x（值为 5）已经被移动并存储在这个结构体内部了。x 的生命周期不再受限于 good 函数的栈帧，而是受限于这个返回的 Future 对象本身。

// use std::future::Future;
// fn bad() -> impl Future<Output = u8> {
//     let x = 5;
//     borrow_x(&x) // ERROR: `x` does not live long enough
// }

// async fn borrow_x(x: &u8) -> u8 { *x }

use std::future::Future;

async fn borrow_x(x: &u8) -> u8 {
    *x
}

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
    // 因此还需要使用`.await`来获取具体的值
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt;
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt;
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}

async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;
    // stream
    //     .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
    //         // jump_n_times(num).await?;
    //         // report_n_jumps(num).await?;
    //         println!("{num}");
    //         Ok(())
    //     })
    //     .await?;

    let fnn = |num| async move {
        println!("{:?}", num);
    };
    stream
        .for_each_concurrent(MAX_CONCURRENT_JUMPERS, fnn)
        .await;

    Ok(())
}
use futures::stream::{self};
fn generator() -> impl Stream<Item = usize> {
    stream::unfold(0, |state| async move { Some((state, state + 1)) })
}

pub async fn test6() {
    let mut counter = generator();
    let mut counter2 = generator();
    let mut counter = pin!(counter.take(100));
    while let Some(item) = counter.next().await {
        println!("{}", item);
    }

    counter2
        .take(5)
        .for_each(|num| async move {
            println!("{}", num);
        })
        .await;
}

// WRONG:同时听歌和看书maybe:
// 实际上智能先读书后听音乐
// async fn enjoy_book_and_music() -> (Book, Music) {
//     let book = enjoy_book().await;
//     let music = enjoy_music().await;
//     (book, music)
// }
//

// WRONG:maybe:
// still wrong.在某些语言中也许可以，但是 Rust 不行。因为在某些语言中，Future一旦创建就开始运行，等到返回的时候，基本就可以同时结束并返回了。 但是 Rust 中的 Future 是惰性的，直到调用 .await 时，才会开始运行。而那两个 await 由于在代码中有先后顺序，因此它们是顺序运行的。
// async fn enjoy_book_and_music() -> (Book, Music) {
//     let book_future = enjoy_book();
//     let music_future = enjoy_music();
//     (book_future.await, music_future.await)
// }

// True:
use futures::join;

async fn enjoy_book() -> u32 {
    25
}

async fn enjoy_music() -> String {
    String::from("asdasdsa")
}

async fn enjoy_book_with_error() -> Result<u32, String> {
    Ok(25)
}

async fn enjoy_music_with_error() -> Result<String, String> {
    Ok(String::from("asdasdsa"))
}

async fn enjoy_book_and_music() -> (u32, String) {
    let book_fut = enjoy_book();
    let music_fut = enjoy_music();
    join!(book_fut, music_fut)
}

async fn enjoy_book_and_music_with_error() -> Result<(u32, String), String> {
    let book_fut = enjoy_book_with_error();
    let music_fut = enjoy_music_with_error();
    futures::try_join!(book_fut, music_fut)
}

pub async fn test7() {
    let (age, name) = enjoy_book_and_music().await;
    println!("{age},{name}");
    if let Ok((a, b)) = enjoy_book_and_music_with_error().await {
        println!("{a},{b}");
    } else {
        println!("Error");
    }
}

pub async fn test8() {
    use futures::{
        future::FutureExt, // for `.fuse()`
        pin_mut,
        select,
    };

    async fn task_one() {}
    async fn task_two() {}

    async fn reac_tasks() {
        let t2 = task_two().fuse();
        let t1 = task_one().fuse();

        let mut t1 = pin!(t1); // 重新绑定，覆盖原来的 t1
        let mut t2 = pin!(t2); // 重新绑定，覆盖原来的 t2

        loop {
            select! {
                () = t1 => {println!("task1")},
                () = t2 => {println!("task2")},
                complete => break,
                default => panic!(),
            }
        }
    }
    reac_tasks().await;
}

use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::FusedStream,
};

async fn get_new_num() -> u8 {
    // 这里可以是网络请求、数据库查询等
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    42 // 示例返回值
}

async fn run_on_new_num(num: u8) {
    /* ... */
    println!("Processing number: {}", num);
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Finished processing number: {}", num);
}

pub async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // 定时器已结束，若`get_new_num_fut`没有在运行，就创建一个新的
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // 收到新的数字 -- 创建一个新的`run_on_new_num_fut`并丢弃掉旧的
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            // 运行 `run_on_new_num_fut`
            () = run_on_new_num_fut => {},
            // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
            //后，执行到 `complete` 分支
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}
