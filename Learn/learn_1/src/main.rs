use std::ffi::c_str;

struct Struct {
    e: i32,
}

const MAX_POINTS: u32 = 100_000;

fn main1() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("{}:{}", spaces, MAX_POINTS);

    let (x1, x2);
    x1 = 5;
    x2 = 6;
    assert_eq!([x1, x2], [5, 6]);
    println!("yes");
}

fn main_2() {
    let _guess: String = "42".parse().expect("Not a Number");
    let _guess2 = "42".parse::<u32>().expect("Not a Number");
}

fn main_3() {
    println!(
        "{},{},{},{},{}",
        23_198_232, 0xff, 0o23, 0b11111111, b'A' as char
    );

    // let p: u8 = 256; // Error
    let p = 244;
    println!("{}", p);

    let mut a: u8 = 255;
    a = a.wrapping_add(20);
    println!("{}", a);

    let c1: f32 = 2.01;
    let c2: f64 = 3.0;
    println!("{:10.5},{}", c1 as f32, c2);

    if (0.1_f64 + 0.2 - 0.3).abs() < 0.0001 {
        println!("0.1+0.2==0.3");
    } else {
        println!("not equal");
    }
}

fn main_4() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    if (-42.1 as f64).sqrt().is_nan() {
        println!("未定义");
    }

    let s: f64 = 2523423445.234_f64 * 232321331_f64;
    println!("s:{}", s);

    let res = 21.45 / 2.0;
    println!("{}", res);
}

fn main_5() {
    let mut a: u32 = 20;
    println!("{:08b}", a);
    a = a << 20;
    println!("{:08b}", a);
}

use num::complex::Complex;

fn main_6() {
    for i in [1, 2, 3, 4, 5] {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 1;
    x + y
}

fn main() {
    let res = add_with_extra(5, 6);
    println!("{}", res);
}
