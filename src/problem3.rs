//O(n^2)

fn problem3() {

    fn largest_prime(n: &i64) -> Option<i64> {
        (2 .. ((*n as f64).sqrt().ceil() as i64))
            .filter(|i| *n % *i == 0 && is_prime(i)).last()

    }

    fn is_prime(n: &i64) -> bool {
        if *n == 0{
            false
        }else if *n == 1 {
            true
        } else {
            (2..((*n as f64).sqrt().ceil() as i64))
                .find(|i| *n % *i == 0)
                .is_none()
        }
    }

    match largest_prime(&600851475143) {
        Some(i) => println!("Larget prime factor is {}",i),
        None => println!("No prime factors")
    }
}
