use std::io::{stdin, stdout, Read, Write};

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


// This function implements Bakjoon Online Judge problem #2741
// https://www.acmicpc.net/problem/2741
pub fn print_n() {
    let mut n_string = String::new();
    let mut output_string = String::new();
    stdin().read_line(&mut n_string)
            .expect("Cannot read from stdin!");
    
    let trimmed = n_string.trim();
    match trimmed.parse::<i32>() {
        Ok(n) => for x in 1..(n + 1) {
            // println!("{}", x);
            output_string.push_str(format!("{}\n", x).as_str());
        },
        Err(_) => println!("Failed to parse int32!"),
    }
    println!("{}", output_string);
}

// This function implements Bakjoon Online Judge problem #2742
// https://www.acmicpc.net/problem/2742
pub fn print_n_reverse() {
    let mut n_string = String::new();
    let mut output_string = String::new();
    stdin().read_line(&mut n_string)
            .expect("Cannot read from stdin!");
    
    let trimmed = n_string.trim();
    match trimmed.parse::<i32>() {
        // .rev() does reverse iteration. Neat!
        Ok(n) => for x in (1..(n + 1)).rev() {
            // println!("{}", x);
            output_string.push_str(format!("{}\n", x).as_str());
        },
        Err(_) => println!("Failed to parse int32!"),
    }
    println!("{}", output_string);
}

// This function implements Bakjoon Online Judge problem #2739
// https://www.acmicpc.net/problem/2739
pub fn print_n_gugudan() {
    let mut n_string = String::new();
    let mut output_string = String::new();
    stdin().read_line(&mut n_string)
            .expect("Cannot read from stdin!");
    
    let trimmed = n_string.trim();
    match trimmed.parse::<i32>() {
        // .rev() does reverse iteration. Neat!
        Ok(n) => for x in 1..10 {
            // println!("{}", x);
            output_string.push_str(format!("{} * {} = {}\n", n, x, n * x).as_str());
        },
        Err(_) => println!("Failed to parse int32!"),
    }
    println!("{}", output_string);

}

pub fn calculate_sugar_bags() {
    let sugar_bags: Vec<i32> = vec![5, 3];



}

// @TODO: complete this
fn calculate_sugar_bags_basis(kilogram: i32, bag: i32) -> i32 {
    match kilogram {
        n if n < bag => -1,
        _ => -2,
    }


}