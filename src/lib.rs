use std::{env, error::Error, fs};

// 参数
pub struct Params {
    query: String,
    file_path: String,
    ignore_case: bool,
}
// 解析参数
impl Params {
    // 参数构建
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Params, &'static str> {
        // 跳过第一个参数
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少查询参数"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少文件路径参数"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Params {
            query,
            file_path,
            ignore_case,
        })
    }
}
// 运行
pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(params.file_path)?;
    let results = if params.ignore_case {
        search_case_insensitive(&params.query, &contents)
    } else {
        search(&params.query, &contents)
    };
    for line in results {
        println!("{}", line)
    }
    Ok(())
}
// 搜索
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
// 搜索（忽略大小写）
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn two_result() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
