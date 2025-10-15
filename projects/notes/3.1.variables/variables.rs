// Variable mutability
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // error
    println!("The value of x is: {x}");
}

// need to make x mutable with 'let mut x = 5;'
