use std::fmt;

pub fn run() {
    println!("{0}是{1}，{1}不是{0}", "a", "b");

    println!("{key1},{key2}", key2 = 5, key1 = 3);

    println!("{:b}这是啥", 2);

    println!("{number:>width$}", number = 1, width = 6);

    println!("{number:4<width$}", number = 2, width = 10);

    #[allow(dead_code)]
    struct What(String);
    impl fmt::Display for What {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    println!("{}", What(String::from("4")));

    #[derive(Debug)]
    struct CodeFmt(i32);

    println!("这又是个啥{:?}", CodeFmt(3));
}
