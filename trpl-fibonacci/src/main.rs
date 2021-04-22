use std::env;

fn fib(n: u8) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }
    let mut p1 = 1;
    let mut p2 = 1;
    for _ in 3..n {
        let n1 = p2;
        let n2 = p1 + p2;
        p1 = n1;
        p2 = n2;
    }
    p1 + p2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
    }
}

fn main() {
    let n = env::args()
        .nth(1)
        .expect("Usage: trpl-fibonacci <N>")
        .parse::<u8>()
        .expect("N is too large");
    println!("{}", fib(n));
}
