use std::error::Error;
use std::fs;
use std::env;
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let _results = if config.ignore_case {
        for QueryResult { line, line_num } in search_case_insensitive(&config.query, &contents) {
            println!("结果行 {line_num}：{line}");
        }
    } else {
        for QueryResult { line, line_num } in search(&config.query, &contents) {
            println!("结果行 {line_num}：{line}");
        }
    };

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数量不足");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// TDD 测试驱动开发

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe,fase,productive.
Pick three.
Duct tape
        ";
        let result = search(query, contents);
        assert_eq!(result[0].line, "safe,fase,productive.");
        assert_eq!(result[0].line_num, 2);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe,fase,productive.
Pick three.
Trust me
        ";
        let result = search_case_insensitive(query, contents);
        assert_eq!(result[0].line, "Rust:");
        assert_eq!(result[1].line, "Trust me");
        assert_eq!(result[0].line_num, 1);
        assert_eq!(result[1].line_num, 4);
    }
}

pub struct QueryResult<'a> {
    pub line: &'a str,
    pub line_num: usize,
}

// 搜索
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<QueryResult<'a>> {
    // let mut results = Vec::new();
    // for (idx, line) in contents.lines().enumerate() {
    //     if line.contains(query) {
    //         results.push(QueryResult {
    //             line,
    //             line_num: idx + 1,
    //         });
    //     }
    // }
    // results

    // contents.lines()
    //         .enumerate()
    //         .filter_map(|(i,line)|{
    //             if line.contains(query){
    //                 Some(QueryResult{
    //                     line,
    //                     line_num:i+1
    //                 })
    //             }else{
    //                 None
    //             }
    //         }).collect()
    contents
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            line.contains(&query).then(|| QueryResult {
                line,
                line_num: i + 1,
            })
        })
        .collect()
}

// 无视大小写的搜索
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<QueryResult<'a>> {
    let query = query.to_lowercase();
    contents
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            line.to_lowercase().contains(&query).then(|| QueryResult {
                line,
                line_num: idx + 1,
            })
        })
        .collect()
}
