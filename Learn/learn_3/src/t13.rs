use core::slice;
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

pub fn test1() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("r1 is {}", unsafe { *r1 });
    println!("r2 is {}", unsafe { *r2 });

    fn get_memroy_location() -> (usize, usize) {
        let string = "Hello World";
        let pointer = string.as_ptr() as usize;
        let length = string.len();
        (pointer, length)
    }

    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    let (pointer, length) = get_memroy_location();
    let message = get_str_at_location(pointer, length);
    println!("pointer:{},len:{},message:{}", pointer, length, message);
}

pub fn test2() {
    let a = 1;
    let b: *const i32 = &a as *const i32;
    let c: *const i32 = &a;
    unsafe {
        println!("{}:{}", *b, *c);
    }

    let a1 = Box::new(10);
    let b1: *const i32 = &*a1;
    let c1 = Box::into_raw(a1);
    unsafe {
        println!("{},{}", *b1, *c1);
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn test3(){
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = split_at_mut(r, 3);
    println!("{:?},{:?}",a,b);
}
