use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("出现了错误：{err}");
        process::exit(1);
    }) ;

    println!("搜索词: {}", config.query);
    println!("文件路径: {}", config.file_path);

    if let Err(e) =  minigrep::run(config) {
        eprintln!("这是什么错误：{e}")
    }


}
