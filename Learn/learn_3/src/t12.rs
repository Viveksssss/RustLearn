use std::num;

pub fn test1() {
    let mut o1: Result<&str, &str> = Ok("ok1");
    println!("{:?}", o1);
    o1 = Err("dasd");

    let b = o1.or_else(|err| {
        println!("or_else");
        Err("SAd")
    });
    println!("{:?}", b);

    let s1 = Some(3);
    s1.filter(|x: &i32| {
        println!("{}", x);
        x % 2 == 0
    });

    let s2 = Some("asdas");
    let o2: Result<&str, &str> = Ok("yes");
    println!("{:?}", s2.map(|s: &str| { s.chars().count() }));
    println!("{:?}", o2.map(|s: &str| { s.chars().count() }));

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

    assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2

    const V_DEFAULT: u32 = 1;

    let s: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);

    let b = Some(5).map_or(25, |i: i32| 25);
    let c = Some(5).map_or_else(|| 25, |i: i32| 254);

    let bb = Some("abc").ok_or("23");
    let cc = Some("asdasd").ok_or_else(|| "err");
}

pub fn test2() {
    use std::fmt::{Debug, Display, Formatter};
    #[derive(Debug)]
    struct AppError;

    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "An Error occurred,please try again!")
        }
    }

    fn produce_error() -> Result<(), AppError> {
        Err(AppError)
    }

    match produce_error() {
        Err(e) => eprintln!("{}", e),

        _ => {
            println!("No Error!")
        }
    }
}

pub fn test3() {
    use std::fmt;
    struct AppError {
        code: usize,
        message: String,
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let err_msg = match self.code {
                404 => "Sorry Can not fnd the page",
                _ => "Sorry something is wrong ! Please try again!",
            };
            write!(f, "{}", err_msg)
        }
    }

    impl fmt::Debug for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "AppError {{ code: {}, message: {} }}",
                self.code, self.message
            )
        }
    }

    fn produce_error() -> Result<(), AppError> {
        Err(AppError {
            code: 404,
            message: String::from("Page not found"),
        })
    }

    match produce_error() {
        Err(e) => eprintln!("{}", e), // 抱歉，未找到指定的页面!
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error()); // Err(AppError { code: 404, message: Page not found })

    eprintln!("{:#?}", produce_error());
}

pub fn test4() {
    use std::fs::File;
    use std::io;

    #[derive(Debug)]
    struct AppError {
        kind: String,    // 错误类型
        message: String, // 错误信息
    }

    impl From<io::Error> for AppError {
        fn from(error: io::Error) -> Self {
            AppError {
                kind: String::from("io"),
                message: error.to_string(),
            }
        }
    }

    impl From<num::ParseIntError> for AppError {
        fn from(error: num::ParseIntError) -> Self {
            AppError {
                kind: String::from("parse"),
                message: error.to_string(),
            }
        }
    }

    fn func() -> Result<(), AppError> {
        let _file = File::open("asdasdasdsadsa")?;
        Ok(())
    }
    func();
}

pub fn test5() {
    use std::error::Error;
    use std::fs::read_to_string;
    fn func() -> Result<(), Box<dyn Error>> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> Result<String, Box<dyn Error>> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string(file)?;
        Ok(source)
    }

    func();
}

// 使用thiserror/anyhow简化自定义错误
pub fn test6() {
    use std::fs::read_to_string;

    fn main() -> Result<(), MyError> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> Result<String, MyError> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string(file)?;
        Ok(source)
    }

    #[derive(thiserror::Error, Debug)]
    enum MyError {
        #[error("Environment variable not found")]
        EnvironmentVariableNotFound(#[from] std::env::VarError),
        #[error(transparent)]
        IOError(#[from] std::io::Error),
    }

    // --------------------------------------------

    use anyhow::Result;

    fn render2() -> Result<String> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string(file)?;
        Ok(source)
    }

    fn main2() -> Result<()> {
        let html = render2()?;
        println!("{}", html);
        Ok(())
    }
}
