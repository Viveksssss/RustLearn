#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}

pub fn test1() {
    // 报错:loan是返回值&Self,但是由于生命周期消除规则,所以&Self生命周期'a等于&mut self.于是在main作用域,可变借用依然存活
    // let mut foo = Foo;
    // let loan = foo.mutate_and_share();

    // foo.share();
    // println!("{:?}", loan);
}

pub fn test2() {
    use std::collections::HashMap;
    use std::hash::Hash;
    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        // 事实上在第一次map.get_mut之后可变借用就没了,但是编译器不理解,因此实际作用于一直持续到结束,导致下面使用insert/get_mut报错.
        // match map.get_mut(&key){
        //     Some(value)=>value,
        //     None=>{
        //         map.insert(key.clone(),V::default());
        //         map.get_mut(&key).unwrap()
        //     }
        // }

        // 1.
        map.entry(key).or_insert_with(|| V::default())

        // 2.
        // if map.contains_key(&key) {
        //     map.get_mut(&key).unwrap()
        // } else {
        //     map.insert(key.clone(), V::default());
        //     map.get_mut(&key).unwrap()
        // }
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 此处必须加上'a : 'b
// 原因是,返回值的生命周期是'b,而我们返回的self.part的生命周期是'a
// 因此必须要求'a活的比'b久
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn fn_elision(x: &i32) -> &i32 {
    x
}

fn clousure_slision<T, F: Fn(&T) -> &T>(f: F) -> F {
    f
}

pub fn test3() {
    let cs = clousure_slision(|x: &i32| -> &i32 { x });
    println!("{}", cs(&5));
}

pub fn test4() {
    // NLL
    // 旧版本会报错,因为r1/r2生命周期一直持续到{}结束.
    // 新版本生命周期持续到最后一次使用,因此不影响r3
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub fn test5() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    // reborrow! 此时对`r`的再借用不会导致跟上面的借用冲突
    let rr: &Point = &*r;

    // 再借用`rr`最后一次使用发生在这里，在它的生命周期中，我们并没有使用原来的借用`r`，因此不会报错
    println!("{:?}", rr);

    // 再借用结束后，才去使用原来的借用`r`
    r.move_to(10, 10);
    println!("{:?}", r);

    // 这里实际也发生了再借用
    use std::vec::Vec;
    fn read_length(strings: &mut Vec<String>) -> usize {
        strings.len()

        // 等价
        // let _temp = &*strings;
        // _temp.len()
        // 因为len的真正参数是(&self),但是目前的strings是&mut Vec<String>
        // 因此自动解引用再引用
    }
}

pub fn test6() {
    struct Interface<'a> {
        manager: &'a mut Manager<'a>,
    }

    impl<'a> Interface<'a> {
        pub fn noop(self) {
            println!("interface consumed");
        }
    }

    struct Manager<'a> {
        text: &'a str,
    }

    struct List<'a> {
        manager: Manager<'a>,
    }

    impl<'a> List<'a> {
        pub fn get_interface(&'a mut self) -> Interface {
            Interface {
                manager: &mut self.manager,
            }
        }
    }

    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了

    // use_list(&list);

    fn use_list(list: &List) {
        println!("{}", list.manager.text);
    }

    // 首先在直觉上，list.get_interface() 借用的可变引用，按理来说应该在这行代码结束后，就归还了，但是为什么还能持续到 use_list(&list) 后面呢？
    // 这是因为我们在 get_interface 方法中声明的 lifetime 有问题，该方法的参数的生命周期是 'a，而 List 的生命周期也是 'a，说明该方法至少活得跟 List 一样久，再回到 main 函数中，list 可以活到 main 函数的结束，因此 list.get_interface() 借用的可变引用也会活到 main 函数的结束，在此期间，自然无法再进行借用了。
}

pub fn test7() {
    struct Interface<'a: 'b, 'b> {
        manager: &'b mut Manager<'a>,
    }

    impl<'a: 'b, 'b> Interface<'a, 'b> {
        pub fn noop(self) {
            println!("interface consumed");
        }
    }

    struct Manager<'a> {
        text: &'a str,
    }

    struct List<'a> {
        manager: Manager<'a>,
    }

    impl<'a> List<'a> {
        pub fn get_interface<'b>(&'b mut self) -> Interface<'a, 'b>
        where
            'a: 'b,
        {
            Interface {
                manager: &mut self.manager,
            }
        }
    }

    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用可以通过，因为Interface的生命周期不需要跟list一样长
    use_list(&list);

    fn use_list(list: &List) {
        println!("{}", list.manager.text);
    }
}
