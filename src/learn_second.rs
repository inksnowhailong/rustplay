
pub fn ownership(){
    let mut s = String::from("hello");

    change(&mut s);
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    // println!("{}, {}", r1, r2);
}
fn change(some_string:&mut String) {
    some_string.push_str(", world");
}
