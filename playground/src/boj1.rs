use std::io::{stdin, Read};

// This function implements Bakjoon Online Judge problem #11718.
pub fn read_console_then_echo() {
    let mut buf : String = String::from("");
    match stdin().read_to_string(&mut buf) {
        Ok(thing) if thing > 0 => println!("{}", buf),
        Ok(_) => println!("read_console_then_echo: cannot read console!"),
        Err(_) => println!("read_console_then_echo: cannot read console!"),
    }
}

