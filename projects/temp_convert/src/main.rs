fn main() {
    let deg_c: i32 = 67;
    let deg_f: i32 = 67;

    // C to F
    let result_f: f64 = (deg_c as f64 * (9.0/5.0)) + 32.0;

    // F to C
    let result_c: f64 = (deg_f as f64 - 32.0) * (5.0/9.0);
    // Note division of integers gives integer result i.e. 5/9 = 0

    println!("{deg_c}C -> {result_f:.2}F");
    println!("{deg_f}F -> {result_c:.2}C");
}
