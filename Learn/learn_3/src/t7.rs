enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

use std::convert::TryFrom;

// 需要为每个枚举成员都实现一个转换分支，非常麻烦。
impl TryFrom<i32> for Color {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Color::Red as i32 => Ok(Color::Red),
            x if x == Color::Green as i32 => Ok(Color::Green),
            x if x == Color::Blue as i32 => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

// 使用宏来简化，自动根据枚举的定义来实现TryFrom特征:先不用掌握,害怕...
#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
    enum MyEnum {
        A = 1,
        B,
        C,
    }
}

#[repr(i32)]
enum PRICE {
    BAD = 1,
    GOOD,
    NCIE,
}

pub fn test1() {
    let color = Color::Green;
    let value = color as i32;
    println!("{}", value);
    // 报错,不能相互转换
    // match value{
    //     Color::Read => {},
    //     Color::Green => {},
    //     Color::Blue => {}
    //     _ => {}
    // }

    // 结合TryInto转换
    match value.try_into() {
        Ok(Color::Red) => println!("red"),
        Ok(Color::Green) => println!("green"),
        Ok(Color::Blue) => println!("blue"),
        Err(_) => eprintln!("unknown number"),
    }
}

pub fn test2() {
    let x = PRICE::BAD;
    let y = PRICE::GOOD as i32;
    let z: PRICE = unsafe { std::mem::transmute(y) };
    match z {
        PRICE::BAD => {
            println!("BAD")
        }
        PRICE::GOOD => {
            println!("GOOD")
        }
        PRICE::NCIE => {
            println!("NICE")
        }
    }
}
