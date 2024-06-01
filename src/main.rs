use std::time::SystemTime;

struct Config {
    delay: usize,
    format: String,
    file_path: String,
    extra_lines: usize,
}

fn main() {
    //  [2023-02-14 13:42:48] local.INFO: Incoming webhook: 2
    //      pork

    let config = Config {
        file_path: "".to_owned(),
        delay: 2000,
        format: "".to_owned(),
        extra_lines: 0,
    };

    let now = SystemTime::now();
let     dt: 

    println!("Hello, world!");
}
