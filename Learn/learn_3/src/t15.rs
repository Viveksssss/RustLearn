use std::time::Duration;

use futures::executor::block_on;

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

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.start.elapsed() >= self.duration {
                Poll::Ready(())
            } else {
                let waker = Some(cx.waker().clone());
                // 注意：这里的定时器永远不会被唤醒
                // 因为实际实现中，需要一个线程在定时器到达时调用 waker.wake()
                thread::spawn(move || {
                    thread::sleep(self.duration);
                    if let Some(waker) = waker {
                        waker.wake();
                    }
                });
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
