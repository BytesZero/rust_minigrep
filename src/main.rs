use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let params = Params::new(&args).unwrap_or_else(|err| {
        println!("参数解析失败：{}", err);
        process::exit(1);
    });
    println!(
        "Searching for '{}' in file:{}",
        params.query, params.file_path
    );
    let contents = fs::read_to_string(params.file_path).expect("没有读取到文件，请检查文件路径");
    println!("With text:\n{}", contents);
}
// 参数
struct Params {
    query: String,
    file_path: String,
}
// 解析参数
impl Params {
    fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("参数不足");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Params { query, file_path })
    }
}
