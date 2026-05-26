pub fn test() {
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);

    // let v2 = vec![1, 2, 3, 4, 5];
    // for i in &v2 {
    //     println!("{}", i);
    // }

    // let third: &i32 = &v2[3];
    // println!("third:{}", third);

    // match v2.get(3) {
    //     Some(four) => println!("four:{}", four),
    //     None => println!("None!"),
    // }

    // let mut vv = vec![1, 2, 3, 4, 5];
    // vv.push(6);
    // let first = &vv[0];
    // println!("The firset element is : {first}");

    // let mut v = vec![1, 2, 3];
    // for i in &mut v {
    //     *i += 10
    // }
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    fn show_addr(ip: IpAddr) {
        println!("{:?}", ip);
    }

    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in v {
        show_addr(ip)
    }
}

pub fn test2() {
    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0)
        }
    }
    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }

    let v: Vec<Box<dyn IpAddr>> = vec![
        // Box类似cpp的智能指针指向基类
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in &v {
        ip.display();
    }

    /*  等价cpp
       class IpAddr {
       public:
           virtual void display() = 0;
           virtual ~IpAddr() = default;
       };

       class V4 : public IpAddr {
           string addr;
       public:
           V4(string s) : addr(s) {}
           void display() override { cout << "ipv4: " << addr << endl; }
       };

       class V6 : public IpAddr {
           string addr;
       public:
           V6(string s) : addr(s) {}
           void display() override { cout << "ipv6: " << addr << endl; }
       };

       // 存储不同类型的IP地址
       vector<unique_ptr<IpAddr>> v;
       v.push_back(make_unique<V4>("127.0.0.1"));
       v.push_back(make_unique<V6>("::1"));

       for (auto& ip : v) {
           ip->display();  // 虚函数调用
       }
    */
}

pub fn test3() {
    let mut v: Vec<i32> = Vec::with_capacity(100);
    v.extend([1, 2, 3]); // 附加数据到 v
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.reserve(100); // 调整 v 的容量，至少要有 100 的容量
    println!(
        "Vector（reserve） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    v.shrink_to_fit(); // 释放剩余的容量，一般情况下，不会主动去释放容量
    println!(
        "Vector（shrink_to_fit） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );
}

pub fn test4() {
    let mut vec = vec![1, 5, 8, 4, 3, 3, 567, 100];
    vec.sort_unstable();
    for i in &vec {
        println!("{i}");
    }

    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    for i in &vec {
        println!("{i}");
    }
}

pub fn test5() {
    #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
    struct Person {
        age: u32,
        name: String,
    }

    impl Person {
        fn new(name: String, age: u32) -> Person {
            Person { age, name }
        }
    }

    let mut people = vec![
        Person::new("zoe".to_string(), 23),
        Person::new("Al".to_string(), 25),
        Person::new("John".to_string(), 1),
    ];

    // people.sort_unstable_by(|a, b| a.age.cmp(&b.age));
    people.sort_unstable();
    println!("{:?}", people);
}
