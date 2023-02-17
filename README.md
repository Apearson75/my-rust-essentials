# My Essentials for Rust

```rust
use my_essentials::*;

fn main() {
    clear_screen();

    println!("Welcome to the survey");

    let array = ["1. Bad", "2. Ok", "3. Good", "4. Great", "5. Excelent"];
    print_each(&array);
    
    let inp = input("How Would you rate your experience?");
    let res = parse_int(&inp);

    if res < 1 || res > 5 {
        println!("Invalid Answer");
        exit();
        return;
    }

    println!("Thank you for the feedback.");

    exit();
}
```