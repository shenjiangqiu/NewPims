fn main() {
    println!("Hello, world!");
    new_pims::cpp_main();
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_fibonacci_sequence() {
        let expected_results = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        let mut actual_results = Vec::new();

        for i in 0..10 {
            actual_results.push(fibonacci(i));
        }

        assert_eq!(expected_results, actual_results);
    }

    fn fibonacci(n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let mut fib: Vec<u32> = vec![0; (n + 1) as usize];
        fib[1] = 1;
        for i in 2..=n {
            fib[i as usize] = fib[(i - 1) as usize] + fib[(i - 2) as usize];
        }
        fib[n as usize]
    }
}
