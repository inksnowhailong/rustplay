use std::{sync::mpsc, thread, time::Duration};
use std::sync::{Arc,Mutex};
use std::rc::Rc;
fn main() {
    //    let haddle =  thread::spawn(|| {
    //         for i in 1..10 {
    //             println!("新线程中的打印: {}", i);
    //             thread::sleep(Duration::from_millis(1));
    //         }
    //     });
    //     haddle.join().unwrap();
    //     for i in 1..5 {
    //         println!("主线程的数: {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    //     println!("=========================");
    //     let v = vec![1, 2, 3];

    //     let handle = thread::spawn(move|| {
    //         println!("Here's a vector: {v:?}");
    //     });

    //     // drop(v);
    //     handle.join().unwrap();
    //     println!("=========================");

    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });
    // let res = rx.recv().unwrap();
    // println!("res: {}", res);
    //     println!("=========================");

    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from('1'),
    //         String::from('2'),
    //         String::from('3'),
    //         String::from('4'),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }

    // });
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("11"),
    //         String::from("21"),
    //         String::from("31"),
    //         String::from("41"),
    //     ];
    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1))
    //     }
    // });
    // for received in rx {
    //     println!("接受消息：{received}")
    // };
    //   println!("=========================");
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num =  6;
    // }
    // println!("m = {m:?}");
    println!("=========================");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
