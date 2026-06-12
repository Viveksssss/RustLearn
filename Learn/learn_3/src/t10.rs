use core::time;
use std::thread::{self, sleep, spawn};
use std::time::Duration;

pub fn test1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    })
    .join()
    .unwrap();

    for i in 1..10 {
        println!("s:{}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

use std::sync::{Arc, Barrier, Mutex};

pub fn test2() {
    let mut handlers = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    for _ in 0..6 {
        let b = barrier.clone();
        handlers.push(thread::spawn(move || {
            println!("before wait!");
            b.wait();
            println!("after wait!");
        }))
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}

use std::cell::RefCell;

thread_local!(static FOO : RefCell<u32> = RefCell::new(1));

pub fn test3() {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    t.join().unwrap();

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

pub fn test4() {
    use std::cell::Cell;
    use std::sync::Arc;
    use std::thread;
    use thread_local::ThreadLocal;

    let tls = Arc::new(ThreadLocal::new());
    let mut v = vec![];
    // 创建多个线程
    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            // 将计数器加1
            // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
            // 这只能在线程退出后发生，因此不会导致任何竞争条件
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        });
        v.push(handle);
    }
    for handle in v {
        handle.join().unwrap();
    }
    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
        // 因为一些线程已退出，并且其他线程会回收退出线程的对象
        println!("x: {}, y: {}", x, y.get());
        x + y.get()
    });

    // 和为5
    assert_eq!(total, 5);
}

pub fn test5() {
    use std::sync::{Condvar, Mutex};
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut start = lock.lock().unwrap();
        println!("changing started");
        *start = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");
}

use std::sync::mpsc;

pub fn test6() {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let s = String::from("康哥来咯");
        thread::sleep(std::time::Duration::from_secs(2));
        tx.send(s).unwrap();
        // println!("{}", s); // s被移动
    });

    println!("receive : {}", rx.recv().unwrap()); // 阻塞等待
}

pub fn test7() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("1s"),
            String::from("2s"),
            String::from("3s"),
            String::from("4s"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got : {}", received);
    }
}

pub fn test8() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned t2x")).unwrap();
    });

    for received in rx {
        println!("Got : {}", received);
    }
}

use std::sync::mpsc::{Receiver, Sender};

enum Fruit {
    Apple(u8),
    Orange(String),
}

pub fn test9() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }
}

pub fn test10() {
    use std::thread;

    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // 以上代码看起来非常正常，但是运行后主线程会一直阻塞，最后一行打印输出也不会被执行，原因在于： 子线程拿走的是复制后的send的所有权，这些拷贝会在子线程结束后被drop，因此无需担心，但是send本身却直到main函数的结束才会被drop。

    // 之前提到，通道关闭的两个条件：发送者全部drop或接收者被drop，要结束for循环显然是要求发送者全部drop，但是由于send自身没有被drop，会导致该循环永远无法结束，最终主线程会一直阻塞。

    // 在这里drop send...
    drop(send);
    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}

pub fn test11() {
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::RwLock;

    let mt = Arc::new(Mutex::<i32>::new(5));
    let mut handles = vec![];
    for i in 0..3 {
        let mtx = mt.clone();
        let handle = thread::spawn(move || {
            let mut num = mtx.lock().unwrap();
            println!("{:?}:{}", std::thread::current().id(), num);
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *mt.lock().unwrap()); // 10

    let lock = RwLock::new(5);
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
}

pub fn test12() {
    use std::sync::Condvar;

    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            while !*lock {
                // wait方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新。
                // 同时当前线程在此处会被阻塞，直到被其他地方notify后，它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程。
                lock = ccond.wait(lock).unwrap();
            }

            *lock = false;

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);
}

pub async fn test13() {
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = vec![];
    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            println!("{}", 1);
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
