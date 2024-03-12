fn main() {
    const EXITCODE_ERROR: i32 = 1;
    println!("Exit Code => {}", EXITCODE_ERROR);
    ::std::process::exit(EXITCODE_ERROR);
}

//cargo run --example main_exitcode_1 ; echo $?
