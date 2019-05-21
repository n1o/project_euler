mod problem4;
use std::cmp;

fn largest_palindrome(f: i32,s: i32): i64 {

    let max = cmp::max(f,s);
    for (x= max; x > 0; x--) {
        println!("{}",x);
    }

    f + s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_palindrome_test() {
        assert_eq!(largest_palindrome(10, 20), 1)
    }
}