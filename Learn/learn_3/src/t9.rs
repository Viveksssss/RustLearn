use crate::t9::ListNode::{Cons, Nil};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum ListNode {
    Cons(i32, RefCell<Weak<ListNode>>),
    Nil,
}

impl ListNode {
    fn tail(&self) -> Option<&RefCell<Weak<ListNode>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

impl Drop for ListNode {
    fn drop(&mut self) {
        match self {
            ListNode::Cons(val, _) => println!("   💀 销毁节点(值={})", val),
            ListNode::Nil => println!("   💀 销毁 Nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct List {
    head: Rc<ListNode>,
    tail: Rc<ListNode>,
    size: u32,
}

impl List {
    pub fn new() -> List {
        let nil = Rc::new(ListNode::Nil);
        List {
            head: Rc::clone(&nil),
            tail: nil,
            size: 0,
        }
    }

    pub fn preappend(&mut self, value: i32) -> &mut Self {
        let new_head = Rc::new(ListNode::Cons(value, RefCell::new(Weak::new())));
        self.head = Rc::clone(&new_head);
        if self.is_empty() {
            self.tail = Rc::clone(&new_head);
        } else {
            if let ListNode::Cons(_, next_ref) = &*new_head {
                *next_ref.borrow_mut() = Rc::downgrade(&self.head);
            }
        }
        self.size += 1;
        self
    }

    pub fn append(&mut self, value: i32) -> &mut Self {
        let new_node = Rc::new(ListNode::Cons(value, RefCell::new(Weak::new())));
        if self.is_empty() {
            self.head = Rc::clone(&new_node);
            self.tail = Rc::clone(&new_node); // 都使用 clone
        } else {
            // 这里有大bug:最后的节点只有 tail 一个指针指向，当插入时，最后的节点被弱引用，然后指针切换指向导致引用计数 -1 被删除
            if let ListNode::Cons(_, next_ref) = &*self.tail {
                *next_ref.borrow_mut() = Rc::downgrade(&new_node);
            }
            self.tail = new_node;
        }

        self.size += 1;
        self
    }

    pub fn head(&self) -> Option<i32> {
        match &*self.head {
            ListNode::Cons(val, _) => Some(*val),
            ListNode::Nil => None,
        }
    }

    pub fn tail(&self) -> Option<i32> {
        match &*self.tail {
            ListNode::Cons(val, _) => Some(*val),
            ListNode::Nil => None,
        }
    }

    // 检查是否为空
    pub fn is_empty(&self) -> bool {
        matches!(&*self.head, ListNode::Nil)
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // 清理链表的逻辑
        println!("List 被销毁，释放了 {} 个节点", self.size);

        // 由于使用了 Rc 和 Weak，大部分清理会自动进行
        // 但如果有循环引用，需要手动打破
    }
}

pub fn test1() {
    // Rust 的安全性是众所周知的，但是不代表它不会内存泄漏。一个典型的例子就是同时使用 Rc<T> 和 RefCell<T> 创建循环引用，最终这些引用的计数都无法被归零，因此 Rc<T> 拥有的值也不会被释放清理。

    let mut list: List = List::new();
    println!("empty ? : {}", list.is_empty());
    println!("size : {}", list.size());
    list.append(32)
        .append(6546)
        .append(34)
        .append(89)
        .append(100);
    list.preappend(55);
    println!("empty ? : {}", list.is_empty());
    println!("head : {}", list.head().unwrap());
    println!("tail : {}", list.tail().unwrap());
    println!("size : {}", list.size());

    let node1 = Rc::new(ListNode::Cons(1, RefCell::new(Weak::new())));
    let node2 = Rc::new(ListNode::Cons(2, RefCell::new(Weak::new())));
    println!("初始引用计数:");
    println!(
        "  node1 强引用: {}, 弱引用: {}",
        Rc::strong_count(&node1),
        Rc::weak_count(&node1)
    );
    println!(
        "  node2 强引用: {}, 弱引用: {}",
        Rc::strong_count(&node2),
        Rc::weak_count(&node2)
    );
    if let ListNode::Cons(_, next_ref) = &*node1 {
        *next_ref.borrow_mut() = Rc::downgrade(&node2);
    }

    if let ListNode::Cons(_, next_ref) = &*node2 {
        *next_ref.borrow_mut() = Rc::downgrade(&node1);
    }
    println!("\n建立弱引用循环后:");
    println!(
        "  node1 强引用: {}, 弱引用: {}",
        Rc::strong_count(&node1),
        Rc::weak_count(&node1)
    );
    println!(
        "  node2 强引用: {}, 弱引用: {}",
        Rc::strong_count(&node2),
        Rc::weak_count(&node2)
    );

    // 尝试升级弱引用
    if let ListNode::Cons(_, next_ref) = &*node1 {
        if let Some(upgraded) = next_ref.borrow().upgrade() {
            if let ListNode::Cons(val, _) = &*upgraded {
                println!("\n从 node1 可以访问到 node2: 值={}", val);
            }
        }
    }

    println!("\n准备释放 node1 和 node2...");

    println!("\n注意：使用 Weak 没有循环引用，节点会被正确释放！");
}
