use minigrep::Params;
use std::env;
use std::process;

fn main() {
    let params = Params::build(env::args()).unwrap_or_else(|err| {
        eprintln!("参数解析失败：{}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(params) {
        eprintln!("运行失败：{}", e);
        process::exit(1);
    }
}
