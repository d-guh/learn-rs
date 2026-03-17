use std::{collections::HashMap, vec};

fn median(integers: &mut Vec<i32>) -> f64 {
    let length: usize = integers.len();
    let med: f64;

    integers.sort();

    if length % 2 != 0 {
        med = integers[length/2] as f64;
    } else {
        med = (integers[length/2] + integers[(length/2)-1]) as f64 / 2.0;
    }

    med
}

fn mode(integers: Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, i32>  = HashMap::new();

    for int in integers {
        *counts.entry(int).or_insert(0) += 1;
    }

    let mode: Option<i32> = counts.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val);

    match mode {
        Some(m) => m,
        None => 0
    }
}

fn main() {
    let mut int_big: Vec<i32> = vec![67, 32, 128, -23, 109, 214, 73, 999, -67, -134, 2841, 67];
    let mut int_odd: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut int_even: Vec<i32> = vec![1, 2, 3, 4];
    let mut repeats: Vec<i32> = vec![1, 2, 3, 1, 1, 67, 4, 2, 3, 1];

    println!("int_big Median: {}", median(&mut int_big));
    println!("int_odd Median: {}", median(&mut int_odd));
    println!("int_even Median: {}", median(&mut int_even));
    println!("repeats Median: {}", median(&mut repeats));
    println!();
    println!("int_big Mode: {}", mode(int_big));
    println!("int_odd Mode: {}", mode(int_odd));
    println!("int_even Mode: {}", mode(int_even));
    println!("repeats Mode: {}", mode(repeats));
}
