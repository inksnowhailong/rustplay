use std::env;
use std::process;
use minigrep::Config;
// 初代
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let config = Config::build(&args).unwrap_or_else(|err| {
//         eprintln!("出现了错误：{err}");
//         process::exit(1);
//     }) ;

//     println!("搜索词: {}", config.query);
//     println!("文件路径: {}", config.file_path);

//     if let Err(e) =  minigrep::run(config) {
//         eprintln!("这是什么错误：{e}")
//     }


// }
fn main() {
    let config = Config::build( env::args()).unwrap_or_else(|err| {
        eprintln!("出现了错误：{err}");
        process::exit(1);
    }) ;

    println!("搜索词: {}", config.query);
    println!("文件路径: {}", config.file_path);

    if let Err(e) =  minigrep::run(config) {
        eprintln!("这是什么错误：{e}")
    }

}
