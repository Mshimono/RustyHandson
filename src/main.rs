fn main() {
    println!("1~100までの数字でFizzBuzzする");
    // answer1();
    //answer2();
    answer3();
}

#[allow(dead_code)]
fn answer1() {
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
fn answer2() {
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
fn answer3() {
    for count in 1..=100 {
        match count {
            x if x % 15 == 0 => println!("{x} FizzBuzz"),
            x if x % 3 == 0 => println!("{x} Fizz"),
            x if count % 5 == 0 => println!("{x} Buzz"),
            _ => println!("{count}"),
        }
    }
}
