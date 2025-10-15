# Scalar Types
Single values, integers, floating-point numbers, Booleans, and characters
## Integers
- `i` for signed (two's complement)
- `u` for unsigned
- `8`-`128`
- `size` (architecture)
### Integer Literals
`_` can be used as a visual separator, but not required
```rs
98_222 // decimal 98,222
0xff // hex 255
0o77 // octal 63
0b1111_0000 // binary
b'A' // byte (u8 only)
```
overflow of any type is considered an error
## Floating-Point Types
`f32` and `f64`\
Default is `f64`
```rs
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```
## Booleans
Classic `bool`, `true`, `false`, etc.
## Characters
Classic `char`.
# Compound Types
Tuples and arrays
## Tuples
Classic tuple, type annotated as follows:
```rs
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
## Arrays
Classic array
```rs
fn main() {
    let a = [1, 2, 3, 4, 5];
}

let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5];

let a = [3; 5];
```
