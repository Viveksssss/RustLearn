pub fn test() {
    // 匹配字面量
    println!("1.匹配字面量");
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("else"),
    };

    // 匹配命名变量
    println!("2.匹配命名变量");
    let x = Some(10);
    let y = 10;
    match x {
        Some(5) => println!("Got {}", 50),
        Some(y) => println!("Matched y = {:?}", y),
        _ => println!("Default case x = {:?}", x),
    }

    println!("at the end,x = {:?},y = {:?}", x, y);

    // 单分值多模式
    println!("3.单分支多模式");
    let x = 1;
    match x {
        1 | 2 => println!("onw or two"),
        3 => println!("three"),
        _ => println!("anything"),
    };

    // 序列匹配
    println!("4.序列匹配");
    let x = 5;
    match x {
        1..=5 => {
            println!("one through five");
        }
        _ => {
            println!("something else");
        }
    }
    // 结构并分解值
    struct Point {
        x: i32,
        y: i32,
    };

    let p = Point { x: 5, y: 5 };
    let Point { x: wa, y: wb } = p;
    println!("(wa,wb) = ({},{})", wa, wb);

    match p {
        Point { x, y: 0 } => println!("_,0"),
        Point { x: 0, y } => println!("0,_"),
        Point { x, y } => println!("{},{}", x, y),
    }

    // 结构枚举
    println!("5.结构枚举");
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
        /* 匿名字段使用(),命名字段使用{} */
    };
    let msg = Message::ChangeColor((0), (160), (255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructive");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y,);
        }
        Message::Write(w) => {
            println!("The text is {}", w);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {},green {},and blue {}", r, g, b,);
        }
    };

    // 结构嵌套的结构体和枚举
    println!("6.结构嵌套的结构体和枚举");
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    };

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
        /* 匿名字段使用(),命名字段使用{} */
    };

    let msg = Message2::ChangeColor(Color::Hsv((0), (160), (180)));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {},green {},blue {}", r, g, b);
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to h {},s {},v {}", h, s, v);
        }
        _ => {
            println!("");
        }
    };

    // 解构结构体和元组
    println!("7.解构结构体和元组");
    struct Point2 {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point2 { x, y }) = ((3, 10), Point2 { x: 3, y: -10 });
    println!("{},{}:({},{})", feet, inches, x, y);

    // 结构数组
    println!("8.结构数组");
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;
    assert_eq!(x, 114);
    assert_eq!(y, 514);

    let arr: &[u16] = &[114, 514];

    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }

    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }

    let arr: &[u16] = &[];

    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [x, ..]));

    // 用 .. 忽略剩余值
    println!("9用 .. 忽略剩余值");
    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    // 匹配守卫提供的额外条件
    println!("10.匹配守卫提供的额外条件");
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // @绑定
    println!("11.@绑定");
    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello { id: id_v @ 3..7 } => {
            println!("Found an id in range {}", id_v);
        }
        Message3::Hello { id: id_v @ 10..=12 } => {
            println!("in another range:{}", id_v);
        }
        Message3::Hello { id } => {
            println!("{}", id);
        }
    }
    // @前绑定后解构(Rust 1.56 新增)
    println!("@前绑定后解构(Rust 1.56 新增)");
    #[derive(Debug)]
    struct Point4 {
        x: i32,
        y: i32,
    }
    let p @ Point4 { x: px, y: py } = Point4 { x: 10, y: 20 };
    println!("{}:{}", p.x, p.y);
    println!("{}:{}", px, py);
    println!("p:{:?}", p);

    let point = Point4 { x: 10, y: 5 };
    if let p @ Point4 { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }

    // @新特性(Rust 1.53 新增)
    println!("12.@新特性(Rust 1.53 新增)");
    match 1 {
        num @ (1 | 2) => {
            println!("matched {}", num);
        }
        _ => (),
    }

    // match的自动解引用
    println!("13.match的自动解引用");
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        // err : &mut value => value.push_str(" world!"),
        value => {
            value.push_str(" world!");
            println!("{}", value);
            println!("{}", value);
        }
    }
}
