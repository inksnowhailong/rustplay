pub fn run() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let u_8: u8 = 255; // 无符号
    let i_8: i8 = -128; // 有符号
    let f_32: f32 = 3.14; // 浮点数
    let f_64: f64 = 3.14; // 双精度浮点数
    let b: bool = true; // 布尔值
    let c: char = 'z'; // 字符

    let char = "abc"; // 字符串
    let tuple = (1, 2, 3); // 元组 可以包含不同类型的元素 固定长度
    let arr: [i32; 4] = [1, 2, 3,4]; // 数组 每个元素必须有相同类型，固定长度
    let arr2 = [3; 5]; // [3, 3, 3, 3, 3]   可以用第一个元素，填充第二个元素次数来初始化一个数字
}

// mod testBaseInfo {

// }
