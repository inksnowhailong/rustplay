
use std::thread;
#[derive(Debug,PartialEq,Copy,Clone)]
enum ShirtColor {
    Red,
    Blue
}

// 库存 结构
struct Inventory {
    shirts:Vec<ShirtColor>
}
// 为这个结构实现
impl Inventory {
    // 获取赠品
    fn giveaway(&self, user_preference:Option<ShirtColor>)->ShirtColor{
        // 有用户偏好的，就返回用户偏好的，否则就返回数量较多的颜色
        user_preference.unwrap_or_else(||self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        }else{
            ShirtColor::Blue
        }
    }
}
fn main() {
 let store = Inventory {
    shirts:vec![
        ShirtColor::Blue,
        ShirtColor::Blue,
        ShirtColor::Red,
    ]
 };

 let user_pref1 = Some(ShirtColor::Red);
 let giveaway1=store.giveaway(user_pref1);
 println!("1号喜欢{:?} 拿到了{:?}",user_pref1,giveaway1);

 let user_pref2 = None;
 let giveaway2=store.giveaway(user_pref2);
 println!("2号喜欢{:?} 拿到了{:?}",user_pref2,giveaway2);

 println!("=============");
 let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    println!("after defining closure: {list:?}");
}

