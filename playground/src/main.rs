use std::time::{Duration, Instant};

fn main() {

    let a = vec![1,2,3];
    
    let mut start = Instant::now();
    let mut b = vec![0; a.len()];
    for i in 0..a.len() {
        b[i] = a[i]*a[i];
    }
    let mut duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    start = Instant::now();
    let mut c = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        c.push(a[i]*a[i]);
    }
    duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    start = Instant::now();
    let d: Vec<i32> = a.iter().map(|x| x*x).collect();
    duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("c: {:?}", d);
}


