use std::{
    io::{self, Write},
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    match args.next().unwrap().as_str() {
        "invalid utf-8 stdout" => io::stdout().write_all(&[0x80]).unwrap(),
        "exit code 42" => std::process::exit(42),
        "stream chunk then wait for file" => {
            println!("foo");
            io::stdout().flush().unwrap();
            let file = PathBuf::from("./file");
            while !file.exists() {
                sleep(Duration::from_secs_f32(0.1));
            }
        }
        "output foo and exit with 42" => {
            println!("foo");
            std::process::exit(42)
        }
        arg => panic!("stir_test_helper: invalid arg: {}", arg),
    }
}