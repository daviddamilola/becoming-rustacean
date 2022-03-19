
fn main() {
    // let number = 3;
    // check_number(number);
    // loop_example();
    // while_loop();
    // for_loop()
    // loop_return()
    twelve_days_of_christmas()
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

fn while_loop() {
    let a  = [10, 20, 30, 40, 50];
    let mut index = 0;
    while(index < 5){
        println!("the value is {}",a[index]);
        index = index + 1;
    }
}


fn for_loop() {
    let a  = [10, 20, 30, 40, 50];
    for elem in a {
        println!("the value is {}",elem);
    }
}


fn loop_return() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {}", result)
}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    (celcius * 1.8) + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn twelve_days_of_christmas () {
    let gifts = vec!["a partridge in a pear tree", "two turtle doves", "three french hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];
    let days = vec!["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    for (i, day) in days.iter().enumerate()  {
        println!("On the {} day of Christmas my true love sent to me", day);
        let mut current_gift = &gifts[0..i+1];
        let y = current_gift.iter().rev();
        for elem in y {
            println!("{}", elem);
        }
        println!("");
    }
}