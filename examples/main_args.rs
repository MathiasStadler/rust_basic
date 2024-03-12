use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

// cargo run --example main_args arg1 arg2
