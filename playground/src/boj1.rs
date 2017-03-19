use std::io::{stdin, Read};

// This function implements Bakjoon Online Judge problem #11718.
// https://www.acmicpc.net/problem/11718
pub fn read_console_then_echo() {
    let mut buf : String = String::from("");
    match stdin().read_to_string(&mut buf) {
        Ok(thing) if thing > 0 => println!("{}", buf),
        Ok(_) => println!("read_console_then_echo: cannot read console!"),
        Err(_) => println!("read_console_then_echo: cannot read console!"),
    }
}

// This function implements Bakjoon Online Judge problem #10718
// https://www.acmicpc.net/problem/10718 
#[allow(unused_variables)]
pub fn print_army() {

    for x in 0..2 {
        println!("강한친구 대한육군");
    }
}
