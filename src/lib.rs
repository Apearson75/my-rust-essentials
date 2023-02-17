use std::io;

pub fn print_each<T: std::fmt::Display>(array: &[T]) {
    for i in array {
        println!("{i}");
    }
}

pub fn input(text: &str) -> String {
    println!("{text}");

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_line(&mut buffer) {
        println!("{e}");
    }


    return buffer;
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn exit() {
    input("\nPress Enter To Exit");
    clear_screen();
}


pub fn parse_int(s: &String) -> i32 {
    if s.ends_with("\n") {
        return s.replace("\n", "").parse::<i32>().unwrap();
    }
    return s.parse::<i32>().unwrap();
}

