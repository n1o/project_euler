
fn problem1() -> i32 {
    (1..1000).filter(|&x:i32| x % 3 == 0 || x % 5 == 0).sum();
}



