fn main() {
    let number = 3;
    check_number(number);
    loop_example();
}


fn check_number(number: i32) {
    if number < 5 {
        return println!("condition was true");
    } 
    println!("condition was false")
}

fn loop_example() {
    loop {
        println!("again!");
        break;
    }
}

