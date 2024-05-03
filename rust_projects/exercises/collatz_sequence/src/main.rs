/// Determine the length of the collatz sequence beginning at `n`.

fn collatz_length(mut n: i32) -> u32 {
    let mut len = 0;
    while n != 1 {
        if n%2 != 0 {
            n = 3*n + 1;
            len += 1;
        }
        else {
            n = n/2;
            len += 1;
        }
    }
    len 
  }
  
  fn main() {
    println!("The length is: {}", collatz_length(3));
  }