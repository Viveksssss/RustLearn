use std::collections::HashMap;

pub fn test1() {
    let mut hash = HashMap::new();
    hash.reserve(200);
    hash.insert("红宝石", 1);
    hash.insert("蓝宝石", 2);
    hash.insert("河边捡的误以为是宝石的破石头", 18);

    println!("{:?}", hash);

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_app: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_app)
}

pub fn test2() {
    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);
    println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    let p = name;
    println!("{}", p);
    // println!("{}",name);
    // handsome_boys存的name引用.因为那么被转移到p,如果handsome_boys不再使用
    // name没问题.如果使用,name引用失效,报错.
    // println!("{:?}", handsome_boys);
    println!("还有，他的真实年龄远远不止{}岁", age);
}

pub fn test3() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // let score: Option<&i32> = scores.get(&team_name);
    let score_ans: i32 = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score_ans);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let new = scores.entry("Blue".to_string()).or_insert(4);
    println!("new : {new}");
}

pub fn test4() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split(' ') {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

pub fn test5() {
    use std::collections::HashMap;
    use std::hash::BuildHasherDefault;
    // 引入第三方的哈希函数
    use twox_hash::XxHash64;

    // 指定HashMap使用第三方的哈希函数XxHash64
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
    let mut p = hash.get_mut(&42);
    match p {
        Some(v) => *v = "213",
        None => println!(""),
    }
    println!("{:?}", hash);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub fn test6() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    //作为人类，我们可以很清晰的看出 result 实际上引用了 string1，因为 string1 的长度明显要比 string2 长，既然如此，编译器不该如此矫情才对，它应该能认识到 result 没有引用 string2，让我们这段代码通过。只能说，作为尊贵的人类，编译器的发明者，你高估了这个工具的能力，它真的做不到！而且 Rust 编译器在调教上是非常保守的：当可能出错也可能不出错时，它会选择前者，抛出编译错误。
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
    //     println!("Attention please: {}", announcement);
    //     self.part
    // }

    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 结构体被 part 所指向的外部数据“锁住”了。外部数据没了，结构体就必须先消失。所以是外部数据控制结构体的存亡，而不是反过来。

pub fn test7() {
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: std::fmt::Debug,
    {
        println!("announcement:{:?}", ann);
        if x.len() > y.len() { x } else { y }
    }

    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let s = ImportantExcerpt { part: "234234" };
    longest_with_an_announcement::<ImportantExcerpt>("232", "35235", s);
}
