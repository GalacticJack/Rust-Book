const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("The value of our const is: {THREE_HOURS_IN_SECONDS}");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    shadowing();
    new_scope_expression();
    let x = implicit_return();
    println!("IMPLICIT_RETURN - The value of x is {x}");
    if_in_let(true);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("SHADOWING - The value of x in the inner scope is: {x}");
    }
    println!("SHADOWING - The value of x is: {x}");
}

fn new_scope_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("NEW_SCOPE_EXPRESSION - The value of y is: {y}")
}

fn implicit_return() -> i32 {
    5
}

fn if_in_let(condition: bool) {
    let number = if condition { 5 } else { 6 };
    println!("IF_IN_LET - The value of number is: {number}");
}
