fib(n: u32) -> u32 {
    let mut var = vec![0,1];
    if n < 2 {
        n
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}