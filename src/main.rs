fn main() {
    println!("Hello, world!");

    println!("1~100までの数字でFizzBuzzする");
    // example1();
    //example2();
    example3();
}

#[allow(dead_code)]
fn example1() {
    let mut count = 0;

    while count <= 100 {
        if count % 15 == 0 {
            println!("{count} FizzBuzz");
        } else if count % 3 == 0 {
            println!("{count} Fizz");
        } else if count % 5 == 0 {
            println!("{count} Buzz");
        } else {
            println!("{count}");
        }
        count += 1;
    }
}

#[allow(dead_code)]
fn example2() {
    for count in 1..=100 {
        if count % 15 == 0 {
            println!("{count} FizzBuzz");
        } else if count % 3 == 0 {
            println!("{count} Fizz");
        } else if count % 5 == 0 {
            println!("{count} Buzz");
        } else {
            println!("{count}");
        }
    }
}

#[allow(dead_code)]
fn example3() {
    for count in 1..=100 {
        match count {
            x if x % 15 == 0 => println!("{x} FizzBuzz"),
            x if x % 3 == 0 => println!("{x} Fizz"),
            x if count % 5 == 0 => println!("{x} Buzz"),
            _ => println!("{count}"),
        }
    }
}
