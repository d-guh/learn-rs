# Functions
Classic functions `fn`
```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```
## Parameters
Usual params/args stuff
```rs
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```
Function signatures also exist\
Multiple parameters:
```rs
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```
## Statements and Expressions
Statements are instructions that perform some action and do not return, expressions return.
```rs
fn main() {
    let x = (let y = 6); // DOES NOT COMPILE
}
```
Example expression:
```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```
## Functions with Return Values
Functions that return must be type annotated with `->`\
Functions automatically return the last expression, but can also use the `return` keyword.
```rs
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```
Add one example:
```rs
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```
Note the lack of semicolon, if there's a semicolon it becomes a statement not an expression, and will not compile.
