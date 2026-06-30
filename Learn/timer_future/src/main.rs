use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{ArcWake, waker_ref},
    },
    std::{
        future::Future,
        sync::mpsc::{Receiver, SyncSender, sync_channel},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },
    // 引入之前实现的定时器模块
    timer_future::TimerFuture,
};

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUE_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUE_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("队列已满");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("队列已满");
    }
}
impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                if future.as_mut().poll(context).is_pending() {
                    // Future还没执行完，因此将它放回任务中，等待下次被poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}

///
///时间轴：
// ═══════════════════════════════════════════════════════════════════════════
//
// T0: main 启动
//     │
//     ├─ 创建 Executor 和 Spawner
//     │
//     ▼
// T1: 生成任务
//     │
//     ├─ spawner.spawn(async { ... })
//     ├─ async 块编译为 Future
//     ├─ 装箱为 BoxFuture
//     ├─ 创建 Task，包含 future
//     └─ 发送到队列
//     │
//     ▼
// T2: drop(spawner)
//     │
//     └─ 关闭发送端
//     │
//     ▼
// T3: executor.run() 开始
//     │
//     ├─ 从队列取出任务
//     ├─ 取出 future
//     ├─ 创建 Waker (指向 Task)
//     ├─ 第一次 poll
//     │   │
//     │   ├─ 执行 async 块
//     │   ├─ 打印 "howdy!"
//     │   ├─ 创建 TimerFuture
//     │   │   └─ 启动定时器线程
//     │   ├─ TimerFuture.poll()
//     │   │   ├─ completed = false
//     │   │   ├─ 保存 Waker
//     │   │   └─ 返回 Pending
//     │   └─ async 块返回 Pending
//     │
//     ├─ 因为是 Pending，future 放回任务
//     └─ 继续循环，队列为空，阻塞等待
//     │
//     ▼
// T4: 2秒后
//     │
//     ├─ 定时器线程醒来
//     ├─ completed = true
//     ├─ 取出保存的 Waker
//     ├─ waker.wake()
//     │   ├─ Task::wake_by_ref()
//     │   └─ 任务重新发送到队列
//     └─ 定时器线程结束
//     │
//     ▼
// T5: Executor 被唤醒
//     │
//     ├─ 从队列取出任务
//     ├─ 取出 future
//     ├─ 第二次 poll
//     │   │
//     │   ├─ 继续执行 async 块
//     │   ├─ TimerFuture.poll()
//     │   │   ├─ completed = true
//     │   │   └─ 返回 Ready(())
//     │   ├─ 继续执行 async 块
//     │   └─ 打印 "done!"
//     │
//     ├─ 因为是 Ready，future 被丢弃
//     └─ 继续循环，队列为空
//     │
//     ▼
// T6: 所有 Sender 已关闭
//     │
//     ├─ recv() 返回 Err
//     └─ 循环退出
//     │
//     ▼
// T7: 程序结束
//     │
//     ├─ executor.run() 返回
//     └─ main 退出，所有资源释放
///

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    // async 块 = 外层 Future
    //     ↓ poll
    // 执行 async 块内部的代码
    //     ↓ 遇到 .await
    // 调用内层 Future (TimerFuture) 的 poll
    //     ↓ 返回 Pending
    // 外层 Future 也返回 Pending
    //     ↓ 等待...
    // 再次 poll 外层 Future
    //     ↓ 再次调用内层 Future 的 poll
    // 内层 Future 返回 Ready
    //     ↓ 外层 Future 继续执行
    // 打印 "done!"
    //     ↓ 返回 Ready
    spawner.spawn(async move {
        println!("howdy!");
        // 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);

    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}
