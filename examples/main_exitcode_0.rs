fn main() {
    const EXITCODE_SUSSESFUL: i32 = 0;
    println!("Exit Code => {}", EXITCODE_SUSSESFUL);
    ::std::process::exit(EXITCODE_SUSSESFUL);
    // ::std::process::exit(0);
}

//cargo run --example main_exitcode_0 ; echo $?
