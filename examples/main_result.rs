use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    const EXITCODE_SUSSESFUL: i32 = 0;
    println!("Exit Code => {}", EXITCODE_SUSSESFUL);

    ::std::process::exit(EXITCODE_SUSSESFUL);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1.add(2), 3);
    }
}

// cargo run --example main_result ; echo $?

// cargo test --examples main_result
