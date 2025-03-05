#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}
impl Rect {
    fn area(&self)->u32{
        self.w * self.h
    }
}
pub fn run() {
    let w1 = 30;
    let h1 = 40;

    println!("计算矩形面试结果为：{}", area(w1, h1));

    let rect1 = (30, 40);

    println!("计算矩形面试结果为：{}", area2(rect1));

    let rect3: Rect = Rect { w: 30, h: 40 };
    println!("rect1 is {rect3:?}");
    // dbg!(&rect3); // dbg 会带走所有权 然后返回出来  println则一开始就是引用不会影响所有权
    println!("计算矩形面试结果为：{}", areaa3(dbg!(rect3)));
    let rect4 = Rect { w: 30, h: 50 };
    println!("方法的第一个参数总是self：{}", rect4.area());

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(data: (u32, u32)) -> u32 {
    data.0 * data.1
}

fn areaa3(data:Rect)->u32{
    data.w * data.h
}
