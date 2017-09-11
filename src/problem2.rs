fn problem2(){

    let mut i = 1;
    let mut fib = fibonacci(1, 2, i);
    let mut acc = 0;

    while  fib < 4_000_000 {
        if fib % 2 == 0 {
            acc += fib;

        }

        i += 1;
        fib = fibonacci(1, 2, i)

    }

    println!("{}", fibonacci(1, 2, 10));

    fn fibonacci(first: i32, second: i32, n: i32) -> i32 {
        if n > 1 {
            fibonacci(second, first + second, n - 1)
        } else {
            first
        }
    }
}