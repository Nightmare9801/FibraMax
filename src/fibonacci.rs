use std::{str::FromStr, time::{Duration, Instant}};

use num_bigint::BigUint;

pub fn nth_fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut previous_prev = 0;
    let mut prev = 1;
    for _i in 0..n - 1 {
        let temp = prev;
        prev += previous_prev;
        previous_prev = temp;
    }
    prev
}

pub(crate) fn fibonacci_rebounded() -> BigUint {
    let mut previous_prev: BigUint = BigUint::ZERO;
    let mut prev: BigUint = BigUint::from_str("1").unwrap();
    let start_time: Instant = Instant::now();
    let one_second: Duration = Duration::from_secs(1);
    let mut i: i32 = 0;
    loop {
        print!("Rebounded: {} \r", i);
        let elapsed = start_time.elapsed();
        if elapsed >= one_second {
            break;
        }

        let temp: BigUint = prev.clone();
        prev += previous_prev;
        previous_prev = temp;
        i += 1;
    }
    println!();
    prev
}
 
#[test]
pub fn fibonacci_tester() {
    let fibonacci: Vec<u128> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181];
    for i in 0..fibonacci.len() { 
        assert!(fibonacci[i] == nth_fibonacci(i as u128));
    }
}
#[test]
pub fn validate_nth() {
    fn is_perfect_square(x: u128) -> bool {
        let s = (x as f64).sqrt() as u128;
        s * s == x
    }
    
    fn validate(n: u128) -> bool {
        is_perfect_square(5 * n * n + 4) || is_perfect_square(5 * n * n - 4) 
    }

    for i in 0..10 {
        assert!(validate(nth_fibonacci(i as u128)));
    }
}