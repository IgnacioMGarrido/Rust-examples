use std::io;

fn fib(num : u32) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 1;

    if num == 0 {
        return a
    }

    for _n in 1..num {
        let c = a + b;
        a = b;
        b = c;
    }

    return b
}

fn fib_rec(num: u32) -> u32 {
    if num <= 1 {
       return num
    }

    return fib_rec(num - 1) + fib_rec(num - 2)
}

fn main() {
    println!("Enter fibonacci nth number: ");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let res = fib(num);

    println!("The nth number {num} in the fibonacci sequence is: {res}");

    let res = fib_rec(num);

    println!("Recursive - The nth number {num} in the fibonacci sequence is: {res}");
}

