use std::time::SystemTime;

fn fib(k: u32) -> u128 {
    match k {
        0 => 0,
        1 => 1,
        _ => fib(k - 1) + fib(k - 2)
    }
}

fn main() {
    for k in 0..=50 {
        let before = SystemTime::now();
        let result = fib(k);
        let after = SystemTime::now();
        let runtime = after.duration_since(before)
            .expect("Clock may have gone backwards");
        println!("F({}) = {}, runtime: {:?} ", k, result, runtime);
    }
}
