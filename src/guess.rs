/*
:: 路径运算符，类似于其他编程语言中的.运算符
 用于命名空间的访问，如模块、类型、常量、枚举变体、函数、trait 等。
. 运算符
用于实例的成员访问，如方法调用、属性访问。

对于rust来说，::是用于静态访问，从大的命名空间或者模块中获取内容
而.是用于实例访问，从具体的对象中获取内容

*/
use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub fn guess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    /*
      let  是用于声明变量，但是默认变量不可变
      mut  用于声明可变性。比如let mut  让变量可变，  &mut 创建可变引用（）
    */
    let mut guess = String::new();
    /*
    一个无限循环，没有break，会一直循环
    而break 后面的值会返回出去
    */
    loop {
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        /*
         match 模式匹配 类似于swtich case
         它不止可以匹配精准值，还能范围匹配，模式结构，返回值
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
