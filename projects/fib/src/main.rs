fn fibonacci(n: u64) -> u64 {
    let mut n1: u64 = 0;  // F_0
    let mut n2: u64 = 1;  // F_1

    for _ in 0..n {
        print!("{n1} ");

        let n3: u64 = n2 + n1;  // Temp variable :(
        n1 = n2;
        n2 = n3;
    }
    println!();
    n1
}

fn fibonacci_tuple(n: u64) -> u64 {
    let (mut n1, mut n2) = (0, 1);

    for _ in 0..n {
        print!("{n1} ");
        (n1, n2) = (n2, n1 + n2);  // No temp var :)  // Calls mem::swap
    }
    println!();
    n1
}

fn fibonacci_mem(n: u64) -> u64 {
    use std::mem;
    let mut n1 = 0;
    let mut n2 = 1;

    for _ in 0..n {
        print!("{n1} ");
        let old_n1: u64 = mem::replace(&mut n1, n2);
        n2 = old_n1 + n2;
    }
    println!();
    n1
}

// SLOW
fn fibonacci_recurse(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    fibonacci_recurse(n-1) + fibonacci_recurse(n-2)
}

fn main() {
    // Argument n represents first 'n' elements of the sequence i.e. 0 = no elements
    println!("\nfibonacci:");
    fibonacci(0);
    fibonacci(1);
    fibonacci(10);

    println!("\nfibonacci_tuple:");
    fibonacci_tuple(0);
    fibonacci_tuple(1);
    fibonacci_tuple(10);

    println!("\nfibonacci_mem:");
    fibonacci_mem(0);
    fibonacci_mem(1);
    fibonacci_mem(10);

    // For recurse n represents fib sequence at pos n
    println!("\nfibonacci_recurse:");
    println!("{}", fibonacci_recurse(0));
    println!("{}", fibonacci_recurse(1));
    println!("{}", fibonacci_recurse(9));

    // use std::time::Instant;
    // println!("\n Time Test:");
    // let start = Instant::now();
    // fibonacci(90);
    // let duration = start.elapsed();
    // println!("fibonacci took: {duration:?}");

    // let start = Instant::now();
    // fibonacci_tuple(90);
    // let duration = start.elapsed();
    // println!("fibonacci_tuple took: {duration:?}");

    // let start = Instant::now();
    // fibonacci_mem(90);
    // let duration = start.elapsed();
    // println!("fibonacci_mem took: {duration:?}");

    // Too slow lol, this recursion is O(2^n)
    // let start = Instant::now();
    // println!("{}", fibonacci_recurse(90));
    // let duration = start.elapsed();
    // println!("fibonacci_recurse took: {duration:?}");
}
