
fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);   

    // We can assign a new value to x cause we marked it as mutable
    x = 7;

    println!("The value of x is {}", x);   

    // Const variables are always inmutable
    // They are valid through the entire program life cycle
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    // We can eclare a new variable with the same name as a previous
    // declared variable

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }


    println!("The value of y is {}", y);   
}


